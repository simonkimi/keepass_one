import 'dart:async';
import 'dart:typed_data';
import 'package:keepass_one/services/file_system/file_system_models.dart';

abstract class SyncDriver {
  Future<List<FileSystemEntity>> list(String path);

  Future<Uint8List> get(
    String path, {
    void Function(int count, int total)? onProgress,
  });

  Future<void> put(
    String path,
    Uint8List data, {
    void Function(int count, int total)? onProgress,
  });

  Future<void> ping() async {}

  /// 驱动名称（用于日志和调试）
  String get driverName;

  /// 关闭驱动并释放资源
  Future<void> close() async {}
}
