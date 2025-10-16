mod objs;
mod path_mapper;

use reqwest_dav::{list_cmd::ListEntity, Auth, Depth};
use std::sync::Arc;

use crate::sync::{
    webdav::objs::{WebDavListFile, WebDavListFolder},
    SyncDriver, SyncError, SyncFolderObj, SyncObject,
};

use path_mapper::PathMapper;

struct WebDavConfig {
    username: String,
    password: String,
    base_url: url::Url,
    tls_insecure_skip_verify: bool,
}

impl WebDavConfig {
    pub fn new(
        base_url: String,
        username: String,
        password: String,
        tls_insecure_skip_verify: bool,
    ) -> anyhow::Result<Self> {
        let base_url = url::Url::parse(&base_url)?;
        Ok(Self {
            base_url,
            username,
            password,
            tls_insecure_skip_verify,
        })
    }
}

struct WebDav {
    config: WebDavConfig,
    client: reqwest_dav::Client,
    path_mapper: Arc<PathMapper>,
}

impl WebDav {
    pub fn new(config: WebDavConfig) -> Result<WebDav, SyncError> {
        let mut agent_builder = reqwest::Client::builder();

        if config.tls_insecure_skip_verify {
            agent_builder = agent_builder.danger_accept_invalid_certs(true);
        }

        let agent = agent_builder.build().unwrap();

        let client = reqwest_dav::ClientBuilder::new()
            .set_host(config.base_url.as_str().to_string())
            .set_auth(if config.username.is_empty() {
                Auth::Anonymous
            } else {
                Auth::Basic(config.username.clone(), config.password.clone())
            })
            .set_agent(agent)
            .build()
            .map_err(|e| SyncError::InitError(e.to_string()))?;

        let path_mapper = PathMapper::new(&config.base_url).into_arc();

        Ok(WebDav {
            config,
            client,
            path_mapper,
        })
    }

    async fn list_path(&self, path: &str) -> Result<Vec<SyncObject>, SyncError> {
        let items = self
            .client
            .list(path, Depth::Number(1))
            .await
            .map_err(get_webdav_err)?;

        let request_path = if path.is_empty() {
            self.path_mapper.base_path().to_string()
        } else {
            let base_path = self.path_mapper.base_path();
            if base_path == "/" {
                format!("/{}", path)
            } else {
                format!("{}/{}", base_path, path)
            }
        };

        Ok(items
            .into_iter()
            .filter_map(|item| {
                let href = match &item {
                    ListEntity::File(file) => &file.href,
                    ListEntity::Folder(folder) => &folder.href,
                };

                if href == &request_path || href == &format!("{}/", request_path) {
                    return None;
                }

                Some(match item {
                    ListEntity::File(file) => SyncObject::File(Box::new(WebDavListFile::new(
                        file,
                        Arc::clone(&self.path_mapper),
                    ))),
                    ListEntity::Folder(dir) => SyncObject::Folder(Box::new(WebDavListFolder::new(
                        dir,
                        Arc::clone(&self.path_mapper),
                    ))),
                })
            })
            .collect())
    }
}

impl SyncDriver for WebDav {
    async fn root(&self) -> Result<Vec<SyncObject>, SyncError> {
        self.list_path("").await
    }

    async fn list(&self, dir: &dyn SyncFolderObj) -> Result<Vec<SyncObject>, SyncError> {
        let path = dir.relative_path().ok_or(SyncError::NotFoundError(format!(
            "Path not found: {}",
            dir.path()
        )))?;
        self.list_path(&path).await
    }
}

fn get_webdav_err(err: reqwest_dav::Error) -> SyncError {
    match err {
        reqwest_dav::Error::Reqwest(e) => {
            // 处理网络相关错误
            if e.is_timeout() {
                SyncError::TimeoutError(format!("Request timeout: {}", e))
            } else if e.is_connect() {
                SyncError::NetworkError(format!("Connection failed: {}", e))
            } else if e.is_request() {
                SyncError::ClientError(format!("Request error: {}", e))
            } else {
                SyncError::NetworkError(format!("Network error: {}", e))
            }
        }
        reqwest_dav::Error::ReqwestDecode(e) => {
            SyncError::ClientError(format!("Response decode error: {:?}", e))
        }
        reqwest_dav::Error::Decode(e) => match e {
            reqwest_dav::DecodeError::StatusMismatched(e) => {
                let status_code = e.response_code;
                let error_message = format!(
                    "HTTP {}: {}",
                    status_code,
                    get_http_status_message(status_code)
                );

                match status_code {
                    // 客户端错误 (4xx)
                    400 => SyncError::ClientError(format!("Bad request: {}", error_message)),
                    401 => SyncError::AuthError(format!("Unauthorized: {}", error_message)),
                    403 => SyncError::ForbiddenError(format!("Forbidden: {}", error_message)),
                    404 => SyncError::NotFoundError(format!("Not found: {}", error_message)),
                    405 => SyncError::ClientError(format!("Method not allowed: {}", error_message)),
                    409 => SyncError::ClientError(format!("Conflict: {}", error_message)),
                    412 => {
                        SyncError::ClientError(format!("Precondition failed: {}", error_message))
                    }
                    413 => SyncError::ClientError(format!(
                        "Request entity too large: {}",
                        error_message
                    )),
                    415 => {
                        SyncError::ClientError(format!("Unsupported media type: {}", error_message))
                    }
                    423 => SyncError::ClientError(format!("Resource locked: {}", error_message)),

                    // 服务器错误 (5xx)
                    500 => {
                        SyncError::ServerError(format!("Internal server error: {}", error_message))
                    }
                    501 => SyncError::ServerError(format!("Not implemented: {}", error_message)),
                    502 => SyncError::ServerError(format!("Bad gateway: {}", error_message)),
                    503 => {
                        SyncError::ServerError(format!("Service unavailable: {}", error_message))
                    }
                    504 => SyncError::ServerError(format!("Gateway timeout: {}", error_message)),
                    505 => SyncError::ServerError(format!(
                        "HTTP version not supported: {}",
                        error_message
                    )),

                    // 其他状态码
                    _ if status_code >= 400 && status_code < 500 => SyncError::ClientError(
                        format!("Client error {}: {}", status_code, error_message),
                    ),
                    _ if status_code >= 500 => SyncError::ServerError(format!(
                        "Server error {}: {}",
                        status_code, error_message
                    )),
                    _ => SyncError::UnknownError(format!(
                        "Unexpected status code {}: {}",
                        status_code, error_message
                    )),
                }
            }
            _ => SyncError::ClientError(format!("Decode error: {}", e)),
        },
        reqwest_dav::Error::MissingAuthContext => {
            SyncError::InitError("Missing authentication context".to_string())
        }
    }
}

/// 获取 HTTP 状态码对应的描述信息
fn get_http_status_message(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        412 => "Precondition Failed",
        413 => "Request Entity Too Large",
        415 => "Unsupported Media Type",
        423 => "Locked",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        _ => "Unknown Status",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webdav() -> anyhow::Result<()> {
        dotenv::dotenv().expect("Failed to load .env file");
        env_logger::init();

        let base_url = std::env::var("WEBDAV_BASE_URL").expect("WEBDAV_BASE_URL not found");
        let username = std::env::var("WEBDAV_USERNAME").expect("WEBDAV_USERNAME not found");
        let password = std::env::var("WEBDAV_PASSWORD").expect("WEBDAV_PASSWORD not found");
        let tls_insecure_skip_verify = true;

        let config =
            WebDavConfig::new(base_url, username, password, tls_insecure_skip_verify).unwrap();
        let webdav = WebDav::new(config)?;
        let list = webdav.root().await?;
        for item in list.into_iter() {
            match item {
                SyncObject::File(file) => println!("File: {}", file.path()),
                SyncObject::Folder(folder) => {
                    println!("Folder: {}", folder.path());
                    walk_webdav(&webdav, &*folder).await;
                }
            }
        }
        Ok(())
    }

    async fn walk_webdav(webdav: &WebDav, dir: &dyn SyncFolderObj) {
        let list = webdav.list(dir).await.expect(&format!(
            "Failed to list folder: {}",
            dir.relative_path().expect("Path not found")
        ));
        for item in list.into_iter() {
            match item {
                SyncObject::File(file) => println!("File: {}", file.path()),
                SyncObject::Folder(folder) => {
                    Box::pin(walk_webdav(webdav, &*folder)).await;
                }
            }
        }
    }
}
