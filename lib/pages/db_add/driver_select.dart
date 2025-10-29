import 'dart:convert';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/db_add/webdav_settings.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';

class DriverSelect extends StatelessWidget {
  const DriverSelect({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text('数据源'),
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context, rootNavigator: true).pop();
          },
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.chevron_left),
        ),
        padding: EdgeInsetsDirectional.zero,
        transitionBetweenRoutes: false,
      ),
      child: Container(
        color: CupertinoColors.systemGroupedBackground,
        child: SafeArea(
          child: Column(
            mainAxisSize: MainAxisSize.max,
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
      type: FileType.any,
    );
    if (file == null || !context.mounted || file.files.isEmpty) return;

    final config = LocalConfig(path: file.files.first.path!);
    if (context.mounted) {
      print(jsonEncode(config.toJson()));
      Navigator.of(context).pop(config);
    }
  }

  Future<void> _onSelectDriver(
    BuildContext context,
    BaseDriverConfig config,
  ) async {
    print(jsonEncode(config.toJson()));
    Navigator.of(context).pop(config);
  }
}
