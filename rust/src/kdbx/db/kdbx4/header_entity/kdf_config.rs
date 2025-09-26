use crate::{crypto, kdbx::db::kdbx4::header_entity::variant_dictionary::VariantDictionary};
use hex_literal::hex;

const KDF_AES: [u8; 16] = hex!("C9D9F39A628A4460BF740D08C18A4FEA");
const KDF_ARGON2D: [u8; 16] = hex!("EF636DDF8C29444B91F7A9A403E30A0C");
const KDF_ARGON2ID: [u8; 16] = hex!("9E298B1956DB4773B23DFC3EC6F0A1E6");

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
                        0x10 => argon2::Version::Version10,
                        0x13 => argon2::Version::Version13,
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
}

fn parse_kdf_keys(vd: &VariantDictionary) -> anyhow::Result<KdfConfig> {
    let uuid: &Vec<u8> = vd.get("$UUID")?;
    if uuid == &KDF_AES {
        let salt: &Vec<u8> = vd.get("S")?;
        if salt.len() != 32 {
            return Err(anyhow::anyhow!("Aes密钥长度不匹配"));
        }

        let rounds: &u64 = vd.get("R")?;

        Ok(KdfConfig::Aes {
            salt: salt[..].try_into()?,
            rounds: *rounds,
        })
    } else if uuid == &KDF_ARGON2D || uuid == &KDF_ARGON2ID {
        let version: &u32 = vd.get("V")?;
        let salt: &Vec<u8> = vd.get("S")?;
        let iterations: &u64 = vd.get("I")?;
        let memory: &u64 = vd.get("M")?;
        let parallelism: &u32 = vd.get("P")?;

        if version != &0x10 && version != &0x13 {
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
