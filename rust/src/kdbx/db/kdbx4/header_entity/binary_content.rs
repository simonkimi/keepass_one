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
