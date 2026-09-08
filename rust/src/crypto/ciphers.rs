use crate::crypto::errors::CryptoError;
use aes::Aes256;
use block_padding::Pkcs7;
use cipher::{
    BlockDecryptMut, BlockEncryptMut, KeyIvInit, StreamCipher, StreamCipherSeek,
};

pub trait Cipher {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

pub trait StreamCipherExt {
    fn decrypt_at_offset(&mut self, offset: usize, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn current_pos(&self) -> usize;
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
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let data = Aes256CbcEnc::new_from_slices(&self.key, &self.iv)
            .map_err(CryptoError::InvalidLength)?
            .encrypt_padded_vec_mut::<Pkcs7>(data);
        Ok(data)
    }
    fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut output = vec![0u8; ciphertext.len()];
        let decryptor = Aes256CbcDec::new_from_slices(&self.key, &self.iv)
            .map_err(CryptoError::InvalidLength)?;
        let len = decryptor
            .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut output)
            .map_err(CryptoError::UnpadError)?
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
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let encryptor = TwofishCbcEncryptor::new_from_slices(&self.key, &self.iv)
            .map_err(CryptoError::InvalidLength)?;
        let data = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data);
        Ok(data)
    }
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = TwofishCbcDecryptor::new_from_slices(&self.key, &self.iv)
            .map_err(CryptoError::InvalidLength)?;

        let mut buf = data.to_vec();
        let len = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf)?.len();
        buf.truncate(len);
        Ok(buf)
    }
}

pub struct ChaCha20Cipher {
    cipher: chacha20::ChaCha20,
}

impl ChaCha20Cipher {
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, CryptoError> {
        let cipher = chacha20::ChaCha20::new_from_slices(key, iv)
            .map_err(CryptoError::InvalidLength)?;
        Ok(Self { cipher })
    }
}

impl Cipher for ChaCha20Cipher {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
}

impl StreamCipherExt for ChaCha20Cipher {
    fn decrypt_at_offset(&mut self, offset: usize, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.cipher.try_seek(offset as u64)?;
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn current_pos(&self) -> usize {
        self.cipher.current_pos()
    }
}

pub struct Salsa20Cipher {
    cipher: salsa20::Salsa20,
}

impl Salsa20Cipher {
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, CryptoError> {
        let cipher = salsa20::Salsa20::new_from_slices(key, iv)
            .map_err(CryptoError::InvalidLength)?;
        Ok(Self { cipher })
    }
}

impl Cipher for Salsa20Cipher {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buffer = Vec::from(data);
        self.cipher.apply_keystream(&mut buffer);
        Ok(buffer)
    }
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buffer = Vec::from(data);
        self.cipher.apply_keystream(&mut buffer);
        Ok(buffer)
    }
}

impl StreamCipherExt for Salsa20Cipher {
    fn decrypt_at_offset(&mut self, offset: usize, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.cipher.try_seek(offset as u64)?;
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut buf = data.to_vec();
        self.cipher.apply_keystream(&mut buf);
        Ok(buf)
    }

    fn current_pos(&self) -> usize {
        self.cipher.current_pos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twofish_roundtrip_strips_padding() {
        let key = [0x42u8; 32];
        let iv = [0x11u8; 16];
        let mut cipher = TwofishCipher::new(&key, &iv);
        let plaintext = b"short";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        assert_eq!(encrypted.len() % 16, 0);
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_chacha20_rejects_bad_iv() {
        assert!(ChaCha20Cipher::new(&[0u8; 32], &[0u8; 8]).is_err());
    }

    #[test]
    fn test_salsa20_rejects_bad_key() {
        assert!(Salsa20Cipher::new(&[0u8; 16], &[0u8; 8]).is_err());
    }
}
