use aes::cipher::{BlockEncrypt, KeyInit};
use aes::Aes256;
use generic_array::typenum::U32;
use generic_array::GenericArray;
use sha2::{Digest, Sha256};

pub trait Kdf {
    fn transform_key(&self, key: &GenericArray<u8, U32>) -> anyhow::Result<GenericArray<u8, U32>>;
}

pub struct AesKdf {
    pub seed: Vec<u8>,
    pub rounds: u64,
}

impl Kdf for AesKdf {
    fn transform_key(&self, key: &GenericArray<u8, U32>) -> anyhow::Result<GenericArray<u8, U32>> {
        let seed_array = GenericArray::from_slice(&self.seed);
        let cipher = Aes256::new(seed_array);

        let mut block1 = GenericArray::clone_from_slice(&key[..16]);
        let mut block2 = GenericArray::clone_from_slice(&key[16..]);
        for _ in 0..self.rounds {
            cipher.encrypt_block(&mut block1);
            cipher.encrypt_block(&mut block2);
        }

        let mut digest = Sha256::new();

        digest.update(block1);
        digest.update(block2);

        Ok(digest.finalize())
    }
}

pub struct Argon2Kdf {
    pub memory: u64,
    pub salt: Vec<u8>,
    pub iterations: u64,
    pub parallelism: u32,
    pub version: argon2::Version,
    pub variant: argon2::Variant,
}

impl Kdf for Argon2Kdf {
    fn transform_key(&self, key: &GenericArray<u8, U32>) -> anyhow::Result<GenericArray<u8, U32>> {
        let config = argon2::Config {
            thread_mode: argon2::ThreadMode::Parallel,
            ad: &[],
            hash_length: 32,
            lanes: self.parallelism,
            mem_cost: (self.memory / 1024) as u32,
            secret: &[],
            time_cost: self.iterations as u32,
            variant: self.variant,
            version: self.version,
        };

        let key = argon2::hash_raw(key, &self.salt, &config)?;
        Ok(*GenericArray::from_slice(&key))
    }
}
