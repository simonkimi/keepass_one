import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:keepass_one/services/sync/exceptions.dart';

part 'webdav_config.freezed.dart';
part 'webdav_config.g.dart';

/// WebDAV配置
@freezed
sealed class WebDavConfig with _$WebDavConfig {
  static const String kDriverType = 'webdav';

  const WebDavConfig._();

  /// 创建WebDAV配置
  const factory WebDavConfig({
    /// 服务器基础URL
    required String baseUrl,

    /// 用户名
    required String username,

    /// 密码
    required String password,

    /// 是否跳过TLS证书验证
    required bool tlsInsecureSkipVerify,
  }) = _WebDavConfig;

  /// 从JSON映射创建
  factory WebDavConfig.fromJson(Map<String, dynamic> json) =>
      _$WebDavConfigFromJson(json);

  void validate() {
    if (baseUrl.isEmpty) {
      throw SyncConfigException('WebDAV base URL cannot be empty');
    }

    // 确保URL格式正确
    try {
      final uri = Uri.parse(baseUrl);
      if (!uri.isAbsolute ||
          (!uri.scheme.startsWith('http') && !uri.scheme.startsWith('https'))) {
        throw SyncConfigException(
          'Invalid WebDAV URL',
          details: 'URL must start with http:// or https://',
        );
      }
    } catch (e) {
      throw SyncConfigException(
        'Invalid WebDAV URL format',
        details: e.toString(),
        originalError: e,
      );
    }
  }
}
