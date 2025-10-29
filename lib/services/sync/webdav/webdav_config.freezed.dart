// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'webdav_config.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$WebDavConfig {

 String get url;/// 用户名
 String get username;/// 密码
 String get password;/// 是否跳过TLS证书验证
 bool get tlsInsecureSkipVerify;
/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebDavConfigCopyWith<WebDavConfig> get copyWith => _$WebDavConfigCopyWithImpl<WebDavConfig>(this as WebDavConfig, _$identity);

  /// Serializes this WebDavConfig to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebDavConfig&&(identical(other.url, url) || other.url == url)&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,url,username,password,tlsInsecureSkipVerify);

@override
String toString() {
  return 'WebDavConfig(url: $url, username: $username, password: $password, tlsInsecureSkipVerify: $tlsInsecureSkipVerify)';
}


}

/// @nodoc
abstract mixin class $WebDavConfigCopyWith<$Res>  {
  factory $WebDavConfigCopyWith(WebDavConfig value, $Res Function(WebDavConfig) _then) = _$WebDavConfigCopyWithImpl;
@useResult
$Res call({
 String url, String username, String password, bool tlsInsecureSkipVerify
});




}
/// @nodoc
class _$WebDavConfigCopyWithImpl<$Res>
    implements $WebDavConfigCopyWith<$Res> {
  _$WebDavConfigCopyWithImpl(this._self, this._then);

  final WebDavConfig _self;
  final $Res Function(WebDavConfig) _then;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? url = null,Object? username = null,Object? password = null,Object? tlsInsecureSkipVerify = null,}) {
  return _then(_self.copyWith(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [WebDavConfig].
extension WebDavConfigPatterns on WebDavConfig {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _WebDavConfig value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _WebDavConfig() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _WebDavConfig value)  $default,){
final _that = this;
switch (_that) {
case _WebDavConfig():
return $default(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _WebDavConfig value)?  $default,){
final _that = this;
switch (_that) {
case _WebDavConfig() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String url,  String username,  String password,  bool tlsInsecureSkipVerify)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _WebDavConfig() when $default != null:
return $default(_that.url,_that.username,_that.password,_that.tlsInsecureSkipVerify);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String url,  String username,  String password,  bool tlsInsecureSkipVerify)  $default,) {final _that = this;
switch (_that) {
case _WebDavConfig():
return $default(_that.url,_that.username,_that.password,_that.tlsInsecureSkipVerify);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String url,  String username,  String password,  bool tlsInsecureSkipVerify)?  $default,) {final _that = this;
switch (_that) {
case _WebDavConfig() when $default != null:
return $default(_that.url,_that.username,_that.password,_that.tlsInsecureSkipVerify);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _WebDavConfig extends WebDavConfig {
  const _WebDavConfig({required this.url, required this.username, required this.password, required this.tlsInsecureSkipVerify}): super._();
  factory _WebDavConfig.fromJson(Map<String, dynamic> json) => _$WebDavConfigFromJson(json);

@override final  String url;
/// 用户名
@override final  String username;
/// 密码
@override final  String password;
/// 是否跳过TLS证书验证
@override final  bool tlsInsecureSkipVerify;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$WebDavConfigCopyWith<_WebDavConfig> get copyWith => __$WebDavConfigCopyWithImpl<_WebDavConfig>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$WebDavConfigToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _WebDavConfig&&(identical(other.url, url) || other.url == url)&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,url,username,password,tlsInsecureSkipVerify);

@override
String toString() {
  return 'WebDavConfig(url: $url, username: $username, password: $password, tlsInsecureSkipVerify: $tlsInsecureSkipVerify)';
}


}

/// @nodoc
abstract mixin class _$WebDavConfigCopyWith<$Res> implements $WebDavConfigCopyWith<$Res> {
  factory _$WebDavConfigCopyWith(_WebDavConfig value, $Res Function(_WebDavConfig) _then) = __$WebDavConfigCopyWithImpl;
@override @useResult
$Res call({
 String url, String username, String password, bool tlsInsecureSkipVerify
});




}
/// @nodoc
class __$WebDavConfigCopyWithImpl<$Res>
    implements _$WebDavConfigCopyWith<$Res> {
  __$WebDavConfigCopyWithImpl(this._self, this._then);

  final _WebDavConfig _self;
  final $Res Function(_WebDavConfig) _then;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? url = null,Object? username = null,Object? password = null,Object? tlsInsecureSkipVerify = null,}) {
  return _then(_WebDavConfig(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

// dart format on
