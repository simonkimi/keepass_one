import 'package:flutter/cupertino.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/services/sync/webdav/webdav.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:keepass_one/widgets/file_picker/file_picker.dart';

class WebdavSettingsPage extends HookWidget {
  const WebdavSettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final urlController = useTextEditingController(
      text: dotenv.env['WEBDAV_BASE_URL'],
    );
    final usernameController = useTextEditingController(
      text: dotenv.env['WEBDAV_USERNAME'],
    );
    final passwordController = useTextEditingController(
      text: dotenv.env['WEBDAV_PASSWORD'],
    );
    final tlsInsecureSkipVerify = useState(false);

    return CupertinoPageScaffold(
      navigationBar: _buildNavigationBar(
        context,
        urlController,
        usernameController,
        passwordController,
        tlsInsecureSkipVerify,
      ),
      child: SafeArea(
        child: Container(
          color: CupertinoColors.systemGroupedBackground,
          child: Form(
            child: CupertinoListSection.insetGrouped(
              children: [
                CupertinoTextFormFieldRow(
                  controller: urlController,
                  prefix: Icon(CupertinoIcons.link),
                  placeholder: 'URL',
                  validator: (value) {
                    if (value == null || value.isEmpty) return '请输入URL';
                    return null;
                  },
                ),
                CupertinoTextFormFieldRow(
                  controller: usernameController,
                  prefix: Icon(CupertinoIcons.person),
                  placeholder: '用户名',
                ),
                CupertinoTextFormFieldRow(
                  controller: passwordController,
                  prefix: Text('密码'),
                  obscureText: true,
                ),
                CupertinoFormRow(
                  prefix: Text('跳过TLS验证'),
                  child: CupertinoSwitch(
                    value: tlsInsecureSkipVerify.value,
                    onChanged: (value) {
                      tlsInsecureSkipVerify.value = value;
                    },
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  CupertinoNavigationBar _buildNavigationBar(
    BuildContext context,
    TextEditingController urlController,
    TextEditingController usernameController,
    TextEditingController passwordController,
    ValueNotifier<bool> tlsInsecureSkipVerify,
  ) {
    return CupertinoNavigationBar(
      middle: Text('WebDAV设置'),
      padding: EdgeInsetsDirectional.zero,
      leading: CupertinoButton(
        onPressed: () {
          Navigator.of(context, rootNavigator: true).pop();
        },
        padding: EdgeInsets.zero,
        child: Icon(CupertinoIcons.chevron_left),
      ),
      trailing: CupertinoButton(
        padding: EdgeInsets.only(right: 16),
        onPressed: () {
          _apply(
            context,
            urlController.text,
            usernameController.text,
            passwordController.text,
            tlsInsecureSkipVerify.value,
          );
        },
        child: Text('应用'),
      ),
    );
  }

  void _apply(
    BuildContext context,
    String url,
    String username,
    String password,
    bool tlsInsecureSkipVerify,
  ) {
    final config = WebDavConfig(
      baseUrl: url,
      username: username,
      password: password,
      tlsInsecureSkipVerify: tlsInsecureSkipVerify,
    );

    final driver = WebDavSyncDriver(config);

    Navigator.of(context).push(
      CupertinoPageRoute(
        builder: (context) => FilePicker(fileSystemProvider: driver),
      ),
    );
  }
}
