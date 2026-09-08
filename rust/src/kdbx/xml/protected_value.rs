use crate::{
    crypto::{ciphers::StreamCipherExt, secure_data::SecureData},
    kdbx::{
        config::MemoryProtectConfig,
        db::kdbx4::header_entity::binary_content::BinaryContent,
        xml::{
            entities::{Entry, Group, KeePassFile, Value},
            errors::{KdbxDatabaseError, KdbxSaveError},
        },
    },
};

pub fn collect_protected_values_document(
    document: &mut KeePassFile,
    binaries: &mut [BinaryContent],
    config: &MemoryProtectConfig,
) -> Result<(), KdbxDatabaseError> {
    let stream_offset = assign_protected_binary_offsets(binaries);
    collect_protected_values_group(&mut document.root.group, stream_offset, config)?;
    Ok(())
}

pub(crate) fn assign_protected_binary_offsets(binaries: &mut [BinaryContent]) -> usize {
    let mut stream_offset = 0;
    for binary in binaries {
        if binary.is_protected() {
            binary.offset = Some(stream_offset);
            stream_offset += binary.content.len();
        }
    }
    stream_offset
}

fn collect_protected_values_group(
    group: &mut Group,
    stream_offset: usize,
    config: &MemoryProtectConfig,
) -> Result<usize, KdbxDatabaseError> {
    let mut stream_offset = stream_offset;
    for entry in &mut group.entry {
        stream_offset = collect_protected_values_entry(entry, stream_offset, config)?;
    }
    for group in &mut group.group {
        stream_offset = collect_protected_values_group(group, stream_offset, config)?;
    }
    Ok(stream_offset)
}

fn collect_protected_values_entry(
    entry: &mut Entry,
    stream_offset: usize,
    config: &MemoryProtectConfig,
) -> Result<usize, KdbxDatabaseError> {
    let mut stream_offset = stream_offset;

    stream_offset = process_protected_values(stream_offset, entry, config)?;
    if let Some(ref mut history) = entry.history {
        for history_entry in &mut history.entry {
            stream_offset = process_protected_values(stream_offset, history_entry, config)?;
        }
    }
    Ok(stream_offset)
}

fn process_protected_values(
    stream_offset: usize,
    entry: &mut Entry,
    config: &MemoryProtectConfig,
) -> Result<usize, KdbxDatabaseError> {
    let mut stream_offset = stream_offset;
    for value in &mut entry.string {
        if let Value::Protected {
            ref mut value,
            ref mut offset,
        } = value.value
        {
            *offset = Some(stream_offset);
            stream_offset += value.len();
            if config.enable_memory_crypt {
                value.crypt()?;
            }
            if config.enable_mlock {
                value.mlock()?;
            }
        }
    }
    Ok(stream_offset)
}

pub fn encrypt_protected_binaries(
    binaries: &mut [BinaryContent],
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) -> Result<(), KdbxSaveError> {
    for binary in binaries {
        if !binary.is_protected() {
            continue;
        }
        let plaintext = match binary.offset {
            Some(offset) => old_cipher.decrypt_at_offset(offset, &binary.content)?,
            None => binary.content.clone(),
        };
        let offset = new_cipher.current_pos();
        binary.content = new_cipher.encrypt(&plaintext)?;
        binary.offset = Some(offset);
    }
    Ok(())
}

pub fn encrypt_protected_value(
    document: &mut KeePassFile,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) -> Result<(), KdbxSaveError> {
    encrypt_protected_values_group(&mut document.root.group, old_cipher, new_cipher)?;
    Ok(())
}

fn encrypt_protected_values_group(
    group: &mut Group,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) -> Result<(), KdbxSaveError> {
    for entry in &mut group.entry {
        encrypt_protected_values_entry(entry, old_cipher, new_cipher)?;
    }
    for group in &mut group.group {
        encrypt_protected_values_group(group, old_cipher, new_cipher)?;
    }
    Ok(())
}

fn encrypt_protected_values_entry(
    entry: &mut Entry,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) -> Result<(), KdbxSaveError> {
    for value in &mut entry.string {
        let new_value = match &value.value {
            Value::Protected {
                ref value,
                offset,
            } => {
                let protected_data = value.unsecure()?;
                let data = match offset {
                    Some(off) => old_cipher.decrypt_at_offset(*off, &protected_data)?,
                    None => old_cipher.decrypt(&protected_data)?,
                };
                let offset = new_cipher.current_pos();
                let new_data = new_cipher.encrypt(&data)?;
                Some(Value::Protected {
                    value: SecureData::new(&new_data),
                    offset: Some(offset),
                })
            }
            Value::WaitProtect(ref value) => {
                let offset = new_cipher.current_pos();
                let new_data = new_cipher.encrypt(value.as_bytes())?;
                Some(Value::Protected {
                    value: SecureData::new(&new_data),
                    offset: Some(offset),
                })
            }
            _ => None,
        };
        if let Some(new_value) = new_value {
            value.value = new_value;
        }
    }
    if let Some(ref mut history) = entry.history {
        for history_entry in &mut history.entry {
            encrypt_protected_values_entry(history_entry, old_cipher, new_cipher)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kdbx::db::kdbx4::header_entity::binary_content::BinaryContent;

    #[test]
    fn test_protected_binary_offsets_precede_strings() {
        let mut binaries = vec![
            BinaryContent::new(0, vec![1, 2, 3]),
            BinaryContent::new(BinaryContent::PROTECTED_FLAG, vec![0; 10]),
            BinaryContent::new(BinaryContent::PROTECTED_FLAG, vec![0; 4]),
        ];
        let offset = assign_protected_binary_offsets(&mut binaries);
        assert_eq!(offset, 14);
        assert_eq!(binaries[0].offset, None);
        assert_eq!(binaries[1].offset, Some(0));
        assert_eq!(binaries[2].offset, Some(10));
    }
}
