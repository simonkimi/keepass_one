use thiserror::Error;

#[derive(Debug, Error)]

pub enum KdbxDatabaseError {
    #[error("XML parse error")]
    XmlParseError(#[from] quick_xml::DeError),

    #[error("Protected value not found: {uuid}, {key}, {entity_index}")]
    ProtectedValueNotFound {
        uuid: String,
        key: String,
        entity_index: usize,
    },

    #[error("Protected value decrypt error: {uuid}, {key}, {entity_index}")]
    ProtectedValueDecryptError {
        uuid: String,
        key: String,
        entity_index: usize,
    },
}
