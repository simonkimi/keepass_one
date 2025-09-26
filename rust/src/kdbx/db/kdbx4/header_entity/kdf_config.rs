use std::collections::HashMap;

use crate::{
    crypto,
    kdbx::db::kdbx4::header_entity::variant_dictionary::{
        VariantDictionary, VariantDictionaryValue,
    },
};
use hex_literal::hex;

const KDF_AES: [u8; 16] = hex!("C9D9F39A628A4460BF740D08C18A4FEA");
const KDF_ARGON2D: [u8; 16] = hex!("EF636DDF8C29444B91F7A9A403E30A0C");
const KDF_ARGON2ID: [u8; 16] = hex!("9E298B1956DB4773B23DFC3EC6F0A1E6");

const KDF_UUID_KEY: &str = "$UUID";
const AES_SALT_KEY: &str = "S";
const AES_ROUNDS_KEY: &str = "R";

const ARGON2_SALT_KEY: &str = "S";
const ARGON2_VERSION_KEY: &str = "V";
const ARGON2_ITERATIONS_KEY: &str = "I";
const ARGON2_MEMORY_KEY: &str = "M";
const ARGON2_PARALLELISM_KEY: &str = "P";

const ARGON2_VERSION_10: u32 = 0x10;
const ARGON2_VERSION_13: u32 = 0x13;

#[derive(Debug, PartialEq)]
pub enum KdfConfig {
    Aes {
        salt: [u8; 32],
        rounds: u64,
    },
    Argon2 {
        version: u32,
        salt: Vec<u8>,
        iterations: u64,
        memory: u64,
        parallelism: u32,
        variant: argon2::Variant,
    },
}

impl TryFrom<&[u8]> for KdfConfig {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let vd = VariantDictionary::parse(value)?;
        let kdf = parse_kdf_keys(&vd)?;
        Ok(kdf)
    }
}

impl KdfConfig {
    pub fn get_kdf(&self) -> Box<dyn crypto::kdf::Kdf> {
        match self {
            KdfConfig::Aes { salt, rounds } => Box::new(crypto::kdf::AesKdf {
                seed: salt.to_vec(),
                rounds: *rounds,
            }),
            KdfConfig::Argon2 {
                version,
                salt,
                iterations,
                memory,
                parallelism,
                variant,
            } => {
                Box::new(crypto::kdf::Argon2Kdf {
                    version: match version {
                        &ARGON2_VERSION_10 => argon2::Version::Version10,
                        &ARGON2_VERSION_13 => argon2::Version::Version13,
                        _ => argon2::Version::Version13, // 默认使用最新版本
                    },
                    salt: salt.to_vec(),
                    iterations: *iterations,
                    memory: *memory,
                    parallelism: *parallelism,
                    variant: *variant,
                })
            }
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut map: HashMap<String, VariantDictionaryValue> = HashMap::new();
        match self {
            KdfConfig::Aes { salt, rounds } => {
                map.insert(
                    KDF_UUID_KEY.to_string(),
                    VariantDictionaryValue::ByteArray(KDF_AES.to_vec()),
                );
                map.insert(
                    AES_SALT_KEY.to_string(),
                    VariantDictionaryValue::ByteArray(salt.to_vec()),
                );
                map.insert(
                    AES_ROUNDS_KEY.to_string(),
                    VariantDictionaryValue::UInt64(*rounds),
                );
            }
            KdfConfig::Argon2 {
                version,
                salt,
                iterations,
                memory,
                parallelism,
                variant,
            } => {
                let uuid = match variant {
                    argon2::Variant::Argon2d => KDF_ARGON2D.to_vec(),
                    _ => KDF_ARGON2ID.to_vec(),
                };
                map.insert(
                    KDF_UUID_KEY.to_string(),
                    VariantDictionaryValue::ByteArray(uuid),
                );
                map.insert(
                    ARGON2_VERSION_KEY.to_string(),
                    VariantDictionaryValue::UInt32(*version),
                );
                map.insert(
                    ARGON2_SALT_KEY.to_string(),
                    VariantDictionaryValue::ByteArray(salt.to_vec()),
                );
                map.insert(
                    ARGON2_ITERATIONS_KEY.to_string(),
                    VariantDictionaryValue::UInt64(*iterations),
                );
                map.insert(
                    ARGON2_MEMORY_KEY.to_string(),
                    VariantDictionaryValue::UInt64(*memory),
                );
                map.insert(
                    ARGON2_PARALLELISM_KEY.to_string(),
                    VariantDictionaryValue::UInt32(*parallelism),
                );
            }
        }
        VariantDictionary::from(map).write()
    }
}

fn parse_kdf_keys(vd: &VariantDictionary) -> anyhow::Result<KdfConfig> {
    let uuid: &Vec<u8> = vd.get(KDF_UUID_KEY)?;
    if uuid == &KDF_AES {
        let salt: &Vec<u8> = vd.get(AES_SALT_KEY)?;
        if salt.len() != 32 {
            return Err(anyhow::anyhow!("Aes密钥长度不匹配"));
        }

        let rounds: &u64 = vd.get(AES_ROUNDS_KEY)?;

        Ok(KdfConfig::Aes {
            salt: salt[..].try_into()?,
            rounds: *rounds,
        })
    } else if uuid == &KDF_ARGON2D || uuid == &KDF_ARGON2ID {
        let version: &u32 = vd.get(ARGON2_VERSION_KEY)?;
        let salt: &Vec<u8> = vd.get(ARGON2_SALT_KEY)?;
        let iterations: &u64 = vd.get(ARGON2_ITERATIONS_KEY)?;
        let memory: &u64 = vd.get(ARGON2_MEMORY_KEY)?;
        let parallelism: &u32 = vd.get(ARGON2_PARALLELISM_KEY)?;

        if version != &ARGON2_VERSION_10 && version != &ARGON2_VERSION_13 {
            return Err(anyhow::anyhow!("不支持的Argon2版本"));
        }

        Ok(KdfConfig::Argon2 {
            version: *version,
            salt: salt[..].into(),
            iterations: *iterations,
            memory: *memory,
            parallelism: *parallelism,
            variant: if uuid == &KDF_ARGON2D {
                argon2::Variant::Argon2d
            } else {
                argon2::Variant::Argon2id
            },
        })
    } else {
        Err(anyhow::anyhow!("不支持的KDF"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_kdf_config_write_and_read() {
        let kdf_config = KdfConfig::Aes {
            salt: [1; 32],
            rounds: 100,
        };

        let data = kdf_config.write();
        let kdf_config2 = KdfConfig::try_from(data.as_slice()).unwrap();

        assert_eq!(kdf_config, kdf_config2);
    }

    #[test]
    fn test_argon2_kdf_config_write_and_read() {
        let kdf_config = KdfConfig::Argon2 {
            version: 0x13,
            salt: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            iterations: 100,
            memory: 1024,
            parallelism: 8,
            variant: argon2::Variant::Argon2id,
        };

        let data = kdf_config.write();
        let kdf_config2 = KdfConfig::try_from(data.as_slice()).unwrap();

        assert_eq!(kdf_config, kdf_config2);
    }
}
