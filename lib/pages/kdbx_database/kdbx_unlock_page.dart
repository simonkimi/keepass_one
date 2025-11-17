import 'package:flutter/cupertino.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_key_file_page.dart';
import 'package:keepass_one/widgets/sheet.dart';

class KdbxUnlockPage extends HookWidget {
  const KdbxUnlockPage({super.key, required this.name});

  final String name;

  @override
  Widget build(BuildContext context) {
    final displayPassword = useState(false);
    final keyFile = useState<KdbxKeyFileResult?>(null);
    final passwordController = useTextEditingController();

    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(middle: Text(name)),
      child: SafeArea(
        child: SizedBox(
          width: double.infinity,
          child: Padding(
            padding: .symmetric(horizontal: 50),
            child: Column(
              mainAxisAlignment: .center,
              crossAxisAlignment: .center,
              children: [
                CupertinoTextField(
                  placeholder: '密码',
                  controller: passwordController,
                  obscureText: !displayPassword.value,
                  suffix: CupertinoButton(
                    padding: .zero,
                    child: Icon(
                      displayPassword.value
                          ? CupertinoIcons.eye_slash_fill
                          : CupertinoIcons.eye_solid,
                    ),
                    onPressed: () {
                      displayPassword.value = !displayPassword.value;
                    },
                  ),
                ),
                SizedBox(height: 5),
                ClipRRect(
                  borderRadius: .circular(5),
                  child: CupertinoListTile(
                    padding: .zero,
                    leading: Icon(CupertinoIcons.doc, size: 16),
                    title: Text(
                      keyFile.value?.fileName ?? '添加密钥文件',
                      style: TextStyle(
                        color: CupertinoColors.systemBlue.resolveFrom(context),
                        fontSize: 16,
                      ),
                    ),
                    onTap: () async {
                      final result =
                          await showCupertinoModal<KdbxKeyFileResult>(
                            context: context,
                            builder: (context) => KdbxKeyFilePage(),
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
                ),
                SizedBox(height: 20),
                SizedBox(
                  width: double.infinity,
                  child: CupertinoButton.tinted(
                    padding: .zero,
                    onPressed: () {},
                    child: Text('解锁'),
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
