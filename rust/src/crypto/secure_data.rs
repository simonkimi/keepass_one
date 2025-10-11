use std::fmt;
use std::marker::PhantomData;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::crypto::memory_crypt::{memory_crypt, SecureDataError};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureData {
    data: Vec<u8>,
    original_len: usize,
    _marker: PhantomData<*const ()>,

    is_crypt: bool,
    is_mlocked: bool,
}

impl PartialEq for SecureData {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl SecureData {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            original_len: data.len(),
            _marker: PhantomData,
            is_crypt: false,
            is_mlocked: false,
        }
    }

    pub fn crypt(&mut self) -> Result<(), SecureDataError> {
        self.data = memory_crypt::crypt_memory(&self.data)?;
        self.is_crypt = true;
        self.is_mlocked = true;
        Ok(())
    }

    pub fn mlock(&mut self) -> Result<(), SecureDataError> {
        memory_crypt::mlock(&mut self.data)?;
        self.is_mlocked = true;
        Ok(())
    }

    pub fn is_mlocked(&self) -> bool {
        self.is_mlocked
    }

    pub fn len(&self) -> usize {
        self.original_len
    }

    pub fn unsecure(&self) -> Result<Zeroizing<Vec<u8>>, SecureDataError> {
        if !self.is_crypt {
            return Ok(Zeroizing::new(self.data.clone()));
        }

        let data = memory_crypt::uncrypt_memory(&self.data, self.original_len)?;
        Ok(Zeroizing::new(data))
    }
}

impl fmt::Debug for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecureData")
            .field("protected_data", &"***SECRET***")
            .field("original_len", &self.original_len)
            .finish()
    }
}

impl fmt::Display for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecureData {{ original_len: {} }}", self.original_len)
    }
}

impl Clone for SecureData {
    fn clone(&self) -> Self {
        let mut cloned = Self {
            data: self.data.clone(),
            original_len: self.original_len,
            _marker: PhantomData,
            is_crypt: self.is_crypt,
            is_mlocked: false,
        };

        if self.is_mlocked() {
            _ = cloned.mlock();
        }

        cloned
    }
}
