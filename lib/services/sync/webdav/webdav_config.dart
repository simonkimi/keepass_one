import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/utils/url_utils.dart';

part 'webdav_config.freezed.dart';
part 'webdav_config.g.dart';

/// WebDAV配置
@freezed
sealed class WebDavConfig with _$WebDavConfig implements BaseDriverConfig {
  const WebDavConfig._();

  @override
  DriverType get type => DriverType.webdav;

  @override
  String get name => filePath?.split('/').last ?? '';

  @override
  String get description =>
      filePath != null ? UrlUtils.join(baseUrl, filePath) : baseUrl;

  /// 创建WebDAV配置
  const factory WebDavConfig({
    required String baseUrl,

    /// 用户名
    required String username,

    /// 密码
    required String password,

    /// 是否跳过TLS证书验证
    required bool tlsInsecureSkipVerify,

    /// 文件路径
    String? filePath,
  }) = _WebDavConfig;

  /// 从JSON映射创建
  factory WebDavConfig.fromJson(Map<String, dynamic> json) =>
      _$WebDavConfigFromJson(json);
}
