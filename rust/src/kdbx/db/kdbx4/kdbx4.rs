use std::io::Cursor;

use crate::kdbx::db::{kdbx::Kdbx, version::KdbxVersion};

const KDBX4_HEADER_SIZE: usize = 12;

struct Kdbx4 {}

impl Kdbx for Kdbx4 {}

impl Kdbx4 {
    pub fn open(data: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        // let header_sha256 = &data[KDBX4_HEADER_SIZE..(KDBX4_HEADER_SIZE + 32)];
        // let header_hmac = &data[(KDBX4_HEADER_SIZE + 32)..(KDBX4_HEADER_SIZE + 64)];
        // let hmac_block = &data[(KDBX4_HEADER_SIZE + 64)..];
        


        

        Ok(Kdbx4 {})
    }
}
