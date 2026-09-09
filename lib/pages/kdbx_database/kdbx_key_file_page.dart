import 'dart:typed_data';

import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/file_selector/file_selector_page.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/driver_factory.dart';
import 'package:keepass_one/src/rust/api/kdbx.dart';
import 'package:keepass_one/widgets/sheet.dart';
import 'package:material_ui/material_ui.dart';

class KdbxKeyFileResult {
  final String fileName;
  final Uint8List keyHash;
  const KdbxKeyFileResult({required this.fileName, required this.keyHash});
}

class KdbxKeyFilePage extends StatelessWidget {
  const KdbxKeyFilePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('密钥文件')),
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
                      leading: const Icon(Icons.block_outlined),
                      title: const Text('没有密钥文件'),
                      onTap: () {
                        Navigator.of(context).pop(
                          KdbxKeyFileResult(
                            fileName: '',
                            keyHash: Uint8List(0),
                          ),
                        );
                      },
                    ),
                    ListTile(
                      leading: const Icon(Icons.file_copy_outlined),
                      title: const Text('导入密钥文件'),
                      onTap: () => _onImportKdbxFile(context, true),
                    ),
                    ListTile(
                      leading: const Icon(Icons.find_in_page_outlined),
                      title: const Text('选择密钥文件'),
                      onTap: () => _onImportKdbxFile(context, false),
                    ),
                  ],
                ),
              ),
            ),
            StreamBuilder(
              stream: getIt
                  .get<AppDatabase>()
                  .kdbxKeyFileDao
                  .watchAllKdbxKeyFiles(),
              builder: (context, AsyncSnapshot<List<KdbxKeyFileData>> snapshot) {
                if (!snapshot.hasData) {
                  return const Expanded(
                    child: Center(child: CircularProgressIndicator()),
                  );
                }

                if (snapshot.data!.isEmpty) {
                  return const SizedBox();
                }

                return Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Card.filled(
                    clipBehavior: Clip.antiAlias,
                    child: Column(
                      children: [
                        for (final item in snapshot.data!)
                          ListTile(
                            leading: const Icon(Icons.description_outlined),
                            title: Text(item.name),
                            subtitle: Text(item.path),
                            onTap: () {
                              Navigator.of(context).pop(
                                KdbxKeyFileResult(
                                  fileName: item.name,
                                  keyHash: item.data,
                                ),
                              );
                            },
                          ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _onImportKdbxFile(BuildContext context, bool isSave) async {
    showAppModal(
      context: context,
      builder: (context) => FileSelectorPage(
        title: '选择密钥文件',
        onSelectDriver: (config) => _onSelectKeyFile(context, config, isSave),
      ),
      useNestedNavigation: true,
      enableDrag: false,
    );
  }

  Future<void> _onSelectKeyFile(
    BuildContext context,
    BaseDriverConfig? config,
    bool isSave,
  ) async {
    if (config == null) return;
    final data = await SyncDriverFactory.getFile(config);
    final keyHash = openKdbxKeyFile(keyFile: data.toList());
    if (isSave) {
      getIt.get<AppDatabase>().kdbxKeyFileDao.createKdbxKeyFile(
        KdbxKeyFileCompanion.insert(
          name: config.name,
          path: config.description,
          data: keyHash,
          createdAt: DateTime.now(),
          lastModifiedAt: DateTime.now(),
        ),
      );
    }

    Navigator.of(
      // ignore: use_build_context_synchronously
      context,
      rootNavigator: true,
    ).pop(KdbxKeyFileResult(fileName: config.name, keyHash: keyHash));
  }
}
