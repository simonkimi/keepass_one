pub mod webdav;

use std::{future::Future, sync::Arc};

use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::Stream;

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Init error: {0}")]
    InitError(String),

    #[error("Auth error: {0}")]
    AuthError(String),

    #[error("Not found error: {0}")]
    NotFoundError(String),

    #[error("Forbidden error: {0}")]
    ForbiddenError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Client error: {0}")]
    ClientError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("File already exists: {0}")]
    FileExistsError(String),

    #[error("Invalid path: {0}")]
    InvalidPathError(String),

    #[error("IO error: {0}")]
    IoError(String),
}

pub trait SyncFileObj: std::fmt::Debug {
    /// 文件名
    fn name(&self) -> &str;
    /// 文件路径
    fn path(&self) -> &str;
    /// 文件大小
    fn size(&self) -> usize;
    /// 最后修改时间
    fn last_modified(&self) -> DateTime<Utc>;

    /// 文件路径
    fn relative_path(&self) -> Option<String>;
}

pub trait SyncFolderObj: std::fmt::Debug {
    /// 文件名
    fn name(&self) -> &str;
    /// 文件路径
    fn path(&self) -> &str;
    /// 最后修改时间
    fn last_modified(&self) -> DateTime<Utc>;

    /// 文件路径
    fn relative_path(&self) -> Option<String>;
}

#[derive(Debug)]
pub enum SyncObject {
    File(Box<dyn SyncFileObj>),
    Folder(Box<dyn SyncFolderObj>),
}

pub trait SyncDriver {
    fn list(&self, dir: &str) -> impl Future<Output = Result<Vec<SyncObject>, SyncError>>;

    fn get(
        &self,
        path: &str,
    ) -> impl Future<
        Output = Result<
            std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, SyncError>> + Send>>,
            SyncError,
        >,
    >;
    fn put(
        &self,
        path: &str,
        data: impl Stream<Item = Result<Bytes, SyncError>> + Send + 'static,
    ) -> impl Future<Output = Result<(), SyncError>>;
}
