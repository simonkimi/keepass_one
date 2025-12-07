import 'package:keepass_one/services/webdav/auth.dart';
import 'package:keepass_one/services/webdav/file.dart';
import 'package:keepass_one/services/webdav/webdav_client.dart';

Future<void> main() async {
  final url = 'https://webdav.z31.ink:20443/webdav/';
  final username = 'backup';
  final password = '2CYj8_G8yt_NJdZg9_Wc';

  final client = WebdavClient(
    baseUrl: url,
    auth: WebdavBasicAuth(username: username, password: password),
  );

  final entities = await client.readDir("/");
  for (final entity in entities) {
    await walkEntity(client, entity);
  }
}

Future<void> walkEntity(WebdavClient client, WebdavEntiry entiry) async {
  switch (entiry) {
    case WebdavEntiryDirectory dir:
      print(dir);
      final entities = await client.readDir(dir.path ?? '');
      for (final entity in entities) {
        await walkEntity(client, entity);
      }
      break;
    case WebdavEntiryFile file:
      print(file.path);
      break;
  }
}
