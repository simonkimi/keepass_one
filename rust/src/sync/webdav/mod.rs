mod objs;

use reqwest_dav::{list_cmd::ListEntity, Auth, Depth};

use crate::sync::{
    webdav::objs::{WebDavListFile, WebDavListFolder},
    SyncDriver, SyncError, SyncFolderObj, SyncObject,
};

struct WebDavConfig {
    username: String,
    password: String,
    base_url: url::Url,
    tls_insecure_skip_verify: bool,
}

struct WebDav {
    config: WebDavConfig,
    client: reqwest_dav::Client,
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

        Ok(WebDav { config, client })
    }

    async fn list_path(&self, path: &str) -> Result<Vec<SyncObject>, SyncError> {
        Ok(self
            .client
            .list(path, Depth::Number(1))
            .await
            .map_err(get_webdav_err)?
            .into_iter()
            .map(|item| match item {
                ListEntity::File(file) => SyncObject::File(Box::new(WebDavListFile::new(
                    file,
                    self.config.base_url.clone(),
                ))),
                ListEntity::Folder(dir) => SyncObject::Folder(Box::new(WebDavListFolder::new(
                    dir,
                    self.config.base_url.clone(),
                ))),
            })
            .collect())
    }
}

impl SyncDriver for WebDav {
    async fn root(&self) -> Result<Vec<SyncObject>, SyncError> {
        self.list_path("/").await
    }

    async fn list(&self, dir: &dyn SyncFolderObj) -> Result<Vec<SyncObject>, SyncError> {
        let path = dir
            .relative_path()
            .ok_or(SyncError::NotFoundError("Path not found".to_string()))?;
        self.list_path(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn test_webdav() -> anyhow::Result<()> {
        dotenv::dotenv().unwrap();

        let base_url = std::env::var("WEBDAV_BASE_URL").unwrap();
        let username = std::env::var("WEBDAV_USERNAME").unwrap();
        let password = std::env::var("WEBDAV_PASSWORD").unwrap();
        let tls_insecure_skip_verify = std::env::var("WEBDAV_TLS_INSECURE_SKIP_VERIFY")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        let config = WebDavConfig {
            base_url: Url::parse(&base_url)?,
            username,
            password,
            tls_insecure_skip_verify,
        };

        let webdav = WebDav::new(config).unwrap();
        let list = webdav.list_path("/").await?;
        for item in list.into_iter().skip(1) {
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
        let list = webdav.list(dir).await.unwrap();
        for item in list.into_iter().skip(1) {
            match item {
                SyncObject::File(file) => println!("File: {}", file.path()),
                SyncObject::Folder(folder) => {
                    Box::pin(walk_webdav(webdav, &*folder)).await;
                }
            }
        }
    }

    #[test]
    fn test_url_parser() -> anyhow::Result<()> {
        let base = "https://webdav.z31.ink:20443/webdav/backup/";
        let base_url = Url::parse(base)?;
        let server_path = "/webdav/backup/clash-verge-rev-backup";
        let target_url = base_url.join(server_path)?;
        let relative = base_url
            .make_relative(&target_url)
            .ok_or(anyhow::anyhow!("Failed to make relative"))?;

        println!("{}", target_url);
        println!("{}", relative);
        Ok(())
    }
}

fn get_webdav_err(err: reqwest_dav::Error) -> SyncError {
    match err {
        reqwest_dav::Error::Reqwest(e) => SyncError::InitError(e.to_string()),
        reqwest_dav::Error::ReqwestDecode(e) => SyncError::UnknownError("".to_string()),
        reqwest_dav::Error::Decode(e) => match e {
            reqwest_dav::DecodeError::StatusMismatched(e) => match e.response_code {
                401 => SyncError::AuthError("Unauthorized".to_string()),
                404 => SyncError::NotFoundError("Not found".to_string()),
                _ => SyncError::UnknownError("Unknown error".to_string()),
            },
            _ => SyncError::UnknownError("Unknown error".to_string()),
        },
        reqwest_dav::Error::MissingAuthContext => {
            SyncError::InitError("Missing auth context".to_string())
        }
    }
}
