import 'dart:async';
import 'dart:typed_data';

import 'package:keepass_one/services/file_system/file_system_models.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:keepass_one/services/sync/exceptions.dart';
import 'package:keepass_one/services/sync/sync_driver.dart';
import 'package:webdav_client/webdav_client.dart';
import 'package:dio/dio.dart' show DioException, ProgressCallback;

/// WebDAV同步驱动实现
class WebDavSyncDriver implements SyncDriver, FileSystemProvider {
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
  Future<List<FileSystemEntity>> list(String path) async {
    try {
      final entities = <FileSystemEntity>[];
      final items = await client.readDir(path);
      for (final item in items) {
        final name = _getNameFromPath(item.path ?? '');
        final lastModified = item.mTime;

        final entity = FileSystemEntity(
          name: name,
          path: item.path ?? '',
          isDirectory: item.isDir == true,
          lastModified: lastModified,
          size: item.size,
          extension: item.isDir == true ? null : item.name?.split('.').last,
          isHidden: false,
        );
        entities.add(entity);
      }

      return entities;
    } catch (e) {
      throw _handleWebDavException(e, 'Failed to list directory: $path');
    }
  }

  @override
  Future<Uint8List> get({
    String? path,
    void Function(int count, int total)? onProgress,
  }) async {
    print("config: ${config.toJson()}");
    final bytes = await client.read(path ?? '/', onProgress: onProgress);
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
    final parts = path
        .split('/')
        .where((element) => element.isNotEmpty)
        .toList();
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

  @override
  String getRootPath() => '/';

  @override
  Future<List<FileSystemEntity>> listDirectory(String path) => list(path);

  static Future<Uint8List> getFile(
    WebDavConfig config,
    ProgressCallback? onProgress,
  ) async {
    if (config.filePath == null) {
      throw SyncConfigException('File path is required');
    }
    return WebDavSyncDriver(
      config,
    ).get(path: config.filePath!, onProgress: onProgress);
  }
}
