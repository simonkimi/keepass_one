use reqwest_dav::list_cmd::{ListFile, ListFolder};

use crate::sync::{SyncFileObj, SyncFolderObj};

#[derive(Debug)]
pub struct WebDavListFile {
    file: ListFile,
    base_url: url::Url,
}

impl WebDavListFile {
    pub fn new(file: ListFile, base_url: url::Url) -> Self {
        Self { file, base_url }
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
        let server_path = self.base_url.join(&self.file.href).ok()?;
        self.base_url.make_relative(&server_path)
    }

    fn path(&self) -> &str {
        &self.file.href
    }
}

#[derive(Debug)]
pub struct WebDavListFolder {
    folder: ListFolder,
    base_url: url::Url,
}

impl WebDavListFolder {
    pub fn new(folder: ListFolder, base_url: url::Url) -> Self {
        Self { folder, base_url }
    }

    pub fn get_path(&self, base_url: &url::Url) -> Option<String> {
        let server_path = base_url.join(&self.folder.href).ok()?;
        base_url.make_relative(&server_path)
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
        let server_path = self.base_url.join(&self.folder.href).ok()?;
        self.base_url.make_relative(&server_path)
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
