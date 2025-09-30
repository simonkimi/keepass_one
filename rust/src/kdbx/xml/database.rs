use std::collections::HashMap;

use crate::{
    kdbx::{
        db::kdbx4::inner_header::Kdbx4InnerHeader,
        xml::{
            entities::{Entry, Group, KeePassDocument, StringField},
            errors::KdbxDatabaseError,
        },
    },
    utils,
};

pub struct KeePassDatabase {
    pub document: KeePassDocument,
    pub inner_header: Kdbx4InnerHeader,
    pub protect_values: HashMap<String, usize>,
}

impl KeePassDatabase {
    pub fn try_from(xml: &[u8], inner_header: Kdbx4InnerHeader) -> Result<Self, KdbxDatabaseError> {
        let document: KeePassDocument = quick_xml::de::from_reader(xml)?;
        let protected_values = collect_protected_values_document(&document);
        Ok(Self {
            document,
            inner_header,
            protect_values: protected_values,
        })
    }

    pub fn decrypt_protected_value(
        &self,
        uuid: &str,
        key: &str,
        entity_index: usize,
        protected_value: &str,
    ) -> Result<String, KdbxDatabaseError> {
        let mut cipher = self.inner_header.get_stream_cipher();

        let combined_key = format!("{}:{}:{}", uuid, key, entity_index);
        let protect_value = self.protect_values.get(&combined_key).ok_or(
            KdbxDatabaseError::ProtectedValueNotFound {
                uuid: uuid.to_string(),
                key: key.to_string(),
                entity_index,
            },
        )?;

        let decoded =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, protected_value)
                .map_err(|_| KdbxDatabaseError::ProtectedValueDecryptError {
                    uuid: uuid.to_string(),
                    key: key.to_string(),
                    entity_index,
                })?;
        let data = cipher
            .decrypt_stream(*protect_value, &decoded)
            .map_err(|_| KdbxDatabaseError::ProtectedValueDecryptError {
                uuid: uuid.to_string(),
                key: key.to_string(),
                entity_index,
            })?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }
}

fn collect_protected_values_document(document: &KeePassDocument) -> HashMap<String, usize> {
    let mut protected_values: HashMap<String, usize> = HashMap::new();
    collect_protected_values_group(&document.root.group, 0, &mut protected_values);
    protected_values
}

fn collect_protected_values_group(
    group: &Group,
    stream_offset: usize,
    values: &mut HashMap<String, usize>,
) -> usize {
    let mut stream_offset = stream_offset;
    for entry in &group.entry {
        stream_offset = collect_protected_values_entry(entry, stream_offset, values);
    }
    for group in &group.group {
        stream_offset = collect_protected_values_group(group, stream_offset, values);
    }
    stream_offset
}

fn collect_protected_values_entry(
    entry: &Entry,
    stream_offset: usize,
    values: &mut HashMap<String, usize>,
) -> usize {
    let mut stream_offset = stream_offset;

    stream_offset = process_protected_values(&entry.string, &entry.uuid, 0, stream_offset, values);
    if let Some(ref history) = entry.history {
        for (index, history_entry) in history.entry.iter().enumerate() {
            stream_offset = process_protected_values(
                &history_entry.string,
                &history_entry.uuid,
                index + 1,
                stream_offset,
                values,
            );
        }
    }
    stream_offset
}

fn process_protected_values(
    string_fields: &[StringField],
    uuid: &str,
    entity_index: usize,
    stream_offset: usize,
    values: &mut HashMap<String, usize>,
) -> usize {
    let mut stream_offset = stream_offset;

    for value in string_fields {
        if value.value.is_protected() {
            let combined_key = format!("{}:{}:{}", uuid, value.key, entity_index);
            values.insert(combined_key, stream_offset);
            let b64_length = utils::b64_original_length(&value.value.value);
            stream_offset += b64_length;
        }
    }

    stream_offset
}
