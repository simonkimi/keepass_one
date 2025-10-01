use crate::crypto::hash;
use crate::kdbx::db::kdbx4::config::Kdbx4Config;
use crate::kdbx::db::kdbx4::errors::Kdbx4Error;
use crate::kdbx::db::kdbx4::header::Kdbx4Header;
use crate::kdbx::db::kdbx4::hmac::write_hmac_block;
use crate::kdbx::db::kdbx4::inner_header::Kdbx4InnerHeader;
use crate::kdbx::xml::database::KeePassDatabase;
use crate::utils::writer::WritableExt;
use crate::{crypto, kdbx::db::kdbx4::hmac::parse_hmac_block};
use generic_array::{typenum::U32, GenericArray};
use hex_literal::hex;

pub struct Kdbx4 {
    pub key_hash: GenericArray<u8, U32>,
    pub header: Kdbx4Header,
    pub database: KeePassDatabase,
}

impl Kdbx4 {
    pub fn open(data: &[u8], key_hash: &GenericArray<u8, U32>) -> Result<Kdbx4, Kdbx4Error> {
        let (header, header_size) = Kdbx4Header::try_from(data)?;
        let header_bytes = &data[..header_size];
        let header_sha256 = &data[header_size..header_size + 32];
        let header_hmac = &data[header_size + 32..header_size + 64];

        if header_sha256 != crypto::hash::calculate_sha256(header_bytes).as_slice() {
            return Err(Kdbx4Error::HeaderSha256ChecksumMismatch);
        }

        let transformed_key = header
            .config
            .kdf_parameters
            .get_kdf()
            .transform_key(key_hash)?;
        let hmac_key = hash::calculate_sha512_multiple(&[
            &header.config.master_salt_seed,
            &transformed_key,
            &hex!("01"),
        ]);

        let header_hmac_key =
            hash::calculate_sha512_multiple(&[&hex!("FFFFFFFFFFFFFFFF"), &hmac_key]);

        if header_hmac
            != hash::calculate_hmac_multiple(&[&header_bytes], &header_hmac_key)
                .map_err(Kdbx4Error::CalculateHmacError)?
                .as_slice()
        {
            return Err(Kdbx4Error::HeaderHmacChecksumMismatch);
        }

        let payload_encrypted = parse_hmac_block(&data[header_size + 64..], &hmac_key)
            .map_err(Kdbx4Error::ParseHmacBlockError)?;

        let master_key =
            hash::calculate_sha256_multiple(&[&header.config.master_salt_seed, &transformed_key]);

        let payload_decrypted = header
            .config
            .encryption_algorithm
            .get_cipher(&master_key, &header.config.encryption_iv)
            .decrypt(&payload_encrypted)
            .map_err(Kdbx4Error::DecryptPayloadError)?;

        let payload_uncompressed = header
            .config
            .compression_config
            .get_compression()
            .decompress(&payload_decrypted)
            .map_err(Kdbx4Error::DecompressPayloadError)?;

        let (inner_header, header_size) = Kdbx4InnerHeader::try_from(&payload_uncompressed[..])?;
        let xml = &payload_uncompressed[header_size..];
        let now = chrono::Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("demo_{}.xml", timestamp);
        std::fs::write(&filename, xml)?;

        Ok(Self {
            key_hash: key_hash.clone(),
            header,
            database: KeePassDatabase::try_from(xml, inner_header)?,
        })
    }

    // 以新的配置保存keepass数据库
    pub fn save_with_config<W>(
        &self,
        key_hash: &GenericArray<u8, U32>,
        config: Kdbx4Config,
        writer: &mut W,
    ) -> anyhow::Result<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        // 外层header
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
        let hmac_key = hash::calculate_sha512_multiple(&[
            &header.config.master_salt_seed,
            &transformed_key,
            &hex!("01"),
        ]);

        let header_hmac_key =
            hash::calculate_sha512_multiple(&[&hex!("FFFFFFFFFFFFFFFF"), &hmac_key]);
        let header_hmac = hash::calculate_hmac_multiple(&[&header_bytes], &header_hmac_key)?;
        writer.write_all(&header_hmac)?;

        // 计算内层数据
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
            .get_cipher(&master_key, &header.config.encryption_iv)
            .encrypt(&payload_compressed)?;

        write_hmac_block(&payload_encrypted, &hmac_key, writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod kdbx4_tests {
    use std::io::Cursor;

    use crate::kdbx::{
        db::kdbx4::kdbx4::Kdbx4,
        keys::KdbxKey,
        xml::{database::KeePassDatabase, entities},
    };

    #[test]
    fn test_kdbx4_open() -> anyhow::Result<()> {
        let file_path = r#"/Users/simonxu/Project/test.kdbx"#;
        let data = std::fs::read(file_path)?;

        let mut key = KdbxKey::new();
        key.add_master_key("YAZ5pfd4bqz1yhk.tmv");
        let key_hash = key.calc_key_hash()?;
        let kdbx = Kdbx4::open(&data, &key_hash)?;

        let new_config = kdbx.header.config.rekey();

        let mut new_key = KdbxKey::new();
        new_key.add_master_key("test123456");
        let new_key_hash = new_key.calc_key_hash()?;
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        kdbx.save_with_config(&new_key_hash, new_config, &mut writer)?;

        let new_kdbx = Kdbx4::open(&buffer, &new_key_hash)?;
        // 写入文件, 名称为test_save_kdbx4
        std::fs::write("test_save_kdbx4.kdbx", &buffer)?;
        walk_group(
            &new_kdbx.database,
            "",
            &new_kdbx.database.document.root.group,
        );

        Ok(())
    }

    fn walk_group(database: &KeePassDatabase, path: &str, group: &entities::Group) {
        let path = format!("{}/{}", path, group.name);
        for entry in &group.entry {
            walk_entry(database, &path, entry);
        }

        for group in &group.group {
            walk_group(database, &path, group);
        }
    }

    fn walk_entry(database: &KeePassDatabase, path: &str, entry: &entities::Entry) {
        for value in &entry.string {
            let path = format!("{}/{}", path, value.key);
            let value_string = database.get_value_string(&value.value).unwrap();
            println!("{}: {}", path, value_string);
        }
    }
}
