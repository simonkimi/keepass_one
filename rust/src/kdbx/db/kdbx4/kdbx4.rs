use std::fs::File;
use std::io::Write;

use crate::crypto::hash;
use crate::kdbx::db::kdbx4::header::Kdbx4Header;
use crate::kdbx::db::kdbx4::inner_header::Kdbx4InnerHeader;
use crate::kdbx::xml::database::KeePassDatabase;
use crate::kdbx::xml::entities::KeePassDocument;
use crate::{crypto, kdbx::db::kdbx4::hmac::parse_hmac_block};
use generic_array::{typenum::U32, GenericArray};
use hex_literal::hex;

pub struct Kdbx4 {
    pub key_hash: GenericArray<u8, U32>,
    pub database: KeePassDatabase,
}

impl Kdbx4 {
    pub fn open(data: &[u8], key_hash: &GenericArray<u8, U32>) -> anyhow::Result<Kdbx4> {
        let header = Kdbx4Header::try_from(data)?;
        let header_bytes = &data[..header.size];
        let header_sha256 = &data[header.size..header.size + 32];
        let header_hmac = &data[header.size + 32..header.size + 64];

        if header_sha256 != crypto::hash::calculate_sha256(header_bytes).as_slice() {
            return Err(anyhow::anyhow!("Header SHA-256 checksum mismatch"));
        }

        let transformed_key = header.kdf_parameters.get_kdf().transform_key(key_hash)?;
        let hmac_key = hash::calculate_sha512_multiple(&[
            &header.master_salt_seed,
            &transformed_key,
            &hex!("01"),
        ]);

        let header_hmac_key =
            hash::calculate_sha512_multiple(&[&hex!("FFFFFFFFFFFFFFFF"), &hmac_key]);

        if header_hmac
            != hash::calculate_hmac_multiple(&[&header_bytes], &header_hmac_key)?.as_slice()
        {
            return Err(anyhow::anyhow!("Header HMAC checksum mismatch"));
        }

        let payload_encrypted = parse_hmac_block(&data[header.size + 64..], &hmac_key)?;

        let master_key =
            hash::calculate_sha256_multiple(&[&header.master_salt_seed, &transformed_key]);

        let payload_decrypted = header
            .encryption_algorithm
            .get_cipher(&master_key, &header.encryption_iv)
            .decrypt(&payload_encrypted)?;

        let payload_uncompressed = header
            .compression_config
            .get_compression()
            .decompress(&payload_decrypted)?;

        let inner_header = Kdbx4InnerHeader::try_from(&payload_uncompressed[..])?;
        let xml = &payload_uncompressed[inner_header.header_size..];
        std::fs::write("demo.xml", xml)?;

        let xml_document = KeePassDocument::try_from(xml)?;

        Ok(Self {
            key_hash: key_hash.clone(),
            database: KeePassDatabase::new(xml_document, inner_header),
        })
    }
}

#[cfg(test)]
mod kdbx4_tests {
    use crate::kdbx::{
        db::kdbx4::kdbx4::Kdbx4,
        keys::KdbxKey,
        xml::{database::KeePassDatabase, entities},
    };

    #[test]
    fn test_kdbx4_open() -> anyhow::Result<()> {
        let file_path = r#"C:\Users\ms\Desktop\test.kdbx"#;
        let data = std::fs::read(file_path)?;

        let mut key = KdbxKey::new();
        key.add_master_key("test123456");
        let key_hash = key.calc_key_hash()?;
        let kdbx = Kdbx4::open(&data, &key_hash)?;
        walk_group(&kdbx.database, "", &kdbx.database.document.root.group);
        Ok(())
    }

    fn walk_group(database: &KeePassDatabase, path: &str, group: &entities::Group) {
        let path = format!("{}/{}", path, group.name);
        for entry in &group.entry {
            walk_entry(database, &path, entry, 0);
        }

        for group in &group.group {
            walk_group(database, &path, group);
        }
    }

    fn walk_entry(database: &KeePassDatabase, path: &str, entry: &entities::Entry, entity_index: usize) {
        for value in &entry.string {
            let path = format!("{}/{}", path, value.key);
            if value.value.is_protected() {
                let protected_value = database
                    .decrypt_protected_value(entry.uuid.as_str(), &value.key, entity_index, &value.value.value)
                    .unwrap();
                println!("{}: {}", path, protected_value);
            }
        }
        if let Some(history) = &entry.history {
            for (index, history_entry) in history.entry.iter().enumerate() {
                walk_entry(database, &path, history_entry, index + 1);
            }
        }
    }
}
