// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'webdav_config.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WebDavConfigBasic _$WebDavConfigBasicFromJson(Map<String, dynamic> json) =>
    WebDavConfigBasic(
      baseUrl: json['baseUrl'] as String,
      username: json['username'] as String,
      password: json['password'] as String,
      tlsInsecureSkipVerify: json['tlsInsecureSkipVerify'] as bool,
      filePath: json['filePath'] as String?,
      $type: json['runtimeType'] as String?,
    );

Map<String, dynamic> _$WebDavConfigBasicToJson(WebDavConfigBasic instance) =>
    <String, dynamic>{
      'baseUrl': instance.baseUrl,
      'username': instance.username,
      'password': instance.password,
      'tlsInsecureSkipVerify': instance.tlsInsecureSkipVerify,
      'filePath': ?instance.filePath,
      'runtimeType': instance.$type,
    };

WebDavConfigToken _$WebDavConfigTokenFromJson(Map<String, dynamic> json) =>
    WebDavConfigToken(
      baseUrl: json['baseUrl'] as String,
      token: json['token'] as String,
      tlsInsecureSkipVerify: json['tlsInsecureSkipVerify'] as bool,
      filePath: json['filePath'] as String?,
      $type: json['runtimeType'] as String?,
    );

Map<String, dynamic> _$WebDavConfigTokenToJson(WebDavConfigToken instance) =>
    <String, dynamic>{
      'baseUrl': instance.baseUrl,
      'token': instance.token,
      'tlsInsecureSkipVerify': instance.tlsInsecureSkipVerify,
      'filePath': ?instance.filePath,
      'runtimeType': instance.$type,
    };

WebDavConfigNone _$WebDavConfigNoneFromJson(Map<String, dynamic> json) =>
    WebDavConfigNone(
      baseUrl: json['baseUrl'] as String,
      tlsInsecureSkipVerify: json['tlsInsecureSkipVerify'] as bool,
      filePath: json['filePath'] as String?,
      $type: json['runtimeType'] as String?,
    );

Map<String, dynamic> _$WebDavConfigNoneToJson(WebDavConfigNone instance) =>
    <String, dynamic>{
      'baseUrl': instance.baseUrl,
      'tlsInsecureSkipVerify': instance.tlsInsecureSkipVerify,
      'filePath': ?instance.filePath,
      'runtimeType': instance.$type,
    };
