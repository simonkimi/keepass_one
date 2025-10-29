// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'local_config.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$LocalConfig {

 String get path;
/// Create a copy of LocalConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LocalConfigCopyWith<LocalConfig> get copyWith => _$LocalConfigCopyWithImpl<LocalConfig>(this as LocalConfig, _$identity);

  /// Serializes this LocalConfig to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LocalConfig&&(identical(other.path, path) || other.path == path));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,path);

@override
String toString() {
  return 'LocalConfig(path: $path)';
}


}

/// @nodoc
abstract mixin class $LocalConfigCopyWith<$Res>  {
  factory $LocalConfigCopyWith(LocalConfig value, $Res Function(LocalConfig) _then) = _$LocalConfigCopyWithImpl;
@useResult
$Res call({
 String path
});




}
/// @nodoc
class _$LocalConfigCopyWithImpl<$Res>
    implements $LocalConfigCopyWith<$Res> {
  _$LocalConfigCopyWithImpl(this._self, this._then);

  final LocalConfig _self;
  final $Res Function(LocalConfig) _then;

/// Create a copy of LocalConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? path = null,}) {
  return _then(_self.copyWith(
path: null == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [LocalConfig].
extension LocalConfigPatterns on LocalConfig {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _LocalConfig value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _LocalConfig() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _LocalConfig value)  $default,){
final _that = this;
switch (_that) {
case _LocalConfig():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _LocalConfig value)?  $default,){
final _that = this;
switch (_that) {
case _LocalConfig() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String path)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _LocalConfig() when $default != null:
return $default(_that.path);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String path)  $default,) {final _that = this;
switch (_that) {
case _LocalConfig():
return $default(_that.path);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String path)?  $default,) {final _that = this;
switch (_that) {
case _LocalConfig() when $default != null:
return $default(_that.path);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _LocalConfig extends LocalConfig {
  const _LocalConfig({required this.path}): super._();
  factory _LocalConfig.fromJson(Map<String, dynamic> json) => _$LocalConfigFromJson(json);

@override final  String path;

/// Create a copy of LocalConfig
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$LocalConfigCopyWith<_LocalConfig> get copyWith => __$LocalConfigCopyWithImpl<_LocalConfig>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$LocalConfigToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _LocalConfig&&(identical(other.path, path) || other.path == path));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,path);

@override
String toString() {
  return 'LocalConfig(path: $path)';
}


}

/// @nodoc
abstract mixin class _$LocalConfigCopyWith<$Res> implements $LocalConfigCopyWith<$Res> {
  factory _$LocalConfigCopyWith(_LocalConfig value, $Res Function(_LocalConfig) _then) = __$LocalConfigCopyWithImpl;
@override @useResult
$Res call({
 String path
});




}
/// @nodoc
class __$LocalConfigCopyWithImpl<$Res>
    implements _$LocalConfigCopyWith<$Res> {
  __$LocalConfigCopyWithImpl(this._self, this._then);

  final _LocalConfig _self;
  final $Res Function(_LocalConfig) _then;

/// Create a copy of LocalConfig
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? path = null,}) {
  return _then(_LocalConfig(
path: null == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
