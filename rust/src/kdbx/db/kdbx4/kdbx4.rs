use crate::{crypto, kdbx::db::kdbx4::hmac::parse_hmac_block};
use crate::crypto::hash;
use crate::kdbx::db::kdbx4::header::Kdbx4Header;
use crate::kdbx::db::kdbx4::inner_header::Kdbx4InnerHeader;
use byteorder::{ByteOrder, LittleEndian};
use generic_array::{
    typenum::{U32, U64},
    GenericArray,
};
use hex_literal::hex;

pub struct Kdbx4 {

}

impl Kdbx4 {
    pub fn open(data: &[u8], key_hash: &GenericArray<u8, U32>) -> anyhow::Result<Self> {
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

        println!("{}", String::from_utf8_lossy(xml));
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
        let key_hash = key.calc_key_hash().unwrap();
        Kdbx4::open(&data, &key_hash).unwrap();
    }
}
