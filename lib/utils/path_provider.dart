import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

Future<String> createKdbxBackupPath(int configId) async {
  final baseDir = await getApplicationDocumentsDirectory();
  final filename = DateTime.now().millisecondsSinceEpoch.toString();
  return p.join(baseDir.path, 'kdbx_backup', '$configId', '$filename.kdbx');
}
