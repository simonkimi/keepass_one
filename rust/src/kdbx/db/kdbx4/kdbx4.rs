use crate::crypto::hash;
use crate::kdbx::config::MemoryProtectConfig;
use crate::kdbx::db::kdbx4::config::Kdbx4Config;
use crate::kdbx::db::kdbx4::errors::Kdbx4Error;
use crate::kdbx::db::kdbx4::header::Kdbx4Header;
use crate::kdbx::db::kdbx4::hmac::{
    calc_kdbx4_header_hmac_key, calc_kdbx4_hmac_key, write_hmac_block,
};
use crate::kdbx::db::kdbx4::inner_header::Kdbx4InnerHeader;
use crate::kdbx::db::version::KdbxVersion;
use crate::kdbx::xml::database::KeePassDatabase;
use crate::kdbx::xml::errors::KdbxSaveError;
use crate::utils::writer::WritableExt;
use crate::{crypto, kdbx::db::kdbx4::hmac::parse_hmac_block};
use generic_array::{typenum::U32, GenericArray};

pub struct Kdbx4 {
    pub key_hash: GenericArray<u8, U32>,
    pub header: Kdbx4Header,
    pub database: KeePassDatabase,
}

impl Kdbx4 {
    pub fn open(
        data: &[u8],
        key_hash: &GenericArray<u8, U32>,
        config: &MemoryProtectConfig,
    ) -> Result<Kdbx4, Kdbx4Error> {
        match KdbxVersion::parse(data)? {
            KdbxVersion::KDB4(_) => {}
            _ => return Err(Kdbx4Error::UnsupportedVersion),
        }

        let (header, header_size) = Kdbx4Header::try_from(data)?;
        let header_bytes = data.get(..header_size).ok_or(Kdbx4Error::UnexpectedEof)?;
        let header_sha256 = data
            .get(header_size..header_size + 32)
            .ok_or(Kdbx4Error::UnexpectedEof)?;
        let header_hmac = data
            .get(header_size + 32..header_size + 64)
            .ok_or(Kdbx4Error::UnexpectedEof)?;
        let hmac_payload = data
            .get(header_size + 64..)
            .ok_or(Kdbx4Error::UnexpectedEof)?;

        if header_sha256 != crypto::hash::calculate_sha256(header_bytes).as_slice() {
            return Err(Kdbx4Error::HeaderSha256ChecksumMismatch);
        }

        let transformed_key = header
            .config
            .kdf_parameters
            .get_kdf()
            .transform_key(key_hash)?;
        let hmac_key = calc_kdbx4_hmac_key(&header.config.master_salt_seed, &transformed_key);
        let header_hmac_key = calc_kdbx4_header_hmac_key(&hmac_key);

        hash::verify_hmac_multiple(&[&header_bytes], &header_hmac_key, header_hmac)
            .map_err(|_| Kdbx4Error::HeaderHmacChecksumMismatch)?;

        let payload_encrypted =
            parse_hmac_block(hmac_payload, &hmac_key).map_err(Kdbx4Error::ParseHmacBlockError)?;

        let master_key =
            hash::calculate_sha256_multiple(&[&header.config.master_salt_seed, &transformed_key]);

        let payload_decrypted = header
            .config
            .encryption_algorithm
            .get_cipher(&master_key, &header.config.encryption_iv)
            .map_err(Kdbx4Error::DecryptPayloadError)?
            .decrypt(&payload_encrypted)
            .map_err(Kdbx4Error::DecryptPayloadError)?;

        let payload_uncompressed = header
            .config
            .compression_config
            .get_compression()
            .decompress(&payload_decrypted)
            .map_err(Kdbx4Error::DecompressPayloadError)?;

        let (inner_header, inner_header_size) =
            Kdbx4InnerHeader::try_from(&payload_uncompressed[..])?;
        let xml = payload_uncompressed
            .get(inner_header_size..)
            .ok_or(Kdbx4Error::UnexpectedEof)?;

        Ok(Self {
            key_hash: key_hash.clone(),
            header,
            database: KeePassDatabase::try_from(xml, inner_header, config)?,
        })
    }

    pub fn dump_xml(&self) -> Result<Vec<u8>, std::io::Error> {
        self.database.document.dump()
    }

    pub fn save_with_config<W>(
        &self,
        key_hash: &GenericArray<u8, U32>,
        config: Kdbx4Config,
        writer: &mut W,
    ) -> Result<(), KdbxSaveError>
    where
        W: std::io::Write + std::io::Seek,
    {
        let header = self.header.copy_with(config);
        let header_bytes = header.write_to_buffer()?;
        writer.write_all(&header_bytes)?;
        let header_sha256 = crypto::hash::calculate_sha256(header_bytes.as_slice());
        writer.write_all(&header_sha256)?;

        let transformed_key = header
            .config
            .kdf_parameters
            .get_kdf()
            .transform_key(key_hash)?;
        let hmac_key = calc_kdbx4_hmac_key(&header.config.master_salt_seed, &transformed_key);
        let header_hmac_key = calc_kdbx4_header_hmac_key(&hmac_key);
        let header_hmac = hash::calculate_hmac_multiple(&[&header_bytes], &header_hmac_key)?;
        writer.write_all(&header_hmac)?;

        let new_database = self.database.encrypt_database()?;
        let new_database_bytes = new_database.write_to_buffer()?;

        let payload_compressed = header
            .config
            .compression_config
            .get_compression()
            .compress(&new_database_bytes)?;
        let master_key =
            hash::calculate_sha256_multiple(&[&header.config.master_salt_seed, &transformed_key]);

        let payload_encrypted = header
            .config
            .encryption_algorithm
            .get_cipher(&master_key, &header.config.encryption_iv)?
            .encrypt(&payload_compressed)?;

        write_hmac_block(&payload_encrypted, &hmac_key, writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod kdbx4_tests {
    use std::io::Cursor;

    use crate::kdbx::{
        config::MemoryProtectConfig,
        db::{
            kdbx4::{
                config::Kdbx4Config,
                errors::Kdbx4Error,
                header::Kdbx4Header,
                header_entity::{
                    compression::CompressionConfig, encryption_algorithm::EncryptionAlgorithm,
                    kdf_config::KdfConfig,
                },
                inner_header::{Kdbx4InnerEncryption, Kdbx4InnerHeader},
            },
            version::{KDBX_IDENTIFIER, KEEPASS_LATEST_ID},
        },
        keys::KdbxKey,
        xml::database::KeePassDatabase,
    };

    use super::Kdbx4;

    fn test_config() -> MemoryProtectConfig {
        MemoryProtectConfig {
            enable_memory_crypt: false,
            enable_mlock: false,
        }
    }

    fn test_kdbx_config() -> Kdbx4Config {
        Kdbx4Config {
            encryption_algorithm: EncryptionAlgorithm::Aes256,
            compression_config: CompressionConfig::GZip,
            master_salt_seed: [0x42; 32],
            encryption_iv: vec![0x11; 16],
            kdf_parameters: KdfConfig::Aes {
                salt: [0x24; 32],
                rounds: 1,
            },
        }
    }

    fn minimal_xml(password_b64: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<KeePassFile>
  <Meta>
    <Generator>KeePassOne</Generator>
    <SettingsChanged>2020-01-01T00:00:00Z</SettingsChanged>
    <DatabaseName>test</DatabaseName>
    <DatabaseNameChanged>2020-01-01T00:00:00Z</DatabaseNameChanged>
    <DatabaseDescription></DatabaseDescription>
    <DatabaseDescriptionChanged>2020-01-01T00:00:00Z</DatabaseDescriptionChanged>
    <DefaultUserName></DefaultUserName>
    <DefaultUserNameChanged>2020-01-01T00:00:00Z</DefaultUserNameChanged>
    <MaintenanceHistoryDays>365</MaintenanceHistoryDays>
    <Color></Color>
    <MasterKeyChanged>2020-01-01T00:00:00Z</MasterKeyChanged>
    <MasterKeyChangeRec>-1</MasterKeyChangeRec>
    <MasterKeyChangeForce>-1</MasterKeyChangeForce>
    <MemoryProtection>
      <ProtectTitle>False</ProtectTitle>
      <ProtectUserName>False</ProtectUserName>
      <ProtectPassword>True</ProtectPassword>
      <ProtectURL>False</ProtectURL>
      <ProtectNotes>False</ProtectNotes>
    </MemoryProtection>
    <CustomIcons></CustomIcons>
    <RecycleBinEnabled>False</RecycleBinEnabled>
    <RecycleBinChanged>2020-01-01T00:00:00Z</RecycleBinChanged>
    <EntryTemplatesGroupChanged>2020-01-01T00:00:00Z</EntryTemplatesGroupChanged>
    <HistoryMaxItems>10</HistoryMaxItems>
    <HistoryMaxSize>6291456</HistoryMaxSize>
  </Meta>
  <Root>
    <Group>
      <UUID>AQEBAQEBAQEBAQEBAQEBAQ==</UUID>
      <Name>Root</Name>
      <IconID>0</IconID>
      <Times>
        <CreationTime>2020-01-01T00:00:00Z</CreationTime>
        <LastModificationTime>2020-01-01T00:00:00Z</LastModificationTime>
        <LastAccessTime>2020-01-01T00:00:00Z</LastAccessTime>
        <ExpiryTime>2020-01-01T00:00:00Z</ExpiryTime>
        <Expires>False</Expires>
        <UsageCount>0</UsageCount>
        <LocationChanged>2020-01-01T00:00:00Z</LocationChanged>
      </Times>
      <IsExpanded>True</IsExpanded>
      <EnableAutoType>Null</EnableAutoType>
      <EnableSearching>Null</EnableSearching>
      <Entry>
        <UUID>AgICAgICAgICAgICAgICAg==</UUID>
        <IconID>0</IconID>
        <Times>
          <CreationTime>2020-01-01T00:00:00Z</CreationTime>
          <LastModificationTime>2020-01-01T00:00:00Z</LastModificationTime>
          <LastAccessTime>2020-01-01T00:00:00Z</LastAccessTime>
          <ExpiryTime>2020-01-01T00:00:00Z</ExpiryTime>
          <Expires>False</Expires>
          <UsageCount>0</UsageCount>
          <LocationChanged>2020-01-01T00:00:00Z</LocationChanged>
        </Times>
        <String>
          <Key>Password</Key>
          <Value Protected="True">{password_b64}</Value>
        </String>
        <AutoType>
          <Enabled>False</Enabled>
          <DataTransferObfuscation>0</DataTransferObfuscation>
        </AutoType>
      </Entry>
    </Group>
  </Root>
</KeePassFile>"#
        )
    }

    #[test]
    fn test_open_rejects_truncated_file() {
        let mut key = KdbxKey::new();
        key.add_master_key("test");
        let key_hash = key.calc_key_hash().unwrap();
        let err = match Kdbx4::open(&[0u8; 8], &key_hash, &test_config()) {
            Err(e) => e,
            Ok(_) => panic!("expected truncated file to fail"),
        };
        assert!(matches!(err, Kdbx4Error::Version(_)));
    }

    #[test]
    fn test_open_rejects_kdbx3() {
        let mut data = Vec::new();
        data.extend_from_slice(&KDBX_IDENTIFIER);
        data.extend_from_slice(&KEEPASS_LATEST_ID.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&3u16.to_le_bytes());
        data.extend_from_slice(&[0u8; 64]);

        let mut key = KdbxKey::new();
        key.add_master_key("test");
        let key_hash = key.calc_key_hash().unwrap();
        let err = match Kdbx4::open(&data, &key_hash, &test_config()) {
            Err(e) => e,
            Ok(_) => panic!("expected kdbx3 to be rejected"),
        };
        assert!(matches!(err, Kdbx4Error::UnsupportedVersion));
    }

    #[test]
    fn test_protected_binary_then_password_roundtrip() -> anyhow::Result<()> {
        use crate::kdbx::db::kdbx4::header_entity::binary_content::BinaryContent;
        use crate::kdbx::xml::entities::Value;
        use base64::Engine;

        let inner = Kdbx4InnerEncryption::new()?;
        let mut cipher = inner.get_stream_cipher()?;
        let attachment = b"attachment-bytes";
        let password = b"hunter2";
        let attachment_ct = cipher.encrypt(attachment)?;
        let password_ct = cipher.encrypt(password)?;
        let password_b64 =
            base64::engine::general_purpose::STANDARD.encode(&password_ct);

        let xml = minimal_xml(&password_b64);
        let inner_header = Kdbx4InnerHeader {
            encryption: inner,
            binary_content: vec![BinaryContent::new(
                BinaryContent::PROTECTED_FLAG,
                attachment_ct,
            )],
        };

        let database = KeePassDatabase::try_from(xml.as_bytes(), inner_header, &test_config())?;
        let password_field = &database.document.root.group.entry[0].string[0].value;
        assert_eq!(database.get_value_string(password_field)?, "hunter2");
        assert_eq!(
            database.inner_header.binary_content[0].offset,
            Some(0)
        );
        if let Value::Protected { offset, .. } = password_field {
            assert_eq!(*offset, Some(attachment.len()));
        } else {
            panic!("expected protected password");
        }

        let mut key = KdbxKey::new();
        key.add_master_key("roundtrip");
        let key_hash = key.calc_key_hash()?;

        let kdbx = Kdbx4 {
            key_hash: key_hash.clone(),
            header: Kdbx4Header::from_config(test_kdbx_config()),
            database,
        };

        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        kdbx.save_with_config(&key_hash, test_kdbx_config(), &mut writer)?;

        let opened = Kdbx4::open(&buffer, &key_hash, &test_config())?;
        let dumped = opened.dump_xml()?;
        assert!(dumped.starts_with(b"<?xml"));
        let opened_password = &opened.database.document.root.group.entry[0].string[0].value;
        assert_eq!(opened.database.get_value_string(opened_password)?, "hunter2");

        let mut cipher = opened.database.inner_header.encryption.get_stream_cipher()?;
        let binary = &opened.database.inner_header.binary_content[0];
        let plain = cipher.decrypt_at_offset(binary.offset.unwrap(), &binary.content)?;
        assert_eq!(plain, attachment);

        Ok(())
    }
}
