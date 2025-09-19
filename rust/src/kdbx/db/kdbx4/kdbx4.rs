use crate::crypto;
use crate::kdbx::db::kdbx::Kdbx;
use crate::kdbx::db::kdbx4::kdbx4_header::Kdbx4Header;
use crate::kdbx::keys::KdbxKey;
use hex_literal::hex;

pub struct Kdbx4 {}

impl Kdbx for Kdbx4 {}

impl Kdbx4 {
    pub fn open(data: &[u8], db_key: &KdbxKey) -> anyhow::Result<Self> {
        let (header, header_end_pos) = Kdbx4Header::parse(data)?;
        let header_bytes = &data[..header_end_pos];
        let header_sha256 = &data[header_end_pos..header_end_pos + 32];
        let header_hmac = &data[header_end_pos + 32..header_end_pos + 64];

        if header_sha256 != crypto::hash::calculate_sha256(header_bytes).as_slice() {
            return Err(anyhow::anyhow!("Header SHA-256 checksum mismatch"));
        }

        let key_hash = db_key.calc_key_hash()?;
        let transformed_key = header.kdf_parameters.get_kdf().transform_key(&key_hash)?;

        let hmac_key = crypto::hash::calculate_sha512_multiple(&[
            &header.master_salt_seed,
            &transformed_key,
            &hex!("01"),
        ]);

        let header_hmac_key =
            crypto::hash::calculate_sha512_multiple(&[&hex!("FFFFFFFFFFFFFFFF"), &hmac_key]);

        if header_hmac
            != crypto::hash::calculate_hmac_multiple(&[&header_bytes], &header_hmac_key)?.as_slice()
        {
            return Err(anyhow::anyhow!("Header HMAC checksum mismatch"));
        }

        Ok(Kdbx4 {})
    }
}

#[cfg(test)]
mod kdbx4_tests {
    use crate::kdbx::{db::kdbx4::kdbx4::Kdbx4, keys::KdbxKey};

    #[test]
    fn test_kdbx4_open() {
        let file_path = r#"C:\Users\ms\Desktop\test.kdbx"#;
        let data = std::fs::read(file_path).unwrap();

        let mut key = KdbxKey::new();
        key.add_master_key("test123456");

        Kdbx4::open(&data, &key).unwrap();
    }
}
