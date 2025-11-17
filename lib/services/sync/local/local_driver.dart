import 'dart:io';
import 'dart:typed_data';

import 'package:keepass_one/services/sync/exceptions.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';

class LocalDriver {
  static Future<Uint8List> getFile(LocalConfig config) async {
    try {
      final file = File(config.path);
      return file.readAsBytes();
    } catch (e) {
      throw SyncIOException(
        'Failed to read local file',
        details: e.toString(),
        originalError: e,
      );
    }
  }
}
