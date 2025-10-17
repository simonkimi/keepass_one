import 'dart:convert';

import 'package:keepass_one/services/sync/exceptions.dart';
import 'package:keepass_one/services/sync/sync_driver.dart';
import 'package:keepass_one/services/sync/webdav/webdav.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';

/// 同步驱动工厂类
///
/// 用于根据配置创建不同类型的同步驱动实例
class SyncDriverFactory {
  /// 根据驱动类型和JSON配置创建驱动实例
  ///
  /// [driverType] 驱动类型标识: 'webdav', 'onedrive', 'sftp', 's3'
  /// [configJson] JSON格式的配置字符串
  ///
  /// 返回对应的驱动实例
  ///
  /// 抛出 ArgumentError 如果驱动类型未知
  /// 抛出 FormatException 如果JSON格式错误
  /// 抛出 SyncException 如果配置参数不完整或错误
  static Future<SyncDriver> create(String driverType, String configJson) async {
    try {
      SyncDriver driver;
      switch (driverType) {
        case WebDavSyncDriver.kDriverName:
          final config = WebDavConfig.fromJson(jsonDecode(configJson));
          driver = WebDavSyncDriver(config);
          break;

        default:
          throw ArgumentError('Unknown driver type: $driverType');
      }
      return driver;
    } on FormatException catch (e) {
      throw SyncIOException(
        'Invalid JSON configuration',
        details: e.message,
        originalError: e,
      );
    } catch (e) {
      if (e is SyncException) rethrow;
      throw SyncIOException(
        'Failed to create driver',
        details: e.toString(),
        originalError: e,
      );
    }
  }
}
