// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'webdav_config.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_WebDavConfig _$WebDavConfigFromJson(Map<String, dynamic> json) =>
    _WebDavConfig(
      url: json['url'] as String,
      username: json['username'] as String,
      password: json['password'] as String,
      tlsInsecureSkipVerify: json['tlsInsecureSkipVerify'] as bool,
    );

Map<String, dynamic> _$WebDavConfigToJson(_WebDavConfig instance) =>
    <String, dynamic>{
      'url': instance.url,
      'username': instance.username,
      'password': instance.password,
      'tlsInsecureSkipVerify': instance.tlsInsecureSkipVerify,
    };
