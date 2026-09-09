import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_key_file_page.dart';
import 'package:keepass_one/widgets/sheet.dart';
import 'package:material_ui/material_ui.dart';

class KdbxUnlockPage extends HookWidget {
  const KdbxUnlockPage({super.key, required this.name});

  final String name;

  @override
  Widget build(BuildContext context) {
    final displayPassword = useState(false);
    final keyFile = useState<KdbxKeyFileResult?>(null);
    final passwordController = useTextEditingController();
    final colorScheme = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(title: Text(name)),
      body: SafeArea(
        child: SizedBox(
          width: double.infinity,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 50),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                TextField(
                  controller: passwordController,
                  obscureText: !displayPassword.value,
                  decoration: InputDecoration(
                    hintText: '密码',
                    suffixIcon: IconButton(
                      icon: Icon(
                        displayPassword.value
                            ? Icons.visibility_off_outlined
                            : Icons.visibility_outlined,
                      ),
                      onPressed: () {
                        displayPassword.value = !displayPassword.value;
                      },
                    ),
                  ),
                ),
                const SizedBox(height: 5),
                ListTile(
                  contentPadding: EdgeInsets.zero,
                  leading: const Icon(Icons.description_outlined, size: 16),
                  title: Text(
                    keyFile.value?.fileName ?? '添加密钥文件',
                    style: TextStyle(color: colorScheme.primary, fontSize: 16),
                  ),
                  onTap: () async {
                    final result = await showAppModal<KdbxKeyFileResult>(
                      context: context,
                      builder: (context) => const KdbxKeyFilePage(),
                    );
                    if (result != null) {
                      if (result.keyHash.isNotEmpty) {
                        keyFile.value = result;
                      } else {
                        keyFile.value = null;
                      }
                    }
                  },
                ),
                const SizedBox(height: 20),
                SizedBox(
                  width: double.infinity,
                  child: FilledButton.tonal(
                    onPressed: () {},
                    child: const Text('解锁'),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
