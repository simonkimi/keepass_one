import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/kdbx_add/kdbx_source_selector.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/widgets/sheet.dart';

class KdbxSelectorPage extends StatelessWidget {
  const KdbxSelectorPage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: _buildCupertinoNavigationBar(context),
      child: StreamBuilder(
        stream: getIt.get<AppDatabase>().kdbxItemDao.watchAllKdbxItems(),
        builder: (context, AsyncSnapshot<List<KdbxItem>> snapshot) {
          if (!snapshot.hasData) {
            return const Center(child: CupertinoActivityIndicator());
          }

          if (snapshot.data!.isEmpty) {
            return Center(
              child: CupertinoButton(
                onPressed: () {
                  _onAddKdbxSource(context);
                },
                child: Text('添加数据源'),
              ),
            );
          }

          return ListView.builder(
            itemCount: snapshot.data!.length,
            itemBuilder: (context, index) {
              final item = snapshot.data![index];
              return CupertinoListTile(
                title: Text(item.name),
                subtitle: Text(item.description),
                trailing: Icon(CupertinoIcons.chevron_right),
                onTap: () {},
              );
            },
          );
        },
      ),
    );
  }

  CupertinoNavigationBar _buildCupertinoNavigationBar(BuildContext context) {
    return CupertinoNavigationBar(
      leading: CupertinoButton(
        onPressed: () {
          _onAddKdbxSource(context);
        },
        padding: EdgeInsets.zero,
        child: Icon(CupertinoIcons.add),
      ),
      padding: EdgeInsetsDirectional.zero,
      middle: Text('Keepass One'),
    );
  }

  void _onAddKdbxSource(BuildContext context) {
    showCupertinoModal(
      context: context,
      builder: (context) => KdbxSourceSelectorPage(),
      useNestedNavigation: true,
      enableDrag: false,
    );
  }
}
