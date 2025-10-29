import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:keepass_one/services/sync/driver_config.dart';

part 'webdav_config.freezed.dart';
part 'webdav_config.g.dart';

/// WebDAV配置
@freezed
sealed class WebDavConfig with _$WebDavConfig implements BaseDriverConfig {
  static const String kDriverType = 'webdav';

  const WebDavConfig._();

  /// 创建WebDAV配置
  const factory WebDavConfig({
    required String url,

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
}
