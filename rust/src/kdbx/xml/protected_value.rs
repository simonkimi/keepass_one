use crate::{
    crypto::{ciphers::StreamCipherExt, secure_data::SecureData},
    kdbx::{
        config::MemoryProtectConfig,
        xml::{
            entities::{Entry, Group, KeePassFile, Value},
            errors::{KdbxDatabaseError, KdbxSaveError},
        },
    },
};

pub fn collect_protected_values_document(
    document: &mut KeePassFile,
    config: &MemoryProtectConfig,
) -> Result<(), KdbxDatabaseError> {
    collect_protected_values_group(&mut document.root.group, 0, config)?;
    Ok(())
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
            Value::Protected { ref value, .. } => {
                let protected_data = value.unsecure()?;
                let data = old_cipher.decrypt(&protected_data)?;
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
