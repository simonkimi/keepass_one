import 'dart:async';
import 'dart:convert';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/file_selector/file_selector_page.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_unlock_page.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/utils/platform.dart';
import 'package:keepass_one/widgets/sheet.dart';

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
      builder: (context) => FileSelectorPage(
        title: '添加数据源',
        onSelectDriver: (config) => _onSelectDriver(context, config),
        onFileSelect: (path) => _onSelectFile(context, path),
      ),
      useNestedNavigation: true,
      enableDrag: false,
    );
  }

  void _onSelectDriver(BuildContext context, BaseDriverConfig? config) {
    if (config == null) return;
    final configJson = jsonEncode(config.toJson());
    getIt.get<AppDatabase>().kdbxFileDao.createKdbxItem(
      KdbxFileCompanion.insert(
        name: config.name,
        type: config.type.key,
        description: config.description,
        config: configJson,
        createdAt: DateTime.now(),
        lastAccessedAt: DateTime.fromMillisecondsSinceEpoch(0),
        lastModifiedAt: DateTime.now(),
        lastSyncedAt: DateTime.fromMillisecondsSinceEpoch(0),
        lastHashValue: '',
      ),
    );
    Navigator.of(context, rootNavigator: true).pop(config);
  }

  FutureOr<bool> _onSelectFile(BuildContext context, String path) async {
    if (!path.endsWith('.kdbx')) {
      final result = await showCupertinoDialog<bool>(
        barrierDismissible: true,
        context: context,
        builder: (context) => CupertinoAlertDialog(
          title: Text('提示'),
          content: Text('这看起来并不是一个kdbx文件, 是否仍然要选择此文件?'),
          actions: platformActionsBaseOnMobile([
            CupertinoDialogAction(
              child: Text('取消'),
              onPressed: () {
                Navigator.of(context).pop(false);
              },
            ),
            CupertinoDialogAction(
              child: Text('确定', style: TextStyle(fontWeight: FontWeight.bold)),
              onPressed: () {
                Navigator.of(context).pop(true);
              },
            ),
          ]),
        ),
      );
      return result == true;
    }
    return true;
  }
}
