import 'dart:convert';

import 'package:dio/dio.dart';

abstract class WebdavAuth {
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {}
}

class WebdavBasicAuth implements WebdavAuth {
  final String username;
  final String password;

  final String authToken;

  WebdavBasicAuth({required this.username, required this.password})
    : authToken = 'Basic ${base64Encode(utf8.encode('$username:$password'))}';

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    options.headers['Authorization'] = authToken;
    handler.next(options);
  }
}

class WebdavTokenAuth implements WebdavAuth {
  final String token;

  WebdavTokenAuth({required this.token});

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    options.headers['Authorization'] = token;
    handler.next(options);
  }
}

class WebdavNoneAuth implements WebdavAuth {
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {}
}
