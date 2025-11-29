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

  const factory WebDavConfig.basic({
    required String baseUrl,
    required String username,
    required String password,
    required bool tlsInsecureSkipVerify,
    String? filePath,
  }) = WebDavConfigBasic;

  const factory WebDavConfig.token({
    required String baseUrl,
    required String token,
    required bool tlsInsecureSkipVerify,
    String? filePath,
  }) = WebDavConfigToken;

  const factory WebDavConfig.none({
    required String baseUrl,
    required bool tlsInsecureSkipVerify,
    String? filePath,
  }) = WebDavConfigNone;

  /// 从JSON映射创建
  factory WebDavConfig.fromJson(Map<String, dynamic> json) =>
      _$WebDavConfigFromJson(json);
}
