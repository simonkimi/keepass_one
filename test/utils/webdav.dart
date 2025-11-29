import 'package:keepass_one/services/webdav/auth.dart';
import 'package:keepass_one/services/webdav/webdav_client.dart';

Future<void> main() async {
  final url = 'https://webdav.z31.ink:20443/webdav/';
  final username = 'backup';
  final password = '2CYj8_G8yt_NJdZg9_Wc';

  final client = WebdavClient(
    baseUrl: url,
    auth: WebdavBasicAuth(username: username, password: password),
  );

  final bytes = await client.readDir("");
  print(bytes);
}
