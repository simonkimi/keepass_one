import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/file_selector/file_selector_page.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/services/sync/driver_config.dart';
import 'package:keepass_one/services/sync/driver_factory.dart';
import 'package:keepass_one/utils/hash.dart';
import 'package:keepass_one/utils/path_provider.dart';
import 'package:keepass_one/utils/platform.dart';
import 'package:keepass_one/utils/throttle.dart';
import 'package:keepass_one/widgets/loading.dart';
import 'package:keepass_one/widgets/sheet.dart';

void onAddKdbxSource(BuildContext context) {
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
  Navigator.of(context).push(
    CupertinoPageRoute(
      builder: (context) => KdbxLoadingPage(
        config: config,
        onFileLoaded: (Uint8List data) async {
          _onFileLoaded(context, data, config);
          Navigator.of(context, rootNavigator: true).pop(data);
        },
      ),
    ),
  );
}

Future<void> _onFileLoaded(
  BuildContext context,
  Uint8List data,
  BaseDriverConfig config,
) async {
  final database = getIt.get<AppDatabase>();
  final configId = await database.kdbxFileDao.createKdbxItemFromConfig(config);
  final backupPath = await createKdbxBackupPath(configId);
  await File(backupPath).writeAsBytes(data);
  await database.kdbxFileBackupDao.createKdbxFileBackup(
    configId,
    backupPath,
    hashU8List(data),
  );
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

class KdbxLoadingPage extends StatefulWidget {
  const KdbxLoadingPage({
    super.key,
    required this.config,
    required this.onFileLoaded,
  });

  final BaseDriverConfig config;
  final ValueChanged<Uint8List> onFileLoaded;

  @override
  State<KdbxLoadingPage> createState() => _KdbxLoadingPageState();
}

class _KdbxLoadingPageState extends State<KdbxLoadingPage> {
  double _loadingProgress = 0.0;
  late final ThrottleCallback _throttledUpdateProgress;

  @override
  void initState() {
    super.initState();
    _throttledUpdateProgress = throttle(const Duration(milliseconds: 200), () {
      if (mounted) {
        setState(() {});
      }
    });

    SyncDriverFactory.getFile(
      widget.config,
      onProgress: (count, total) {
        if (total != 0) {
          _loadingProgress = count / total;
          _throttledUpdateProgress();
        }
      },
    ).then((Uint8List data) {
      if (mounted) {
        widget.onFileLoaded(data);
        Navigator.of(context, rootNavigator: true).pop();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return LoadingWidget(
      message: '正在添加数据源...',
      loadingProgress: _loadingProgress,
    );
  }
}
