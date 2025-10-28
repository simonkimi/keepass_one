import 'package:flutter/foundation.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';

class FilePickerProvider extends ChangeNotifier {
  FilePickerProvider({required this.fileSystemProvider});
  final FileSystemProvider fileSystemProvider;

  final List<String> _fileNamePath = [];

  String? get name => _fileNamePath.isEmpty ? null : _fileNamePath.last;

  void pushPath(String path) {
    _fileNamePath.add(path);
    notifyListeners();
  }

  void popPath() {
    if (_fileNamePath.isNotEmpty) {
      _fileNamePath.removeLast();
    }
    notifyListeners();
  }
}
