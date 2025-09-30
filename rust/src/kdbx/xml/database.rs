use std::collections::HashMap;

use crate::{
    kdbx::{
        db::kdbx4::inner_header::Kdbx4InnerHeader,
        xml::entities::{self, KeePassDocument},
    },
    utils,
};

pub struct KeePassDatabase {
    pub document: KeePassDocument,
    pub inner_header: Kdbx4InnerHeader,
    pub protect_values: HashMap<String, usize>,
}

impl KeePassDatabase {
    pub fn new(document: KeePassDocument, inner_header: Kdbx4InnerHeader) -> Self {
        let protected_values = collect_protected_values_document(&document);
        Self {
            document,
            inner_header,
            protect_values: protected_values,
        }
    }

    pub fn decrypt_protected_value(
        &self,
        uuid: &str,
        key: &str,
        entity_index: usize,
        protected_value: &str,
    ) -> anyhow::Result<String> {
        let mut cipher = self.inner_header.get_stream_cipher();

        let combined_key = format!("{}:{}:{}", uuid, key, entity_index);
        let protect_value = self
            .protect_values
            .get(&combined_key)
            .ok_or(anyhow::anyhow!("Protected value not found"))?;

        let decoded =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, protected_value)?;
        let data = cipher.decrypt_stream(*protect_value, &decoded)?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }
}

fn collect_protected_values_document(document: &KeePassDocument) -> HashMap<String, usize> {
    let mut protected_values: HashMap<String, usize> = HashMap::new();
    collect_protected_values_group(&document.root.group, 0, &mut protected_values);
    protected_values
}

fn collect_protected_values_group(
    group: &entities::Group,
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
    entry: &entities::Entry,
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
    string_fields: &[entities::StringField],
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
