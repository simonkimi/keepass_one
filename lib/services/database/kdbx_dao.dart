import 'package:drift/drift.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/database/kdbx.dart';

part 'kdbx_dao.g.dart';

@DriftAccessor(tables: [KdbxItems])
class KdbxItemDao extends DatabaseAccessor<AppDatabase>
    with _$KdbxItemDaoMixin {
  KdbxItemDao(super.db);

  Stream<List<KdbxItem>> watchAllKdbxItems() => select(kdbxItems).watch();

  Future<int> createKdbxItem(KdbxItemsCompanion item) =>
      into(kdbxItems).insert(item);
}
