import 'dart:convert';

import 'package:drift/drift.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/database/entity.dart';
import 'package:keepass_one/services/sync/driver_config.dart';

part 'dao.g.dart';

@DriftAccessor(tables: [KdbxFile])
class KdbxFileDao extends DatabaseAccessor<AppDatabase>
    with _$KdbxFileDaoMixin {
  KdbxFileDao(super.db);

  Stream<List<KdbxFileData>> watchAllKdbxItems() => select(kdbxFile).watch();

  Future<int> createKdbxItem(KdbxFileCompanion item) =>
      into(kdbxFile).insert(item);

  Future<int> createKdbxItemFromConfig(BaseDriverConfig config) =>
      into(kdbxFile).insert(
        KdbxFileCompanion.insert(
          name: config.name,
          description: config.description,
          type: config.type.key,
          config: jsonEncode(config.toJson()),
          createdAt: DateTime.now(),
          lastAccessedAt: DateTime.now(),
          lastModifiedAt: DateTime.now(),
          lastSyncedAt: DateTime.now(),
          lastHashValue: '',
        ),
      );
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

@DriftAccessor(tables: [KdbxFileBackup])
class KdbxFileBackupDao extends DatabaseAccessor<AppDatabase>
    with _$KdbxFileBackupDaoMixin {
  KdbxFileBackupDao(super.db);

  Future<int> createKdbxFileBackup(
    int configId,
    String backupPath,
    String fileHash,
  ) => into(kdbxFileBackup).insert(
    KdbxFileBackupCompanion.insert(
      kdbxFileId: configId,
      filePath: backupPath,
      fileHashValue: fileHash,
      isModified: false,
      isSynced: true,
      createdAt: DateTime.now(),
      lastModifiedAt: Value(DateTime.now()),
      lastSyncedAt: Value(DateTime.now()),
    ),
  );
}
