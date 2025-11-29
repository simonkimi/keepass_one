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
WebDavConfig _$WebDavConfigFromJson(
  Map<String, dynamic> json
) {
        switch (json['runtimeType']) {
                  case 'basic':
          return WebDavConfigBasic.fromJson(
            json
          );
                case 'token':
          return WebDavConfigToken.fromJson(
            json
          );
                case 'none':
          return WebDavConfigNone.fromJson(
            json
          );
        
          default:
            throw CheckedFromJsonException(
  json,
  'runtimeType',
  'WebDavConfig',
  'Invalid union type "${json['runtimeType']}"!'
);
        }
      
}

/// @nodoc
mixin _$WebDavConfig {

 String get baseUrl; bool get tlsInsecureSkipVerify; String? get filePath;
/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebDavConfigCopyWith<WebDavConfig> get copyWith => _$WebDavConfigCopyWithImpl<WebDavConfig>(this as WebDavConfig, _$identity);

  /// Serializes this WebDavConfig to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebDavConfig&&(identical(other.baseUrl, baseUrl) || other.baseUrl == baseUrl)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify)&&(identical(other.filePath, filePath) || other.filePath == filePath));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,baseUrl,tlsInsecureSkipVerify,filePath);

@override
String toString() {
  return 'WebDavConfig(baseUrl: $baseUrl, tlsInsecureSkipVerify: $tlsInsecureSkipVerify, filePath: $filePath)';
}


}

/// @nodoc
abstract mixin class $WebDavConfigCopyWith<$Res>  {
  factory $WebDavConfigCopyWith(WebDavConfig value, $Res Function(WebDavConfig) _then) = _$WebDavConfigCopyWithImpl;
@useResult
$Res call({
 String baseUrl, bool tlsInsecureSkipVerify, String? filePath
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
@pragma('vm:prefer-inline') @override $Res call({Object? baseUrl = null,Object? tlsInsecureSkipVerify = null,Object? filePath = freezed,}) {
  return _then(_self.copyWith(
baseUrl: null == baseUrl ? _self.baseUrl : baseUrl // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,filePath: freezed == filePath ? _self.filePath : filePath // ignore: cast_nullable_to_non_nullable
as String?,
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( WebDavConfigBasic value)?  basic,TResult Function( WebDavConfigToken value)?  token,TResult Function( WebDavConfigNone value)?  none,required TResult orElse(),}){
final _that = this;
switch (_that) {
case WebDavConfigBasic() when basic != null:
return basic(_that);case WebDavConfigToken() when token != null:
return token(_that);case WebDavConfigNone() when none != null:
return none(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( WebDavConfigBasic value)  basic,required TResult Function( WebDavConfigToken value)  token,required TResult Function( WebDavConfigNone value)  none,}){
final _that = this;
switch (_that) {
case WebDavConfigBasic():
return basic(_that);case WebDavConfigToken():
return token(_that);case WebDavConfigNone():
return none(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( WebDavConfigBasic value)?  basic,TResult? Function( WebDavConfigToken value)?  token,TResult? Function( WebDavConfigNone value)?  none,}){
final _that = this;
switch (_that) {
case WebDavConfigBasic() when basic != null:
return basic(_that);case WebDavConfigToken() when token != null:
return token(_that);case WebDavConfigNone() when none != null:
return none(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String baseUrl,  String username,  String password,  bool tlsInsecureSkipVerify,  String? filePath)?  basic,TResult Function( String baseUrl,  String token,  bool tlsInsecureSkipVerify,  String? filePath)?  token,TResult Function( String baseUrl,  bool tlsInsecureSkipVerify,  String? filePath)?  none,required TResult orElse(),}) {final _that = this;
switch (_that) {
case WebDavConfigBasic() when basic != null:
return basic(_that.baseUrl,_that.username,_that.password,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigToken() when token != null:
return token(_that.baseUrl,_that.token,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigNone() when none != null:
return none(_that.baseUrl,_that.tlsInsecureSkipVerify,_that.filePath);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String baseUrl,  String username,  String password,  bool tlsInsecureSkipVerify,  String? filePath)  basic,required TResult Function( String baseUrl,  String token,  bool tlsInsecureSkipVerify,  String? filePath)  token,required TResult Function( String baseUrl,  bool tlsInsecureSkipVerify,  String? filePath)  none,}) {final _that = this;
switch (_that) {
case WebDavConfigBasic():
return basic(_that.baseUrl,_that.username,_that.password,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigToken():
return token(_that.baseUrl,_that.token,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigNone():
return none(_that.baseUrl,_that.tlsInsecureSkipVerify,_that.filePath);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String baseUrl,  String username,  String password,  bool tlsInsecureSkipVerify,  String? filePath)?  basic,TResult? Function( String baseUrl,  String token,  bool tlsInsecureSkipVerify,  String? filePath)?  token,TResult? Function( String baseUrl,  bool tlsInsecureSkipVerify,  String? filePath)?  none,}) {final _that = this;
switch (_that) {
case WebDavConfigBasic() when basic != null:
return basic(_that.baseUrl,_that.username,_that.password,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigToken() when token != null:
return token(_that.baseUrl,_that.token,_that.tlsInsecureSkipVerify,_that.filePath);case WebDavConfigNone() when none != null:
return none(_that.baseUrl,_that.tlsInsecureSkipVerify,_that.filePath);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class WebDavConfigBasic extends WebDavConfig {
  const WebDavConfigBasic({required this.baseUrl, required this.username, required this.password, required this.tlsInsecureSkipVerify, this.filePath, final  String? $type}): $type = $type ?? 'basic',super._();
  factory WebDavConfigBasic.fromJson(Map<String, dynamic> json) => _$WebDavConfigBasicFromJson(json);

@override final  String baseUrl;
 final  String username;
 final  String password;
@override final  bool tlsInsecureSkipVerify;
@override final  String? filePath;

@JsonKey(name: 'runtimeType')
final String $type;


/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebDavConfigBasicCopyWith<WebDavConfigBasic> get copyWith => _$WebDavConfigBasicCopyWithImpl<WebDavConfigBasic>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$WebDavConfigBasicToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebDavConfigBasic&&(identical(other.baseUrl, baseUrl) || other.baseUrl == baseUrl)&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify)&&(identical(other.filePath, filePath) || other.filePath == filePath));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,baseUrl,username,password,tlsInsecureSkipVerify,filePath);

@override
String toString() {
  return 'WebDavConfig.basic(baseUrl: $baseUrl, username: $username, password: $password, tlsInsecureSkipVerify: $tlsInsecureSkipVerify, filePath: $filePath)';
}


}

/// @nodoc
abstract mixin class $WebDavConfigBasicCopyWith<$Res> implements $WebDavConfigCopyWith<$Res> {
  factory $WebDavConfigBasicCopyWith(WebDavConfigBasic value, $Res Function(WebDavConfigBasic) _then) = _$WebDavConfigBasicCopyWithImpl;
@override @useResult
$Res call({
 String baseUrl, String username, String password, bool tlsInsecureSkipVerify, String? filePath
});




}
/// @nodoc
class _$WebDavConfigBasicCopyWithImpl<$Res>
    implements $WebDavConfigBasicCopyWith<$Res> {
  _$WebDavConfigBasicCopyWithImpl(this._self, this._then);

  final WebDavConfigBasic _self;
  final $Res Function(WebDavConfigBasic) _then;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? baseUrl = null,Object? username = null,Object? password = null,Object? tlsInsecureSkipVerify = null,Object? filePath = freezed,}) {
  return _then(WebDavConfigBasic(
baseUrl: null == baseUrl ? _self.baseUrl : baseUrl // ignore: cast_nullable_to_non_nullable
as String,username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,filePath: freezed == filePath ? _self.filePath : filePath // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc
@JsonSerializable()

class WebDavConfigToken extends WebDavConfig {
  const WebDavConfigToken({required this.baseUrl, required this.token, required this.tlsInsecureSkipVerify, this.filePath, final  String? $type}): $type = $type ?? 'token',super._();
  factory WebDavConfigToken.fromJson(Map<String, dynamic> json) => _$WebDavConfigTokenFromJson(json);

@override final  String baseUrl;
 final  String token;
@override final  bool tlsInsecureSkipVerify;
@override final  String? filePath;

@JsonKey(name: 'runtimeType')
final String $type;


/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebDavConfigTokenCopyWith<WebDavConfigToken> get copyWith => _$WebDavConfigTokenCopyWithImpl<WebDavConfigToken>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$WebDavConfigTokenToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebDavConfigToken&&(identical(other.baseUrl, baseUrl) || other.baseUrl == baseUrl)&&(identical(other.token, token) || other.token == token)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify)&&(identical(other.filePath, filePath) || other.filePath == filePath));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,baseUrl,token,tlsInsecureSkipVerify,filePath);

@override
String toString() {
  return 'WebDavConfig.token(baseUrl: $baseUrl, token: $token, tlsInsecureSkipVerify: $tlsInsecureSkipVerify, filePath: $filePath)';
}


}

/// @nodoc
abstract mixin class $WebDavConfigTokenCopyWith<$Res> implements $WebDavConfigCopyWith<$Res> {
  factory $WebDavConfigTokenCopyWith(WebDavConfigToken value, $Res Function(WebDavConfigToken) _then) = _$WebDavConfigTokenCopyWithImpl;
@override @useResult
$Res call({
 String baseUrl, String token, bool tlsInsecureSkipVerify, String? filePath
});




}
/// @nodoc
class _$WebDavConfigTokenCopyWithImpl<$Res>
    implements $WebDavConfigTokenCopyWith<$Res> {
  _$WebDavConfigTokenCopyWithImpl(this._self, this._then);

  final WebDavConfigToken _self;
  final $Res Function(WebDavConfigToken) _then;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? baseUrl = null,Object? token = null,Object? tlsInsecureSkipVerify = null,Object? filePath = freezed,}) {
  return _then(WebDavConfigToken(
baseUrl: null == baseUrl ? _self.baseUrl : baseUrl // ignore: cast_nullable_to_non_nullable
as String,token: null == token ? _self.token : token // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,filePath: freezed == filePath ? _self.filePath : filePath // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

/// @nodoc
@JsonSerializable()

class WebDavConfigNone extends WebDavConfig {
  const WebDavConfigNone({required this.baseUrl, required this.tlsInsecureSkipVerify, this.filePath, final  String? $type}): $type = $type ?? 'none',super._();
  factory WebDavConfigNone.fromJson(Map<String, dynamic> json) => _$WebDavConfigNoneFromJson(json);

@override final  String baseUrl;
@override final  bool tlsInsecureSkipVerify;
@override final  String? filePath;

@JsonKey(name: 'runtimeType')
final String $type;


/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebDavConfigNoneCopyWith<WebDavConfigNone> get copyWith => _$WebDavConfigNoneCopyWithImpl<WebDavConfigNone>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$WebDavConfigNoneToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebDavConfigNone&&(identical(other.baseUrl, baseUrl) || other.baseUrl == baseUrl)&&(identical(other.tlsInsecureSkipVerify, tlsInsecureSkipVerify) || other.tlsInsecureSkipVerify == tlsInsecureSkipVerify)&&(identical(other.filePath, filePath) || other.filePath == filePath));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,baseUrl,tlsInsecureSkipVerify,filePath);

@override
String toString() {
  return 'WebDavConfig.none(baseUrl: $baseUrl, tlsInsecureSkipVerify: $tlsInsecureSkipVerify, filePath: $filePath)';
}


}

/// @nodoc
abstract mixin class $WebDavConfigNoneCopyWith<$Res> implements $WebDavConfigCopyWith<$Res> {
  factory $WebDavConfigNoneCopyWith(WebDavConfigNone value, $Res Function(WebDavConfigNone) _then) = _$WebDavConfigNoneCopyWithImpl;
@override @useResult
$Res call({
 String baseUrl, bool tlsInsecureSkipVerify, String? filePath
});




}
/// @nodoc
class _$WebDavConfigNoneCopyWithImpl<$Res>
    implements $WebDavConfigNoneCopyWith<$Res> {
  _$WebDavConfigNoneCopyWithImpl(this._self, this._then);

  final WebDavConfigNone _self;
  final $Res Function(WebDavConfigNone) _then;

/// Create a copy of WebDavConfig
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? baseUrl = null,Object? tlsInsecureSkipVerify = null,Object? filePath = freezed,}) {
  return _then(WebDavConfigNone(
baseUrl: null == baseUrl ? _self.baseUrl : baseUrl // ignore: cast_nullable_to_non_nullable
as String,tlsInsecureSkipVerify: null == tlsInsecureSkipVerify ? _self.tlsInsecureSkipVerify : tlsInsecureSkipVerify // ignore: cast_nullable_to_non_nullable
as bool,filePath: freezed == filePath ? _self.filePath : filePath // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

// dart format on
