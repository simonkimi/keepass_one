import 'package:drift/drift.dart';

class KdbxFile extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get name => text()();
  TextColumn get description => text()();
  TextColumn get type => text()();
  TextColumn get config => text()();

  DateTimeColumn get createdAt => dateTime()();

  DateTimeColumn get lastAccessedAt => dateTime()();
  DateTimeColumn get lastModifiedAt => dateTime()();
  DateTimeColumn get lastSyncedAt => dateTime()();

  TextColumn get lastHashValue => text()();
}

class KdbxKeyFile extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get name => text()();
  TextColumn get path => text()();

  BlobColumn get data => blob()();
  DateTimeColumn get createdAt => dateTime()();
  DateTimeColumn get lastModifiedAt => dateTime()();
}

class KdbxFileBackup extends Table {
  IntColumn get id => integer().autoIncrement()();
  IntColumn get kdbxFileId => integer()();

  TextColumn get filePath => text()();
  TextColumn get apiHashValue => text().nullable()();
  TextColumn get fileHashValue => text()();

  BoolColumn get isModified => boolean()();
  BoolColumn get isSynced => boolean()();

  DateTimeColumn get createdAt => dateTime()();
  DateTimeColumn get lastModifiedAt => dateTime().nullable()();
  DateTimeColumn get lastSyncedAt => dateTime().nullable()();
}
