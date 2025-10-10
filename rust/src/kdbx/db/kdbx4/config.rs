use crate::kdbx::db::kdbx4::header_entity::{
    compression::CompressionConfig, encryption_algorithm::EncryptionAlgorithm,
    kdf_config::KdfConfig,
};

#[derive(Debug, Clone)]
pub struct Kdbx4Config {
    pub encryption_algorithm: EncryptionAlgorithm,
    pub compression_config: CompressionConfig,
    pub master_salt_seed: [u8; 32],
    pub encryption_iv: Vec<u8>,
    pub kdf_parameters: KdfConfig,
}

impl Kdbx4Config {
    pub fn rekey(&self) -> Result<Self, std::io::Error> {
        let mut master_salt_seed = [0; 32];
        getrandom::fill(&mut master_salt_seed)?;

        Ok(Self {
            master_salt_seed,
            encryption_iv: self.encryption_algorithm.get_random_iv()?,
            encryption_algorithm: self.encryption_algorithm.clone(),
            compression_config: self.compression_config.clone(),
            kdf_parameters: self.kdf_parameters.rekey()?,
        })
    }
}
