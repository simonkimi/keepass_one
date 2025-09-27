use byteorder::{ByteOrder, LE};
use generic_array::typenum::U64;
use generic_array::GenericArray;

use crate::crypto::hash;

pub fn parse_hmac_block(data: &[u8], hmac_key: &GenericArray<u8, U64>) -> anyhow::Result<Vec<u8>> {
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
