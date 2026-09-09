import 'dart:async';

import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:keepass_one/services/sync/webdav/webdav.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';
import 'package:keepass_one/widgets/file_picker/file_picker.dart';
import 'package:material_ui/material_ui.dart';

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

    return Scaffold(
      appBar: AppBar(
        title: const Text('WebDAV设置'),
        leading: IconButton(
          onPressed: () {
            Navigator.of(context, rootNavigator: true).pop();
          },
          icon: const Icon(Icons.chevron_left),
        ),
        actions: [
          TextButton(
            onPressed: () {
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
            child: const Text('应用'),
          ),
        ],
      ),
      body: SafeArea(
        child: Form(
          key: formKey,
          child: ListView(
            padding: const EdgeInsets.all(16),
            children: [
              Card.filled(
                clipBehavior: Clip.antiAlias,
                child: Padding(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    children: [
                      TextFormField(
                        controller: urlController,
                        decoration: const InputDecoration(
                          prefixIcon: Icon(Icons.link_outlined),
                          hintText: 'URL',
                          border: InputBorder.none,
                        ),
                        validator: (value) {
                          if (value == null || value.isEmpty) {
                            return '请输入URL';
                          }
                          final uri = Uri.tryParse(value.trim());
                          if (uri == null ||
                              !uri.hasScheme ||
                              (!uri.scheme.startsWith('http'))) {
                            return '请输入有效的URL（如：http://example.com）';
                          }
                          return null;
                        },
                      ),
                      TextFormField(
                        controller: usernameController,
                        decoration: const InputDecoration(
                          prefixIcon: Icon(Icons.person_outline),
                          hintText: '用户名（可选）',
                          border: InputBorder.none,
                        ),
                        validator: (value) {
                          final password = passwordController.text;
                          if ((value == null || value.isEmpty) &&
                              password.isNotEmpty) {
                            return '输入密码时必须提供用户名';
                          }
                          return null;
                        },
                      ),
                      TextFormField(
                        controller: passwordController,
                        obscureText: true,
                        decoration: const InputDecoration(
                          prefixIcon: Icon(Icons.lock_outline),
                          hintText: '密码（可选）',
                          border: InputBorder.none,
                        ),
                        validator: (value) {
                          final username = usernameController.text;
                          if ((value == null || value.isEmpty) &&
                              username.isNotEmpty) {
                            return '输入用户名时必须提供密码';
                          }
                          return null;
                        },
                      ),
                      SwitchListTile(
                        contentPadding: EdgeInsets.zero,
                        title: const Text('跳过TLS验证'),
                        value: tlsInsecureSkipVerify.value,
                        onChanged: (value) {
                          tlsInsecureSkipVerify.value = value;
                        },
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
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
    final config = WebDavConfig.basic(
      baseUrl: url,
      username: username,
      password: password,
      tlsInsecureSkipVerify: tlsInsecureSkipVerify,
    );

    final driver = WebDavSyncDriver(config);

    final path = await Navigator.of(context).push<String>(
      MaterialPageRoute(
        builder: (context) =>
            FilePicker(fileSystemProvider: driver, onFileSelect: onFileSelect),
      ),
    );

    if (path == null) {
      return;
    }

    if (context.mounted) {
      Navigator.of(context).pop(
        WebDavConfig.basic(
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
