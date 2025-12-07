import 'package:dio/dio.dart';
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
    path = _normalizeDirPath(path);
    final response = await _webdavDio.propfind(path, depth: 1);
    return parsePropfindXml(path, response.data ?? '');
  }

  Future<WebdavEntiry?> readEntity(String path) async {
    path = _normalizeFilePath(path);
    final response = await _webdavDio.dio.get(path);
    final entities = parsePropfindXml(path, response.data ?? '');
    return entities.firstOrNull;
  }

  Future<void> downloadFile(
    String path,
    String savePath, {
    ProgressCallback? onProgress,
    CancelToken? cancelToken,
  }) async {
    path = _normalizeFilePath(path);
    await _webdavDio.dio.download(
      path,
      savePath,
      onReceiveProgress: onProgress,
      cancelToken: cancelToken,
    );
  }
}

String _normalizeDirPath(String path) {
  if (!path.startsWith("/")) {
    path = "/$path";
  }
  if (!path.endsWith("/")) {
    path = "$path/";
  }
  return path;
}

String _normalizeFilePath(String path) {
  if (!path.startsWith("/")) {
    path = "/$path";
  }
  if (path.endsWith("/")) {
    path = path.substring(0, path.length - 1);
  }
  return path;
}
