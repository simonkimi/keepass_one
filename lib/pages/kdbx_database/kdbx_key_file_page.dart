import 'dart:typed_data';

import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/file_selector/file_selector_page.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/driver_factory.dart';
import 'package:keepass_one/src/rust/api/kdbx.dart';
import 'package:keepass_one/widgets/sheet.dart';

class KdbxKeyFileResult {
  final String fileName;
  final Uint8List keyHash;
  const KdbxKeyFileResult({required this.fileName, required this.keyHash});
}

class KdbxKeyFilePage extends StatelessWidget {
  const KdbxKeyFilePage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(middle: Text('密钥文件')),
      child: Container(
        color: CupertinoColors.systemGroupedBackground,
        child: SafeArea(
          child: Column(
            children: [
              CupertinoListSection.insetGrouped(
                children: [
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.nosign),
                    title: Text('没有密钥文件'),
                    onTap: () {
                      Navigator.of(context).pop(
                        KdbxKeyFileResult(fileName: '', keyHash: Uint8List(0)),
                      );
                    },
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.doc_on_doc),
                    title: Text('导入密钥文件'),
                    onTap: () => _onImportKdbxFile(context, true),
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.doc_text_search),
                    title: Text('选择密钥文件'),
                    onTap: () => _onImportKdbxFile(context, false),
                  ),
                ],
              ),

              StreamBuilder(
                stream: getIt
                    .get<AppDatabase>()
                    .kdbxKeyFileDao
                    .watchAllKdbxKeyFiles(),
                builder:
                    (context, AsyncSnapshot<List<KdbxKeyFileData>> snapshot) {
                      if (!snapshot.hasData) {
                        return const Expanded(
                          child: Center(child: CupertinoActivityIndicator()),
                        );
                      }

                      if (snapshot.data!.isEmpty) {
                        return const SizedBox();
                      }

                      return CupertinoListSection.insetGrouped(
                        children: [
                          for (final item in snapshot.data!)
                            CupertinoListTile(
                              leading: Icon(CupertinoIcons.doc),
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
                      );
                    },
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _onImportKdbxFile(BuildContext context, bool isSave) async {
    showCupertinoModal(
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
