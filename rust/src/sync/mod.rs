pub mod webdav;

use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Init error: {0}")]
    InitError(String),

    #[error("Auth error: {0}")]
    AuthError(String),

    #[error("Not found error: {0}")]
    NotFoundError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
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
    async fn root(&self) -> Result<Vec<SyncObject>, SyncError>;
    async fn list(&self, dir: &dyn SyncFolderObj) -> Result<Vec<SyncObject>, SyncError>;
}
