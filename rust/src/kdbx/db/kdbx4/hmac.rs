use byteorder::{ByteOrder, LE};
use generic_array::typenum::U64;
use generic_array::GenericArray;
use hex_literal::hex;

use crate::crypto::errors::CryptoError;
use crate::crypto::hash;

const HMAC_BLOCK_SIZE: usize = 1024 * 1024; // 1MB
const KDBX4_MAIN_HMAC_SUFFIX: [u8; 1] = hex!("01");
const KDBX4_HEADER_HMAC_SUFFIX: [u8; 8] = hex!("FFFFFFFFFFFFFFFF");

pub fn parse_hmac_block(
    data: &[u8],
    hmac_key: &GenericArray<u8, U64>,
) -> Result<Vec<u8>, CryptoError> {
    let mut total_block: Vec<u8> = Vec::new();
    let mut pos = 0;
    let mut block_index: u64 = 0;

    loop {
        let block_hmac = &data[pos..pos + 32];
        pos += 32;
        let block_length_buf = &data[pos..pos + 4];
        pos += 4;
        let block_length = LE::read_u32(block_length_buf) as usize;
        let block_data = &data[pos..pos + block_length];
        pos += block_length;

        let mut block_index_buf = [0u8; 8];
        LE::write_u64(&mut block_index_buf, block_index);

        let hmac_block_key = hash::calculate_sha512_multiple(&[&block_index_buf, &hmac_key]);

        if block_hmac
            != hash::calculate_hmac_multiple(
                &[&block_index_buf, &block_length_buf, &block_data],
                &hmac_block_key,
            )?
            .as_slice()
        {
            return Err(CryptoError::HmacMismatch);
        }

        block_index += 1;
        if block_length == 0 {
            break;
        }
        total_block.extend_from_slice(block_data);
    }

    Ok(total_block)
}

pub fn write_hmac_block<W: std::io::Write + std::io::Seek>(
    data: &[u8],
    hmac_key: &GenericArray<u8, U64>,
    writer: &mut W,
) -> Result<(), CryptoError> {
    let mut pos = 0;
    let mut block_index: u64 = 0;

    while pos < data.len() {
        let remaining = data.len() - pos;
        let block_size = if remaining > HMAC_BLOCK_SIZE {
            HMAC_BLOCK_SIZE
        } else {
            remaining
        };

        let block_data = &data[pos..pos + block_size];
        pos += block_size;

        let mut block_index_buf = [0u8; 8];
        LE::write_u64(&mut block_index_buf, block_index);

        let mut block_length_buf = [0u8; 4];
        LE::write_u32(&mut block_length_buf, block_size as u32);
        let hmac_block_key = hash::calculate_sha512_multiple(&[&block_index_buf, &hmac_key]);
        let block_hmac = hash::calculate_hmac_multiple(
            &[&block_index_buf, &block_length_buf, &block_data],
            &hmac_block_key,
        )?;

        writer.write_all(&block_hmac)?;

        writer.write_all(&block_length_buf)?;

        writer.write_all(block_data)?;

        block_index += 1;
    }

    let mut block_index_buf = [0u8; 8];
    LE::write_u64(&mut block_index_buf, block_index);

    let mut block_length_buf = [0u8; 4];
    LE::write_u32(&mut block_length_buf, 0);
    let hmac_block_key = hash::calculate_sha512_multiple(&[&block_index_buf, &hmac_key]);

    let block_hmac = hash::calculate_hmac_multiple(
        &[&block_index_buf, &block_length_buf, &[]], // 空数据
        &hmac_block_key,
    )?;

    writer.write_all(&block_hmac)?;
    writer.write_all(&block_length_buf)?;

    Ok(())
}

pub fn calc_kdbx4_hmac_key(salt: &[u8], transformed_key: &[u8]) -> GenericArray<u8, U64> {
    hash::calculate_sha512_multiple(&[&salt, &transformed_key, &KDBX4_MAIN_HMAC_SUFFIX])
}

pub fn calc_kdbx4_header_hmac_key(hmac_key: &GenericArray<u8, U64>) -> GenericArray<u8, U64> {
    hash::calculate_sha512_multiple(&[&KDBX4_HEADER_HMAC_SUFFIX, &hmac_key])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hmac_block_roundtrip() {
        let test_data =
            b"Hello, World! This is a test data for HMAC block functionality. ".repeat(1000);
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(&test_data, &hmac_key, &mut cursor).unwrap();

        let parsed_data = parse_hmac_block(&buffer, &hmac_key).unwrap();

        assert_eq!(test_data, parsed_data.as_slice());
    }

    #[test]
    fn test_hmac_block_empty_data() {
        let test_data = b"";
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(test_data, &hmac_key, &mut cursor).unwrap();
        let parsed_data = parse_hmac_block(&buffer, &hmac_key).unwrap();

        assert_eq!(test_data, parsed_data.as_slice());
    }

    #[test]
    fn test_hmac_block_single_block() {
        let test_data = b"Short test data";
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(test_data, &hmac_key, &mut cursor).unwrap();
        let parsed_data = parse_hmac_block(&buffer, &hmac_key).unwrap();

        assert_eq!(test_data, parsed_data.as_slice());
    }

    #[test]
    fn test_hmac_block_multiple_blocks() {
        let test_data = vec![0x42u8; HMAC_BLOCK_SIZE * 2 + 1000];
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(&test_data, &hmac_key, &mut cursor).unwrap();
        let parsed_data = parse_hmac_block(&buffer, &hmac_key).unwrap();

        assert_eq!(test_data, parsed_data.as_slice());
    }

    #[test]
    fn test_hmac_block_wrong_key() {
        let test_data = b"Test data";
        let correct_key = GenericArray::from([0x42u8; 64]);
        let wrong_key = GenericArray::from([0x24u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(test_data, &correct_key, &mut cursor).unwrap();

        let result = parse_hmac_block(&buffer, &wrong_key);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CryptoError::HmacMismatch));
    }

    #[test]
    fn test_hmac_block_corrupted_data() {
        let test_data = b"Test data";
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(test_data, &hmac_key, &mut cursor).unwrap();

        buffer[10] = !buffer[10];

        let result = parse_hmac_block(&buffer, &hmac_key);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CryptoError::HmacMismatch));
    }

    #[test]
    fn test_hmac_block_exact_block_size() {
        let test_data = vec![0x42u8; HMAC_BLOCK_SIZE];
        let hmac_key = GenericArray::from([0x42u8; 64]);

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_hmac_block(&test_data, &hmac_key, &mut cursor).unwrap();
        let parsed_data = parse_hmac_block(&buffer, &hmac_key).unwrap();

        assert_eq!(test_data, parsed_data.as_slice());
    }
}
