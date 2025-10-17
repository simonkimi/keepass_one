// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'webdav_config.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_WebDavConfig _$WebDavConfigFromJson(Map<String, dynamic> json) =>
    _WebDavConfig(
      baseUrl: json['baseUrl'] as String,
      username: json['username'] as String,
      password: json['password'] as String,
      tlsInsecureSkipVerify: json['tlsInsecureSkipVerify'] as bool,
    );

Map<String, dynamic> _$WebDavConfigToJson(_WebDavConfig instance) =>
    <String, dynamic>{
      'baseUrl': instance.baseUrl,
      'username': instance.username,
      'password': instance.password,
      'tlsInsecureSkipVerify': instance.tlsInsecureSkipVerify,
    };
