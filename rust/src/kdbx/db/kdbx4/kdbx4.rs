use crate::crypto;
use crate::crypto::hash;
use crate::kdbx::db::kdbx::Kdbx;
use crate::kdbx::db::kdbx4::header::Kdbx4Header;
use crate::kdbx::db::kdbx4::inner_header::Kdbx4InnerHeader;
use crate::kdbx::keys::KdbxKey;
use byteorder::{ByteOrder, LittleEndian};
use generic_array::typenum::U64;
use generic_array::GenericArray;
use hex_literal::hex;
use std::io::{Cursor, Read};

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

        let payload_encrypted = parse_hmac_block(&data[header_end_pos + 64..], &hmac_key)?;

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

        let inner_header = Kdbx4InnerHeader::parse(&payload_uncompressed)?;
        let xml = &payload_uncompressed[inner_header.header_size..];

        std::fs::write("demo.xml", xml)?;

        println!("{}", String::from_utf8_lossy(xml));
        Ok(Kdbx4 {})
    }
}

fn parse_hmac_block(data: &[u8], hmac_key: &GenericArray<u8, U64>) -> anyhow::Result<Vec<u8>> {
    let mut total_block: Vec<u8> = Vec::new();
    let mut pos = 0;
    let mut block_index: u64 = 0;

    loop {
        let block_hmac = &data[pos..pos + 32];
        pos += 32;
        let block_length_buf = &data[pos..pos + 4];
        pos += 4;
        let block_length = LittleEndian::read_u32(block_length_buf) as usize;
        let block_data = &data[pos..pos + block_length];
        pos += block_length;

        let mut block_index_buf = [0u8; 8];
        LittleEndian::write_u64(&mut block_index_buf, block_index);

        let hmac_block_key = hash::calculate_sha512_multiple(&[&block_index_buf, &hmac_key]);

        if block_hmac
            != hash::calculate_hmac_multiple(
                &[&block_index_buf, &block_length_buf, &block_data],
                &hmac_block_key,
            )?
            .as_slice()
        {
            return Err(anyhow::anyhow!("Block HMAC checksum mismatch"));
        }

        block_index += 1;
        if block_length == 0 {
            break;
        }
        total_block.extend_from_slice(block_data);
    }

    Ok(total_block)
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
