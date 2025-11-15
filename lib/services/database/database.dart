import 'package:drift/drift.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:keepass_one/services/database/kdbx_file.dart';
import 'package:keepass_one/services/database/kdbx_dao.dart';
import 'package:keepass_one/services/database/kdbx_key_file.dart';
import 'package:path_provider/path_provider.dart';

part 'database.g.dart';

@DriftDatabase(tables: [KdbxFile, KdbxKeyFile], daos: [KdbxItemDao])
class AppDatabase extends _$AppDatabase {
  AppDatabase([QueryExecutor? executor]) : super(executor ?? _openConnection());

  @override
  int get schemaVersion => 1;

  static QueryExecutor _openConnection() {
    return driftDatabase(
      name: 'keepass_one.db',
      native: const DriftNativeOptions(
        databaseDirectory: getApplicationSupportDirectory,
      ),
    );
  }
}
