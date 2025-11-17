import 'package:drift/drift.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/database/kdbx_file.dart';
import 'package:keepass_one/services/database/kdbx_key_file.dart';

part 'kdbx_dao.g.dart';

@DriftAccessor(tables: [KdbxFile])
class KdbxFileDao extends DatabaseAccessor<AppDatabase>
    with _$KdbxFileDaoMixin {
  KdbxFileDao(super.db);

  Stream<List<KdbxFileData>> watchAllKdbxItems() => select(kdbxFile).watch();

  Future<int> createKdbxItem(KdbxFileCompanion item) =>
      into(kdbxFile).insert(item);
}

@DriftAccessor(tables: [KdbxKeyFile])
class KdbxKeyFileDao extends DatabaseAccessor<AppDatabase>
    with _$KdbxKeyFileDaoMixin {
  KdbxKeyFileDao(super.db);

  Stream<List<KdbxKeyFileData>> watchAllKdbxKeyFiles() =>
      select(kdbxKeyFile).watch();

  Future<int> createKdbxKeyFile(KdbxKeyFileCompanion item) =>
      into(kdbxKeyFile).insert(item);

  Future<void> deleteKdbxKeyFile(int id) =>
      (delete(kdbxKeyFile)..where((t) => t.id.equals(id))).go();
}
