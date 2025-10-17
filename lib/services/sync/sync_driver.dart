import 'dart:async';
import 'dart:typed_data';

import 'package:keepass_one/services/sync/models.dart';

/// 同步驱动接口
///
/// 此接口定义了所有同步驱动必须实现的方法，用于抹平不同驱动之间的差异
abstract class SyncDriver {
  /// 列出指定路径下的所有文件和目录
  ///
  /// [path] 目录路径，如 "/" 或 "/configs/"
  /// 返回包含文件列表和目录列表的结果
  ///
  /// 可能抛出: SyncConnectionException, SyncAuthenticationException,
  ///          SyncNotFoundException, SyncPermissionException,
  ///          SyncTimeoutException, SyncIOException
  Future<SyncListResult> list(String path);

  /// 读取指定路径的文件内容（流式）
  ///
  /// [path] 文件路径，如 "/config.json"
  /// 返回字节流
  ///
  /// 可能抛出: SyncConnectionException, SyncAuthenticationException,
  ///          SyncNotFoundException, SyncPermissionException,
  ///          SyncTimeoutException, SyncIOException
  Future<Uint8List> get(
    String path, {
    void Function(int count, int total)? onProgress,
  });

  /// 写入文件内容到指定路径（流式）
  ///
  /// [path] 文件路径，如 "/config.json"
  /// [data] 字节流
  /// 如果文件不存在会创建，如果存在会覆盖
  ///
  /// 可能抛出: SyncConnectionException, SyncAuthenticationException,
  ///          SyncPermissionException, SyncTimeoutException, SyncIOException
  Future<void> put(
    String path,
    Uint8List data, {
    void Function(int count, int total)? onProgress,
  });

  /// 测试连接是否正常（可选方法）
  ///
  /// 可能抛出: SyncConnectionException, SyncAuthenticationException,
  ///          SyncTimeoutException
  Future<void> ping() async {}

  /// 驱动名称（用于日志和调试）
  String get driverName;

  /// 关闭驱动并释放资源
  Future<void> close() async {}
}
