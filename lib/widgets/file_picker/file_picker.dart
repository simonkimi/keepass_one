import 'dart:async';
import 'package:keepass_one/services/file_system/file_system_models.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';
import 'package:keepass_one/widgets/file_picker/file_picker_provider.dart';
import 'package:material_ui/material_ui.dart';
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
          return Scaffold(
            appBar: _buildAppBar(context),
            body: Navigator(
              key: _innerNavigatorKey,
              onGenerateInitialRoutes: (navigator, initialRoute) {
                return <Route<void>>[
                  MaterialPageRoute<void>(
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

  AppBar _buildAppBar(BuildContext context) {
    return AppBar(
      title: Text(context.watch<FilePickerProvider>().name ?? '选择文件'),
      leadingWidth: 80,
      leading: TextButton(
        onPressed: () {
          Navigator.of(context).pop();
        },
        child: const Text('返回'),
      ),
      actions: [
        IconButton(
          onPressed: () {
            context.read<FilePickerProvider>().popPath();
            _innerNavigatorKey.currentState?.maybePop();
          },
          icon: const Icon(Icons.keyboard_arrow_up),
        ),
      ],
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
    if (_error != null) {
      return _buildError(context);
    }
    if (_files == null) {
      return _buildLoading();
    }
    return _buildFileList(context, _files!);
  }

  Widget _buildError(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.warning_amber_outlined,
              size: 48,
              color: colorScheme.error,
            ),
            const SizedBox(height: 16),
            Text(
              _error?.toString() ?? '发生错误',
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 16,
                color: colorScheme.onSurface,
              ),
            ),
            const SizedBox(height: 24),
            TextButton.icon(
              onPressed: _handleRefresh,
              icon: const Icon(Icons.refresh),
              label: const Text('刷新'),
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
          return ListTile(
            leading: const Icon(Icons.chevron_left),
            title: const Text('返回上一级'),
            onTap: () {
              context.read<FilePickerProvider>().popPath();
              Navigator.of(context).pop();
            },
          );
        }

        final fileIndex = widget.canGoBack ? index - 1 : index;
        final file = files[fileIndex];

        return ListTile(
          leading: Icon(
            file.isDirectory
                ? Icons.folder_outlined
                : Icons.description_outlined,
          ),
          title: Text(file.name),
          trailing: file.isDirectory ? const Icon(Icons.chevron_right) : null,
          onTap: () async {
            if (file.isDirectory) {
              final provider = context.read<FilePickerProvider>();
              provider.pushPath(file.name);
              await Navigator.of(context).push(
                MaterialPageRoute(
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
    return const Center(child: CircularProgressIndicator());
  }

  @override
  bool get wantKeepAlive => true;
}
