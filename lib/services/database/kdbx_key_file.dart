import 'package:drift/drift.dart';

class KdbxKeyFile extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get name => text()();
  TextColumn get path => text()();

  BlobColumn get data => blob()();
  DateTimeColumn get createdAt => dateTime()();
  DateTimeColumn get lastModifiedAt => dateTime()();
}
