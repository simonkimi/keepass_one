use crate::kdbx::xml::entities::meta::Meta;
use crate::kdbx::xml::entities::root::Root;
use serde::{Deserialize, Serialize};

/// KDBX 4.1 XML Schema.
///
/// Copyright (C) 2007-2025 Dominik Reichl.
/// <https://keepass.info/help/kb/kdbx.html>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeePassDocument {
    #[serde(rename = "Meta")]
    pub meta: Meta,
    #[serde(rename = "Root")]
    pub root: Root,
}
