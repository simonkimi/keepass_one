import 'package:drift/drift.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:keepass_one/services/database/entity.dart';
import 'package:keepass_one/services/database/dao.dart';
import 'package:path_provider/path_provider.dart';

part 'database.g.dart';

@DriftDatabase(
  tables: [KdbxFile, KdbxKeyFile, KdbxFileBackup],
  daos: [KdbxFileDao, KdbxKeyFileDao, KdbxFileBackupDao],
)
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
