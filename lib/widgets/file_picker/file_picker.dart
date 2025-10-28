import 'package:flutter/cupertino.dart';
import 'package:keepass_one/services/file_system/file_system_models.dart';
import 'package:keepass_one/services/file_system/file_system_provider.dart';
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
}

class FilePickerPage extends StatefulWidget {
  const FilePickerPage({super.key, required this.path, this.name});

  final String path;
  final String? name;

  @override
  State<FilePickerPage> createState() => _FilePickerPageState();
}

class _FilePickerPageState extends State<FilePickerPage>
    with AutomaticKeepAliveClientMixin {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Container(
      color: CupertinoColors.systemBackground,
      child: FutureBuilder(
        future: context
            .read<FilePickerProvider>()
            .fileSystemProvider
            .listDirectory(widget.path),
        builder:
            (
              BuildContext context,
              AsyncSnapshot<List<FileSystemEntity>> snapshot,
            ) {
              switch (snapshot.connectionState) {
                case ConnectionState.none:
                  return _buildLoading();
                case ConnectionState.waiting:
                  return _buildLoading();
                case ConnectionState.active:
                  return _buildLoading();
                case ConnectionState.done:
                  return _buildFileList(context, snapshot.data ?? []);
              }
            },
      ),
    );
  }

  Widget _buildFileList(BuildContext context, List<FileSystemEntity> files) {
    return ListView.builder(
      itemCount: files.length,
      itemBuilder: (context, index) {
        final file = files[index];
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
                  builder: (_) => ChangeNotifierProvider.value(
                    value: provider,
                    child: FilePickerPage(path: file.path),
                  ),
                ),
              );
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
