import 'dart:async';
import 'dart:typed_data';

import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:keepass_one/services/sync/exceptions.dart';
import 'package:keepass_one/services/sync/models.dart';
import 'package:keepass_one/services/sync/sync_driver.dart';
import 'package:webdav_client/webdav_client.dart';
import 'package:dio/dio.dart' show DioException;

/// WebDAV同步驱动实现
class WebDavSyncDriver implements SyncDriver {
  static const String kDriverName = 'WebDAV';

  /// WebDAV配置
  final WebDavConfig config;

  /// WebDAV客户端
  final Client client;

  /// 构造函数
  WebDavSyncDriver(this.config)
    : client = newClient(
        config.baseUrl,
        user: config.username,
        password: config.password,
      );

  @override
  String get driverName => WebDavSyncDriver.kDriverName;

  @override
  Future<SyncListResult> list(String path) async {
    try {
      final items = await client.readDir(path);

      final files = <SyncFile>[];
      final directories = <SyncDirectory>[];

      for (final item in items) {
        final name = _getNameFromPath(item.path ?? '');
        final lastModified = item.mTime;

        if (item.isDir == true) {
          directories.add(
            SyncDirectory(
              name: name,
              path: item.path ?? '',
              lastModified: lastModified,
            ),
          );
        } else {
          files.add(
            SyncFile(
              name: name,
              path: item.path ?? '',
              lastModified: lastModified,
              size: item.size,
              hash: item.eTag,
              mimeType: item.mimeType,
            ),
          );
        }
      }

      return SyncListResult(files: files, directories: directories);
    } catch (e) {
      throw _handleWebDavException(e, 'Failed to list directory: $path');
    }
  }

  @override
  Future<Uint8List> get(
    String path, {
    void Function(int count, int total)? onProgress,
  }) async {
    final bytes = await client.read(path, onProgress: onProgress);
    return Uint8List.fromList(bytes);
  }

  @override
  Future<void> put(
    String path,
    Uint8List data, {
    void Function(int count, int total)? onProgress,
  }) async {
    try {
      await client.write(path, data, onProgress: onProgress);
    } catch (e) {
      throw _handleWebDavException(e, 'Failed to write file: $path');
    }
  }

  @override
  Future<void> ping() async {
    try {
      await client.ping();
    } catch (e) {
      throw _handleWebDavException(e, 'Failed to connect to WebDAV server');
    }
  }

  @override
  Future<void> close() async {
    // WebDAV客户端没有显式的close方法，但如果有需要清理的资源，可以在这里添加
  }

  /// 从路径中提取文件名
  String _getNameFromPath(String path) {
    final parts = path.split('/').where((element) => element.isNotEmpty).toList();
    return parts.isNotEmpty ? parts.last : path;
  }

  /// 处理WebDAV异常，转换为统一的异常类型
  SyncException _handleWebDavException(dynamic e, String message) {
    if (e is SyncException) return e;

    // 根据错误类型转换为对应的异常
    if (e is DioException) {
      final statusCode = e.response?.statusCode;
      final details = e.message;

      if (statusCode == 401 || statusCode == 403) {
        return SyncAuthenticationException(
          message,
          details: details,
          originalError: e,
        );
      } else if (statusCode == 404) {
        return SyncNotFoundException(
          message,
          details: details,
          originalError: e,
        );
      } else if (statusCode == 408 || statusCode == 504) {
        return SyncTimeoutException(
          message,
          details: details,
          originalError: e,
        );
      } else {
        return SyncIOException(
          message,
          details: 'Status code: $statusCode, $details',
          originalError: e,
        );
      }
    } else if (e is TimeoutException) {
      return SyncTimeoutException(
        message,
        details: e.toString(),
        originalError: e,
      );
    } else if (e.toString().contains('SocketException') ||
        e.toString().contains('Connection refused')) {
      return SyncConnectionException(
        message,
        details: e.toString(),
        originalError: e,
      );
    }

    // 默认为IO异常
    return SyncIOException(message, details: e.toString(), originalError: e);
  }
}
