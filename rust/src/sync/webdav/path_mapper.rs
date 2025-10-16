use std::sync::Arc;

/// PathMapper 负责处理 WebDAV 路径的映射和转换
///
/// 主要功能：
/// 1. 规范化 base_url 的路径部分
/// 2. 将服务器返回的绝对路径转换为相对路径
/// 3. 提供用于请求的路径
#[derive(Debug, Clone)]
pub struct PathMapper {
    /// 规范化后的 base_path，统一不带尾部 /（根路径 / 除外）
    /// 例如: "/dav/share" 或 "/"
    base_path: String,
}

impl PathMapper {
    /// 从 base_url 创建 PathMapper
    ///
    /// 规范化规则：
    /// - 提取 URL 的 path 部分
    /// - 移除尾部的 /（除非是根路径 /）
    /// - URL 解码
    pub fn new(base_url: &url::Url) -> Self {
        let path = base_url.path();
        let base_path = Self::normalize_base_path(path);

        Self { base_path }
    }

    /// 规范化 base_path
    fn normalize_base_path(path: &str) -> String {
        // 移除尾部 /，除非是根路径
        if path == "/" {
            "/".to_string()
        } else {
            path.trim_end_matches('/').to_string()
        }
    }

    /// 规范化服务器路径（移除尾部 /）
    fn normalize_server_path(path: &str) -> String {
        if path == "/" {
            "/".to_string()
        } else {
            path.trim_end_matches('/').to_string()
        }
    }

    /// 将服务器返回的绝对路径转换为相对路径
    ///
    /// 例如：
    /// - base_path="/dav/share", server_path="/dav/share/folder" -> "folder"
    /// - base_path="/dav/share", server_path="/dav/share" -> ""
    /// - base_path="/", server_path="/folder" -> "folder"
    pub fn server_path_to_relative(&self, server_path: &str) -> Option<String> {
        let normalized_server = Self::normalize_server_path(server_path);

        // 如果服务器路径就是 base_path，返回空字符串
        if normalized_server == self.base_path {
            return Some(String::new());
        }

        // 检查是否以 base_path 开头
        if self.base_path == "/" {
            // 根路径特殊处理：直接移除前导 /
            Some(normalized_server.trim_start_matches('/').to_string())
        } else {
            // 检查是否以 base_path/ 开头
            let prefix = format!("{}/", self.base_path);
            if normalized_server.starts_with(&prefix) {
                Some(normalized_server[prefix.len()..].to_string())
            } else if normalized_server == self.base_path {
                Some(String::new())
            } else {
                // 路径不匹配
                None
            }
        }
    }


    /// 获取 base_path
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    /// 创建一个 Arc 包装的 PathMapper
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_normalize_base_path() {
        assert_eq!(PathMapper::normalize_base_path("/"), "/");
        assert_eq!(PathMapper::normalize_base_path("/dav/share"), "/dav/share");
        assert_eq!(PathMapper::normalize_base_path("/dav/share/"), "/dav/share");
        assert_eq!(
            PathMapper::normalize_base_path("/dav/share//"),
            "/dav/share"
        );
    }

    #[test]
    fn test_path_mapper_root() {
        let url = Url::parse("http://example.com/").unwrap();
        let mapper = PathMapper::new(&url);

        assert_eq!(mapper.base_path, "/");

        // 测试服务器路径转换
        assert_eq!(mapper.server_path_to_relative("/"), Some("".to_string()));
        assert_eq!(
            mapper.server_path_to_relative("/folder"),
            Some("folder".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/folder/"),
            Some("folder".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/folder/subfolder"),
            Some("folder/subfolder".to_string())
        );
    }

    #[test]
    fn test_path_mapper_with_path() {
        let url = Url::parse("http://example.com/dav/share").unwrap();
        let mapper = PathMapper::new(&url);

        assert_eq!(mapper.base_path, "/dav/share");

        // 测试服务器路径转换
        assert_eq!(
            mapper.server_path_to_relative("/dav/share"),
            Some("".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/"),
            Some("".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/folder"),
            Some("folder".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/folder/"),
            Some("folder".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/folder/file.txt"),
            Some("folder/file.txt".to_string())
        );

        // 不匹配的路径
        assert_eq!(mapper.server_path_to_relative("/other/path"), None);
        assert_eq!(mapper.server_path_to_relative("/dav"), None);
    }

    #[test]
    fn test_path_mapper_with_trailing_slash() {
        let url = Url::parse("http://example.com/dav/share/").unwrap();
        let mapper = PathMapper::new(&url);

        // 应该规范化为不带尾部斜杠
        assert_eq!(mapper.base_path, "/dav/share");

        // 功能应该与不带尾部斜杠的版本一致
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/folder"),
            Some("folder".to_string())
        );
    }

    #[test]
    fn test_path_mapper_with_file() {
        let url = Url::parse("http://example.com/dav/share/file.apk").unwrap();
        let mapper = PathMapper::new(&url);

        assert_eq!(mapper.base_path, "/dav/share/file.apk");

        // 文件本身
        assert_eq!(
            mapper.server_path_to_relative("/dav/share/file.apk"),
            Some("".to_string())
        );

        // 不应该匹配其他文件
        assert_eq!(mapper.server_path_to_relative("/dav/share/other.apk"), None);
    }

    #[test]
    fn test_path_mapper_complex_paths() {
        let url = Url::parse("http://example.com/webdav/backup/keepass").unwrap();
        let mapper = PathMapper::new(&url);

        assert_eq!(mapper.base_path, "/webdav/backup/keepass");

        assert_eq!(
            mapper.server_path_to_relative("/webdav/backup/keepass/2024/database.kdbx"),
            Some("2024/database.kdbx".to_string())
        );
        assert_eq!(
            mapper.server_path_to_relative("/webdav/backup/keepass/"),
            Some("".to_string())
        );
    }

    #[test]
    fn test_url_encoding() {
        // 测试包含特殊字符的路径
        // url::Url 的 path() 方法保持编码不变，这是正确的行为
        // 因为服务器返回的路径也会是编码后的格式
        let url = Url::parse("http://example.com/my%20folder/sub").unwrap();
        let mapper = PathMapper::new(&url);

        // 路径保持编码格式
        assert_eq!(mapper.base_path, "/my%20folder/sub");

        // 测试服务器返回编码路径的情况
        assert_eq!(
            mapper.server_path_to_relative("/my%20folder/sub/file.txt"),
            Some("file.txt".to_string())
        );
    }
}
