import 'dart:convert';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/kdbx_add/webdav_settings.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';

class KdbxSourceSelectorPage extends StatelessWidget {
  const KdbxSourceSelectorPage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text('数据源'),
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context, rootNavigator: true).pop();
          },
          padding: .zero,
          child: Icon(CupertinoIcons.chevron_left),
        ),
        padding: EdgeInsetsDirectional.zero,
        transitionBetweenRoutes: false,
      ),
      child: Container(
        color: CupertinoColors.systemGroupedBackground,
        child: SafeArea(
          child: Column(
            mainAxisSize: .max,
            children: [
              CupertinoListSection.insetGrouped(
                children: [
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.folder),
                    title: Text('本地文件'),
                    onTap: () => _onSelectLocal(context),
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.link_circle),
                    title: Text('WebDAV'),
                    onTap: () => _onSelectWebDav(context),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _onSelectWebDav(BuildContext context) async {
    final config = await Navigator.of(context).push<WebDavConfig>(
      CupertinoPageRoute(builder: (context) => WebdavSettingsPage()),
    );
    if (config != null && context.mounted) {
      await _onSelectDriver(context, config);
    }
  }

  Future<void> _onSelectLocal(BuildContext context) async {
    final FilePickerResult? file = await FilePicker.platform.pickFiles(
      type: .any,
    );

    if (file == null || !context.mounted || file.files.isEmpty) return;
    await _onSelectDriver(context, LocalConfig(path: file.files.first.path!));
  }

  Future<void> _onSelectDriver(
    BuildContext context,
    BaseDriverConfig config,
  ) async {
    final configJson = jsonEncode(config.toJson());
    getIt.get<AppDatabase>().kdbxItemDao.createKdbxItem(
      KdbxItemsCompanion.insert(
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
}
