use std::collections::HashMap;

use crate::{
    kdbx::{
        db::kdbx4::inner_header::Kdbx4InnerHeader,
        xml::entities::{self, KeePassDocument},
    },
    utils,
};

pub struct ProtectValue {
    pub value_index: usize,
    pub stream_offset: usize,
}

pub struct KeePassDatabase {
    pub document: KeePassDocument,
    pub inner_header: Kdbx4InnerHeader,
    pub protect_values: HashMap<String, HashMap<usize, ProtectValue>>,
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
        value_index: usize,
        protected_value: &str,
    ) -> anyhow::Result<String> {
        let mut cipher = self.inner_header.get_stream_cipher();

        let protect_values = self
            .protect_values
            .get(uuid)
            .ok_or(anyhow::anyhow!("Protected value not found"))?;

        let protect_value = protect_values
            .get(&value_index)
            .ok_or(anyhow::anyhow!("Protected value index not found"))?;

        let decoded =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, protected_value)?;
        let data = cipher.decrypt_stream(protect_value.stream_offset, &decoded)?;

        Ok(String::from_utf8(data)?)
    }
}

fn collect_protected_values_document(
    document: &KeePassDocument,
) -> HashMap<String, HashMap<usize, ProtectValue>> {
    let mut protected_values: HashMap<String, HashMap<usize, ProtectValue>> = HashMap::new();
    collect_protected_values_group(&document.root.group, 0, &mut protected_values);
    protected_values
}

fn collect_protected_values_group(
    group: &entities::Group,
    stream_offset: usize,
    values: &mut HashMap<String, HashMap<usize, ProtectValue>>,
) -> usize {
    let mut stream_offset = stream_offset;
    for entry in &group.entry {
        stream_offset += collect_protected_values_entry(entry, stream_offset, values);
    }
    for group in &group.group {
        stream_offset += collect_protected_values_group(group, stream_offset, values);
    }
    stream_offset
}

fn collect_protected_values_entry(
    entry: &entities::Entry,
    entry_offset: usize,
    values: &mut HashMap<String, HashMap<usize, ProtectValue>>,
) -> usize {
    let mut stream_offset = entry_offset;
    for (index, value) in entry.string.iter().enumerate() {
        if let Some(ref protected) = value.value.protected {
            if protected == "True" {
                if let Some(protect_values) = values.get_mut(entry.uuid.as_str()) {
                    protect_values.insert(
                        index,
                        ProtectValue {
                            value_index: index,
                            stream_offset: stream_offset,
                        },
                    );
                } else {
                    let mut protect_values = HashMap::new();
                    protect_values.insert(
                        index,
                        ProtectValue {
                            value_index: index,
                            stream_offset: stream_offset,
                        },
                    );
                    values.insert(entry.uuid.clone(), protect_values);
                }
                stream_offset += utils::b64_original_length(&value.value.value);
            }
        }
    }
    stream_offset
}
