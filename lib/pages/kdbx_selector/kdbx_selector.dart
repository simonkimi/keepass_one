import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_unlock_page.dart';
import 'package:keepass_one/pages/kdbx_selector/kdbx_add.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:material_ui/material_ui.dart';

class KdbxSelectorPage extends StatelessWidget {
  const KdbxSelectorPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(context),
      body: StreamBuilder(
        stream: getIt.get<AppDatabase>().kdbxFileDao.watchAllKdbxItems(),
        builder: (context, AsyncSnapshot<List<KdbxFileData>> snapshot) {
          if (!snapshot.hasData) {
            return const Center(child: CircularProgressIndicator());
          }

          if (snapshot.data!.isEmpty) {
            return Center(
              child: FilledButton.tonal(
                onPressed: () {
                  onAddKdbxSource(context);
                },
                child: const Text('添加数据源'),
              ),
            );
          }

          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: snapshot.data!.length,
            itemBuilder: (context, index) {
              final item = snapshot.data![index];
              return Card.filled(
                clipBehavior: Clip.antiAlias,
                margin: const EdgeInsets.only(bottom: 8),
                child: ListTile(
                  title: Text(item.name),
                  subtitle: Text(item.description),
                  trailing: const Icon(Icons.chevron_right),
                  onTap: () {
                    Navigator.of(context).push(
                      MaterialPageRoute(
                        builder: (context) => KdbxUnlockPage(name: item.name),
                      ),
                    );
                  },
                ),
              );
            },
          );
        },
      ),
    );
  }

  AppBar _buildAppBar(BuildContext context) {
    return AppBar(
      leading: IconButton(
        onPressed: () {
          onAddKdbxSource(context);
        },
        icon: const Icon(Icons.add_outlined),
      ),
      title: const Text('Keepass One'),
    );
  }
}
