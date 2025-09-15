use std::io::Cursor;

use anyhow::Ok;

use crate::kdbx::db::{kdbx::Kdbx, version::KdbxVersion};

struct Kdbx4 {}

impl Kdbx for Kdbx4 {}

impl Kdbx4 {
    pub fn open(data: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(data);
        let version = KdbxVersion::parse(&mut cursor)?;
        match version {
            KdbxVersion::KDB4(_) => Ok(Kdbx4 {}),
            _ => Err(anyhow::anyhow!("Invalid KDBX version")),
        }

        
    }
}
