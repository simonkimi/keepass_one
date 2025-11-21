import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_unlock_page.dart';
import 'package:keepass_one/pages/kdbx_selector/kdbx_add.dart';
import 'package:keepass_one/services/database/database.dart';

class KdbxSelectorPage extends StatelessWidget {
  const KdbxSelectorPage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: _buildCupertinoNavigationBar(context),
      child: StreamBuilder(
        stream: getIt.get<AppDatabase>().kdbxFileDao.watchAllKdbxItems(),
        builder: (context, AsyncSnapshot<List<KdbxFileData>> snapshot) {
          if (!snapshot.hasData) {
            return const Center(child: CupertinoActivityIndicator());
          }

          if (snapshot.data!.isEmpty) {
            return Center(
              child: CupertinoButton(
                onPressed: () {
                  onAddKdbxSource(context);
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
                onTap: () {
                  Navigator.of(context).push(
                    CupertinoPageRoute(
                      builder: (context) => KdbxUnlockPage(name: item.name),
                    ),
                  );
                },
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
          onAddKdbxSource(context);
        },
        padding: EdgeInsets.zero,
        child: Icon(CupertinoIcons.add),
      ),
      padding: EdgeInsetsDirectional.zero,
      middle: Text('Keepass One'),
    );
  }
}
