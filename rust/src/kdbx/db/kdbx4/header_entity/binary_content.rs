use crate::utils::writer::{FixedSize, Writable};
use byteorder::WriteBytesExt;
use std::io::{Seek, Write};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct BinaryContent {
    pub flag: u8,
    pub content: Vec<u8>,
}

impl From<&[u8]> for BinaryContent {
    fn from(data: &[u8]) -> Self {
        let flag = data[0];
        let content = data[1..].to_vec();
        Self { flag, content }
    }
}

impl Writable for BinaryContent {
    fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_u8(self.flag)?;
        writer.write_all(&self.content)?;
        Ok(())
    }
}

impl FixedSize for BinaryContent {
    fn fix_size(&self) -> usize {
        self.content.len() + size_of::<u8>()
    }
}
