import 'dart:async';
import 'package:flutter/cupertino.dart';
import 'package:keepass_one/services/file_system/file_system_models.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';
import 'package:keepass_one/widgets/file_picker/file_picker_provider.dart';
import 'package:provider/provider.dart';

class FilePicker extends StatelessWidget {
  FilePicker({super.key, required this.fileSystemProvider, this.onFileSelect});

  final FileSystemProvider fileSystemProvider;
  final GlobalKey<NavigatorState> _innerNavigatorKey =
      GlobalKey<NavigatorState>();

  final FutureOr<bool> Function(String)? onFileSelect;

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
    if (onFileSelect != null) {
      final result = await onFileSelect!(path);
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
  Exception? _error;

  @override
  void initState() {
    super.initState();
    _loadDirectory();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Container(
      color: CupertinoColors.systemBackground,
      child: _error != null
          ? _buildError()
          : _files == null
          ? _buildLoading()
          : _buildFileList(context, _files!),
    );
  }

  Widget _buildError() {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              CupertinoIcons.exclamationmark_triangle,
              size: 48,
              color: CupertinoColors.systemRed,
            ),
            const SizedBox(height: 16),
            Text(
              _error?.toString() ?? '发生错误',
              textAlign: TextAlign.center,
              style: const TextStyle(
                fontSize: 16,
                color: CupertinoColors.label,
              ),
            ),
            const SizedBox(height: 24),
            CupertinoButton(
              onPressed: _handleRefresh,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(CupertinoIcons.arrow_clockwise),
                  const SizedBox(width: 8),
                  Text('刷新'),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _loadDirectory() async {
    try {
      final files = await context
          .read<FilePickerProvider>()
          .fileSystemProvider
          .listDirectory(widget.path);
      if (mounted) {
        setState(() {
          _files = files;
          _error = null;
        });
      }
    } catch (error) {
      if (mounted) {
        setState(() {
          _error = error is Exception ? error : Exception(error.toString());
        });
      }
    }
  }

  Future<void> _handleRefresh() async {
    setState(() {
      _files = null;
      _error = null;
    });
    await _loadDirectory();
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
