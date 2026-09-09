import 'dart:async';

import 'package:file_picker/file_picker.dart';
import 'package:keepass_one/pages/file_selector/webdav_settings.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/local/local_config.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:material_ui/material_ui.dart';

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
    return Scaffold(
      appBar: AppBar(
        title: Text(title),
        leading: IconButton(
          onPressed: () {
            Navigator.of(context, rootNavigator: true).pop();
          },
          icon: const Icon(Icons.chevron_left),
        ),
      ),
      body: SafeArea(
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.all(16),
              child: Card.filled(
                clipBehavior: Clip.antiAlias,
                child: Column(
                  children: [
                    ListTile(
                      leading: const Icon(Icons.folder_outlined),
                      title: const Text('本地文件'),
                      onTap: () => _onSelectLocal(context),
                    ),
                    ListTile(
                      leading: const Icon(Icons.link_outlined),
                      title: const Text('WebDAV'),
                      onTap: () => _onSelectWebDav(context),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _onSelectWebDav(BuildContext context) async {
    final config = await Navigator.of(context).push<WebDavConfig>(
      MaterialPageRoute(builder: (context) => const WebdavSettingsPage()),
    );
    if (config != null && context.mounted) {
      onSelectDriver(config);
    }
  }

  Future<void> _onSelectLocal(BuildContext context) async {
    final file = await FilePicker.pickFile(
      type: FileType.any,
    );

    if (file == null || !context.mounted || file.path == null) return;
    onSelectDriver(LocalConfig(path: file.path!));
  }
}
