import 'dart:async';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/file_selector/webdav_settings.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';

class FileSelectorPage extends StatelessWidget {
  const FileSelectorPage({
    super.key,
    required this.title,
    required this.onSelectDriver,
    this.onFileSelect,
  });

  final String title;
  final ValueChanged<BaseDriverConfig?> onSelectDriver;
  final FutureOr<bool> Function(String)? onFileSelect;

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text(title),
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
      onSelectDriver(config);
    }
  }

  Future<void> _onSelectLocal(BuildContext context) async {
    final FilePickerResult? file = await FilePicker.platform.pickFiles(
      type: .any,
    );

    if (file == null || !context.mounted || file.files.isEmpty) return;
    onSelectDriver(LocalConfig(path: file.files.first.path!));
  }
}
