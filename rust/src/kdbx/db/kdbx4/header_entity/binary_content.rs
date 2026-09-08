use crate::kdbx::db::kdbx4::errors::Kdbx4InnerHeaderError;
use crate::utils::writer::{FixedSize, Writable};
use byteorder::WriteBytesExt;
use std::io::{Seek, Write};

#[derive(Clone)]
pub struct BinaryContent {
    pub flag: u8,
    pub content: Vec<u8>,
    pub offset: Option<usize>,
}

impl BinaryContent {
    pub const PROTECTED_FLAG: u8 = 0x01;

    pub fn new(flag: u8, content: Vec<u8>) -> Self {
        Self {
            flag,
            content,
            offset: None,
        }
    }

    pub fn is_protected(&self) -> bool {
        self.flag & Self::PROTECTED_FLAG != 0
    }
}

impl TryFrom<&[u8]> for BinaryContent {
    type Error = Kdbx4InnerHeaderError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(Kdbx4InnerHeaderError::UnexpectedEof);
        }
        Ok(Self {
            flag: data[0],
            content: data[1..].to_vec(),
            offset: None,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_binary_rejected() {
        assert!(matches!(
            BinaryContent::try_from(&[][..]),
            Err(Kdbx4InnerHeaderError::UnexpectedEof)
        ));
    }
}
