import 'dart:async';

import 'package:flutter/cupertino.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/services/sync/webdav/webdav.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:keepass_one/widgets/file_picker/file_picker.dart';

class WebdavSettingsPage extends HookWidget {
  const WebdavSettingsPage({super.key, this.onFileSelect});

  final FutureOr<bool> Function(String)? onFileSelect;

  @override
  Widget build(BuildContext context) {
    final formKey = useMemoized(() => GlobalKey<FormState>());
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
        formKey,
        urlController,
        usernameController,
        passwordController,
        tlsInsecureSkipVerify,
      ),
      child: SafeArea(
        child: Container(
          color: CupertinoColors.systemGroupedBackground,
          child: Form(
            key: formKey,
            child: CupertinoListSection.insetGrouped(
              children: [
                CupertinoTextFormFieldRow(
                  controller: urlController,
                  prefix: Icon(CupertinoIcons.link),
                  placeholder: 'URL',
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return '请输入URL';
                    }
                    // 验证URL格式
                    final uri = Uri.tryParse(value.trim());
                    if (uri == null ||
                        !uri.hasScheme ||
                        (!uri.scheme.startsWith('http'))) {
                      return '请输入有效的URL（如：http://example.com）';
                    }
                    return null;
                  },
                ),
                CupertinoTextFormFieldRow(
                  controller: usernameController,
                  prefix: Icon(CupertinoIcons.person),
                  placeholder: '用户名（可选）',
                  validator: (value) {
                    final password = passwordController.text;
                    // 如果用户名为空但密码不为空，提示需要用户名
                    if ((value == null || value.isEmpty) &&
                        password.isNotEmpty) {
                      return '输入密码时必须提供用户名';
                    }
                    return null;
                  },
                ),
                CupertinoTextFormFieldRow(
                  controller: passwordController,
                  prefix: Text('密码'),
                  obscureText: true,
                  placeholder: '密码（可选）',
                  validator: (value) {
                    final username = usernameController.text;
                    // 如果密码为空但用户名不为空，提示需要密码
                    if ((value == null || value.isEmpty) &&
                        username.isNotEmpty) {
                      return '输入用户名时必须提供密码';
                    }
                    return null;
                  },
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
    GlobalKey<FormState> formKey,
    TextEditingController urlController,
    TextEditingController usernameController,
    TextEditingController passwordController,
    ValueNotifier<bool> tlsInsecureSkipVerify,
  ) {
    return CupertinoNavigationBar(
      middle: Text('WebDAV设置'),
      padding: .zero,
      leading: CupertinoButton(
        onPressed: () {
          Navigator.of(context, rootNavigator: true).pop();
        },
        padding: .zero,
        child: Icon(CupertinoIcons.chevron_left),
      ),
      trailing: CupertinoButton(
        padding: .only(right: 16),
        onPressed: () {
          // 在应用前验证表单
          if (formKey.currentState?.validate() ?? false) {
            _apply(
              context,
              urlController.text,
              usernameController.text,
              passwordController.text,
              tlsInsecureSkipVerify.value,
            );
          }
        },
        child: Text('应用'),
      ),
    );
  }

  Future<void> _apply(
    BuildContext context,
    String url,
    String username,
    String password,
    bool tlsInsecureSkipVerify,
  ) async {
    final config = WebDavConfig(
      baseUrl: url,
      username: username,
      password: password,
      tlsInsecureSkipVerify: tlsInsecureSkipVerify,
    );

    final driver = WebDavSyncDriver(config);

    final path = await Navigator.of(context).push<String>(
      CupertinoPageRoute(
        builder: (context) =>
            FilePicker(fileSystemProvider: driver, onFileSelect: onFileSelect),
      ),
    );

    if (path == null) {
      return;
    }

    if (context.mounted) {
      Navigator.of(context).pop(
        WebDavConfig(
          baseUrl: url,
          filePath: path,
          username: username,
          password: password,
          tlsInsecureSkipVerify: tlsInsecureSkipVerify,
        ),
      );
    }
  }
}
