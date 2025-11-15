import 'package:flutter/cupertino.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/pages/kdbx_database/kdbx_file_page.dart';
import 'package:keepass_one/widgets/sheet.dart';

class KdbxUnlockPage extends HookWidget {
  const KdbxUnlockPage({super.key, required this.name});

  final String name;

  @override
  Widget build(BuildContext context) {
    final displayPassword = useState(false);

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
                  onChanged: (value) {},
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
                CupertinoListTile(
                  leading: Icon(CupertinoIcons.doc),
                  title: Text(
                    '添加密钥文件',
                    style: TextStyle(
                      color: CupertinoColors.systemBlue.resolveFrom(context),
                    ),
                  ),
                  onTap: () {
                    showCupertinoModal(
                      context: context,
                      builder: (context) => KdbxFileFilePage(),
                    );
                  },
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
