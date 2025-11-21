import 'dart:io';
import 'dart:typed_data';

import 'package:dio/dio.dart';
import 'package:keepass_one/services/sync/exceptions.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';

class LocalDriver {
  static Future<Uint8List> getFile(
    LocalConfig config,
    ProgressCallback? onProgress,
  ) async {
    try {
      final file = File(config.path);
      final bytes = await file.readAsBytes();
      if (onProgress != null) {
        onProgress(bytes.length, bytes.length);
      }
      return bytes;
    } catch (e) {
      throw SyncIOException(
        'Failed to read local file',
        details: e.toString(),
        originalError: e,
      );
    }
  }
}
