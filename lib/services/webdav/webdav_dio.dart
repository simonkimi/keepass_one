import 'dart:io';

import 'package:dio/dio.dart';
import 'package:dio/io.dart';
import 'package:dio_http_formatter/dio_http_formatter.dart';
import 'package:keepass_one/services/webdav/auth.dart';

Dio createDio({
  required String baseUrl,
  WebdavAuth? auth,
  bool insecureSkipVerify = false,
}) {
  final dio = Dio(
    BaseOptions(
      connectTimeout: const Duration(seconds: 3),
      receiveTimeout: const Duration(seconds: 3),
      followRedirects: true,
      baseUrl: baseUrl,
    ),
  );
  if (insecureSkipVerify) {
    (dio.httpClientAdapter as IOHttpClientAdapter).createHttpClient = () {
      final httpClient = HttpClient();
      httpClient.badCertificateCallback =
          (X509Certificate cert, String host, int port) => true;
      return httpClient;
    };
  }
  dio.interceptors.add(HttpFormatter());
  if (auth != null) {
    dio.interceptors.add(WebdavAuthInterceptor(auth));
  }
  return dio;
}

class WebdavDio {
  WebdavDio({
    required String baseUrl,
    WebdavAuth? auth,
    bool insecureSkipVerify = false,
  }) : _dio = createDio(
         baseUrl: baseUrl,
         auth: auth,
         insecureSkipVerify: insecureSkipVerify,
       );

  final Dio _dio;

  Future<Response<String>> propfind(
    String url, {
    int? depth = 1,
    CancelToken? cancelToken,
  }) async {
    final response = await _dio.request<String>(
      url,
      options: Options(
        method: 'PROPFIND',
        headers: {
          'Depth': depth?.toString() ?? 'infinity',
          'Content-Type': 'application/xml;charset=UTF-8',
          'Accept': 'application/xml,text/xml',
          'Accept-charset': 'UTF-8',
        },
      ),
      cancelToken: cancelToken,
    );

    return response;
  }

  Dio get dio => _dio;
}

class WebdavAuthInterceptor implements Interceptor {
  final WebdavAuth auth;

  WebdavAuthInterceptor(this.auth);

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    handler.next(err);
  }

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    auth.onRequest(options, handler);
  }

  @override
  void onResponse(
    Response<dynamic> response,
    ResponseInterceptorHandler handler,
  ) {
    handler.next(response);
  }
}
