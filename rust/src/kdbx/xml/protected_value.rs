use crate::{
    crypto::ciphers::StreamCipherExt,
    kdbx::xml::entities::{Entry, Group, KeePassDocument, Value},
};

pub fn collect_protected_values_document(document: &mut KeePassDocument) {
    collect_protected_values_group(&mut document.root.group, 0);
}

fn collect_protected_values_group(group: &mut Group, stream_offset: usize) -> usize {
    let mut stream_offset = stream_offset;
    for entry in &mut group.entry {
        stream_offset = collect_protected_values_entry(entry, stream_offset);
    }
    for group in &mut group.group {
        stream_offset = collect_protected_values_group(group, stream_offset);
    }
    stream_offset
}

fn collect_protected_values_entry(entry: &mut Entry, stream_offset: usize) -> usize {
    let mut stream_offset = stream_offset;

    stream_offset = process_protected_values(stream_offset, entry);
    if let Some(ref mut history) = entry.history {
        for history_entry in &mut history.entry {
            stream_offset = process_protected_values(stream_offset, history_entry);
        }
    }
    stream_offset
}

fn process_protected_values(stream_offset: usize, entry: &mut Entry) -> usize {
    let mut stream_offset = stream_offset;
    for value in &mut entry.string {
        if let Value::Protected {
            ref value,
            ref mut offset,
        } = value.value
        {
            *offset = Some(stream_offset);
            stream_offset += value.len();
        }
    }
    stream_offset
}

pub fn encrypt_protected_value(
    document: &mut KeePassDocument,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) {
    encrypt_protected_values_group(&mut document.root.group, old_cipher, new_cipher);
}

fn encrypt_protected_values_group(
    group: &mut Group,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) {
    for entry in &mut group.entry {
        encrypt_protected_values_entry(entry, old_cipher, new_cipher);
    }
    for group in &mut group.group {
        encrypt_protected_values_group(group, old_cipher, new_cipher);
    }
}

fn encrypt_protected_values_entry(
    entry: &mut Entry,
    old_cipher: &mut Box<dyn StreamCipherExt>,
    new_cipher: &mut Box<dyn StreamCipherExt>,
) {
    for value in &mut entry.string {
        let new_value = match &value.value {
            Value::Protected { ref value, .. } => {
                let data = old_cipher.decrypt(value).unwrap();
                let offset = new_cipher.current_pos();
                let new_data = new_cipher.encrypt(&data).unwrap();
                Some(Value::Protected {
                    value: new_data,
                    offset: Some(offset),
                })
            }
            Value::WaitProtect(ref value) => {
                let offset = new_cipher.current_pos();
                let new_data = new_cipher.encrypt(value.as_bytes()).unwrap();
                Some(Value::Protected {
                    value: new_data,
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
            encrypt_protected_values_entry(history_entry, old_cipher, new_cipher);
        }
    }
}
