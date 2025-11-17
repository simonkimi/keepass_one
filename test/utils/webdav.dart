import 'package:webdav_client/webdav_client.dart';

Future<void> main() async {
  final url =
      'https://webdav.z31.ink:20443/webdav/';
  final username = 'backup';
  final password = '2CYj8_G8yt_NJdZg9_Wc';

  final client = newClient(url, user: username, password: password);

  final bytes = await client.read("/backup/tabby/tabby-settings.json");
  print(bytes);
}
