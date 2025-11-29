import 'package:keepass_one/services/webdav/auth.dart';
import 'package:keepass_one/services/webdav/file.dart';
import 'package:keepass_one/services/webdav/webdav_dio.dart';
import 'package:keepass_one/services/webdav/xml.dart';

class WebdavClient {
  final WebdavDio _webdavDio;

  WebdavClient({
    required String baseUrl,
    WebdavAuth? auth,
    bool insecureSkipVerify = false,
  }) : _webdavDio = WebdavDio(
         baseUrl: baseUrl,
         auth: auth,
         insecureSkipVerify: insecureSkipVerify,
       );

  Future<List<WebdavEntiry>> readDir(String path) async {
    path = _normalizePath(path);
    final response = await _webdavDio.propfind(path, depth: 1);
    return parsePropfindXml(path, response.data ?? '');
  }
}

String _normalizePath(String path) {
  if (!path.startsWith("/")) {
    path = "/$path";
  }
  if (!path.endsWith("/")) {
    path = "$path/";
  }
  return path;
}
