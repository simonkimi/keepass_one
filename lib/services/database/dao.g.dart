// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'dao.dart';

// ignore_for_file: type=lint
mixin _$KdbxFileDaoMixin on DatabaseAccessor<AppDatabase> {
  $KdbxFileTable get kdbxFile => attachedDatabase.kdbxFile;
  KdbxFileDaoManager get managers => KdbxFileDaoManager(this);
}

class KdbxFileDaoManager {
  final _$KdbxFileDaoMixin _db;
  KdbxFileDaoManager(this._db);
  $$KdbxFileTableTableManager get kdbxFile =>
      $$KdbxFileTableTableManager(_db.attachedDatabase, _db.kdbxFile);
}

mixin _$KdbxKeyFileDaoMixin on DatabaseAccessor<AppDatabase> {
  $KdbxKeyFileTable get kdbxKeyFile => attachedDatabase.kdbxKeyFile;
  KdbxKeyFileDaoManager get managers => KdbxKeyFileDaoManager(this);
}

class KdbxKeyFileDaoManager {
  final _$KdbxKeyFileDaoMixin _db;
  KdbxKeyFileDaoManager(this._db);
  $$KdbxKeyFileTableTableManager get kdbxKeyFile =>
      $$KdbxKeyFileTableTableManager(_db.attachedDatabase, _db.kdbxKeyFile);
}

mixin _$KdbxFileBackupDaoMixin on DatabaseAccessor<AppDatabase> {
  $KdbxFileBackupTable get kdbxFileBackup => attachedDatabase.kdbxFileBackup;
  KdbxFileBackupDaoManager get managers => KdbxFileBackupDaoManager(this);
}

class KdbxFileBackupDaoManager {
  final _$KdbxFileBackupDaoMixin _db;
  KdbxFileBackupDaoManager(this._db);
  $$KdbxFileBackupTableTableManager get kdbxFileBackup =>
      $$KdbxFileBackupTableTableManager(
        _db.attachedDatabase,
        _db.kdbxFileBackup,
      );
}
