import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:keepass_one/services/sync/webdav/webdav.dart';
import 'package:keepass_one/services/sync/webdav/webdav_config.dart';

Future<void> main() async {
  test('test sync webdav', () async {
    await dotenv.load(fileName: '.env');

    final config = WebDavConfig(
      baseUrl: dotenv.env['WEBDAV_BASE_URL']!,
      username: dotenv.env['WEBDAV_USERNAME']!,
      password: dotenv.env['WEBDAV_PASSWORD']!,
      tlsInsecureSkipVerify: true,
    );

    final driver = WebDavSyncDriver(config);

    final list = await driver.list('/share/');
    for (final item in list.directories) {
      print('Directory: $item');
    }
    for (final item in list.files) {
      print('File: $item');
    }
  });
}
