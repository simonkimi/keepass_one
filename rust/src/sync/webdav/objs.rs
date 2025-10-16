use reqwest_dav::list_cmd::{ListFile, ListFolder};
use std::sync::Arc;

use crate::sync::{SyncFileObj, SyncFolderObj};

use super::path_mapper::PathMapper;

#[derive(Debug)]
pub struct WebDavListFile {
    file: ListFile,
    path_mapper: Arc<PathMapper>,
}

impl WebDavListFile {
    pub fn new(file: ListFile, path_mapper: Arc<PathMapper>) -> Self {
        Self { file, path_mapper }
    }
}

impl SyncFileObj for WebDavListFile {
    fn name(&self) -> &str {
        get_file_name(&self.file.href)
    }

    fn size(&self) -> usize {
        self.file.content_length as usize
    }

    fn last_modified(&self) -> chrono::DateTime<chrono::Utc> {
        self.file.last_modified
    }

    fn relative_path(&self) -> Option<String> {
        self.path_mapper.server_path_to_relative(&self.file.href)
    }

    fn path(&self) -> &str {
        &self.file.href
    }
}

#[derive(Debug)]
pub struct WebDavListFolder {
    folder: ListFolder,
    path_mapper: Arc<PathMapper>,
}

impl WebDavListFolder {
    pub fn new(folder: ListFolder, path_mapper: Arc<PathMapper>) -> Self {
        Self { folder, path_mapper }
    }
}

impl SyncFolderObj for WebDavListFolder {
    fn name(&self) -> &str {
        get_file_name(&self.folder.href)
    }

    fn last_modified(&self) -> chrono::DateTime<chrono::Utc> {
        self.folder.last_modified
    }

    fn relative_path(&self) -> Option<String> {
        self.path_mapper.server_path_to_relative(&self.folder.href)
    }

    fn path(&self) -> &str {
        &self.folder.href
    }
}

fn get_file_name(href: &str) -> &str {
    href.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(href)
}
