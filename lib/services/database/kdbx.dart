import 'package:drift/drift.dart';

class KdbxItems extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get name => text()();
  TextColumn get config => text()();
  DateTimeColumn get createdAt => dateTime()();

  DateTimeColumn get lastAccessedAt => dateTime()();
  DateTimeColumn get lastModifiedAt => dateTime()();
  DateTimeColumn get lastSyncedAt => dateTime()();

  TextColumn get lastHashValue => text()();
}
