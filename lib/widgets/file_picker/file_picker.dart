import 'package:flutter/cupertino.dart';
import 'package:keepass_one/services/file_system/file_system_models.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';
import 'package:keepass_one/utils/platform.dart';
import 'package:keepass_one/widgets/file_picker/file_picker_provider.dart';
import 'package:provider/provider.dart';

class FilePicker extends StatelessWidget {
  FilePicker({super.key, required this.fileSystemProvider});

  final FileSystemProvider fileSystemProvider;
  final GlobalKey<NavigatorState> _innerNavigatorKey =
      GlobalKey<NavigatorState>();

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) {
        return FilePickerProvider(fileSystemProvider: fileSystemProvider);
      },
      child: Builder(
        builder: (context) {
          return CupertinoPageScaffold(
            navigationBar: _buildNavigatorBar(context),
            child: Navigator(
              key: _innerNavigatorKey,
              onGenerateInitialRoutes: (navigator, initialRoute) {
                return <Route<void>>[
                  CupertinoPageRoute<void>(
                    builder: (_) => FilePickerPage(
                      path: context
                          .read<FilePickerProvider>()
                          .fileSystemProvider
                          .getRootPath(),
                      onSelect: (path) => _onSelectFile(context, path),
                      canGoBack: false,
                    ),
                  ),
                ];
              },
            ),
          );
        },
      ),
    );
  }

  CupertinoNavigationBar _buildNavigatorBar(BuildContext context) {
    return CupertinoNavigationBar(
      middle: Text(context.watch<FilePickerProvider>().name ?? '选择文件'),
      leading: CupertinoButton(
        onPressed: () {
          Navigator.of(context).pop();
        },
        padding: EdgeInsets.zero,
        child: Text("返回"),
      ),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          CupertinoButton(
            onPressed: () {
              context.read<FilePickerProvider>().popPath();
              _innerNavigatorKey.currentState?.maybePop();
            },
            padding: EdgeInsets.zero,
            child: const Icon(CupertinoIcons.chevron_up),
          ),
        ],
      ),
    );
  }

  Future<void> _onSelectFile(BuildContext context, String path) async {
    if (!path.endsWith('.kdbx')) {
      final result = await showCupertinoDialog<bool>(
        barrierDismissible: true,
        context: context,
        builder: (context) => CupertinoAlertDialog(
          title: Text('提示'),
          content: Text('这看起来并不是一个kdbx文件, 是否任然要选择此文件?'),
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
      if (result != true) {
        return;
      }
    }

    if (context.mounted) {
      Navigator.of(context).pop(path);
    }
  }
}

class FilePickerPage extends StatefulWidget {
  const FilePickerPage({
    super.key,
    required this.path,
    this.name,
    this.onSelect,
    this.canGoBack = false,
  });

  final String path;
  final String? name;
  final ValueChanged<String>? onSelect;
  final bool canGoBack;

  @override
  State<FilePickerPage> createState() => _FilePickerPageState();
}

class _FilePickerPageState extends State<FilePickerPage>
    with AutomaticKeepAliveClientMixin {
  List<FileSystemEntity>? _files;

  @override
  void initState() {
    super.initState();
    context
        .read<FilePickerProvider>()
        .fileSystemProvider
        .listDirectory(widget.path)
        .then((value) {
          setState(() {
            _files = value;
          });
        });
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Container(
      color: CupertinoColors.systemBackground,
      child: _files == null
          ? _buildLoading()
          : _buildFileList(context, _files!),
    );
  }

  Widget _buildFileList(BuildContext context, List<FileSystemEntity> files) {
    return ListView.builder(
      itemCount: widget.canGoBack ? files.length + 1 : files.length,
      itemBuilder: (context, index) {
        if (widget.canGoBack && index == 0) {
          return CupertinoListTile(
            leading: Icon(CupertinoIcons.chevron_left),
            title: Text("返回上一级"),
            onTap: () {
              context.read<FilePickerProvider>().popPath();
              Navigator.of(context).pop();
            },
          );
        }

        final fileIndex = widget.canGoBack ? index - 1 : index;
        final file = files[fileIndex];

        return CupertinoListTile(
          leading: Icon(
            file.isDirectory ? CupertinoIcons.folder : CupertinoIcons.doc,
          ),
          title: Text(file.name),
          trailing: file.isDirectory
              ? Icon(CupertinoIcons.chevron_right)
              : null,
          onTap: () async {
            if (file.isDirectory) {
              final provider = context.read<FilePickerProvider>();
              provider.pushPath(file.name);
              await Navigator.of(context).push(
                CupertinoPageRoute(
                  builder: (_) => FilePickerPage(
                    path: file.path,
                    onSelect: widget.onSelect,
                    canGoBack: true,
                  ),
                ),
              );
            } else {
              widget.onSelect?.call(file.path);
            }
          },
        );
      },
    );
  }

  Widget _buildLoading() {
    return Center(child: CupertinoActivityIndicator());
  }

  @override
  bool get wantKeepAlive => true;
}
