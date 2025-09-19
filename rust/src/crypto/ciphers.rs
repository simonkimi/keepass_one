use crate::crypto;
use aes::Aes256;
use block_padding::Pkcs7;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use generic_array::GenericArray;

pub trait Cipher {
    fn encrypt(&mut self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>>;
    fn decrypt(&mut self, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>>;
}

pub struct AES256Cipher {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl AES256Cipher {
    pub fn new(key: &[u8], iv: &[u8]) -> Self {
        Self {
            key: key.to_vec(),
            iv: iv.to_vec(),
        }
    }
}

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;
impl Cipher for AES256Cipher {
    fn encrypt(&mut self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = Aes256CbcEnc::new_from_slices(&self.key, &self.iv)?
            .encrypt_padded_vec_mut::<Pkcs7>(plaintext);
        Ok(data)
    }
    fn decrypt(&mut self, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut output = vec![0u8; ciphertext.len()];
        let decryptor = Aes256CbcDec::new_from_slices(&self.key, &self.iv)?;
        let len = decryptor
            .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut output)?
            .len();
        output.truncate(len);
        Ok(output)
    }
}

type TwofishCbcEncryptor = cbc::Encryptor<twofish::Twofish>;
type TwofishCbcDecryptor = cbc::Decryptor<twofish::Twofish>;

pub struct TwofishCipher {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl TwofishCipher {
    pub fn new(key: &[u8], iv: &[u8]) -> Self {
        Self {
            key: key.to_vec(),
            iv: iv.to_vec(),
        }
    }
}

impl Cipher for TwofishCipher {
    fn encrypt(&mut self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let encryptor = TwofishCbcEncryptor::new_from_slices(&self.key, &self.iv)?;
        let data = encryptor.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
        Ok(data)
    }
    fn decrypt(&mut self, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let cipher = TwofishCbcDecryptor::new_from_slices(&self.key, &self.iv)?;

        let mut buf = ciphertext.to_vec();
        cipher.decrypt_padded_mut::<Pkcs7>(&mut buf)?;
        Ok(buf)
    }
}

// struct ChaCha20Cipher {
//     cipher: chacha20::ChaCha20,
// }
//
// impl ChaCha20Cipher {
//     pub fn new(key: &[u8]) -> Self {
//         let iv = crypto::hash::calculate_sha512(&key);
//         let key = GenericArray::from_slice(&iv[0..32]);
//         let nonce = GenericArray::from_slice(&iv[32..44]);
//         let cipher = chacha20::ChaCha20::new(key, nonce);
//         Self { cipher }
//     }
//
//     pub fn new_with_iv(key: &[u8], iv: &[u8]) -> anyhow::Result<Self> {
//         Ok(Self {
//             cipher: chacha20::ChaCha20::new_from_slices(key, iv)?,
//         })
//     }
// }
