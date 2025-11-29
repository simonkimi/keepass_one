// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'file.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$WebdavEntiry {

 String? get name; String? get path; String? get etag; DateTime? get lastModified;
/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebdavEntiryCopyWith<WebdavEntiry> get copyWith => _$WebdavEntiryCopyWithImpl<WebdavEntiry>(this as WebdavEntiry, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebdavEntiry&&(identical(other.name, name) || other.name == name)&&(identical(other.path, path) || other.path == path)&&(identical(other.etag, etag) || other.etag == etag)&&(identical(other.lastModified, lastModified) || other.lastModified == lastModified));
}


@override
int get hashCode => Object.hash(runtimeType,name,path,etag,lastModified);

@override
String toString() {
  return 'WebdavEntiry(name: $name, path: $path, etag: $etag, lastModified: $lastModified)';
}


}

/// @nodoc
abstract mixin class $WebdavEntiryCopyWith<$Res>  {
  factory $WebdavEntiryCopyWith(WebdavEntiry value, $Res Function(WebdavEntiry) _then) = _$WebdavEntiryCopyWithImpl;
@useResult
$Res call({
 String? name, String? path, String? etag, DateTime? lastModified
});




}
/// @nodoc
class _$WebdavEntiryCopyWithImpl<$Res>
    implements $WebdavEntiryCopyWith<$Res> {
  _$WebdavEntiryCopyWithImpl(this._self, this._then);

  final WebdavEntiry _self;
  final $Res Function(WebdavEntiry) _then;

/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = freezed,Object? path = freezed,Object? etag = freezed,Object? lastModified = freezed,}) {
  return _then(_self.copyWith(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,path: freezed == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String?,etag: freezed == etag ? _self.etag : etag // ignore: cast_nullable_to_non_nullable
as String?,lastModified: freezed == lastModified ? _self.lastModified : lastModified // ignore: cast_nullable_to_non_nullable
as DateTime?,
  ));
}

}


/// Adds pattern-matching-related methods to [WebdavEntiry].
extension WebdavEntiryPatterns on WebdavEntiry {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( WebdavEntiryFile value)?  file,TResult Function( WebdavEntiryDirectory value)?  directory,required TResult orElse(),}){
final _that = this;
switch (_that) {
case WebdavEntiryFile() when file != null:
return file(_that);case WebdavEntiryDirectory() when directory != null:
return directory(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( WebdavEntiryFile value)  file,required TResult Function( WebdavEntiryDirectory value)  directory,}){
final _that = this;
switch (_that) {
case WebdavEntiryFile():
return file(_that);case WebdavEntiryDirectory():
return directory(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( WebdavEntiryFile value)?  file,TResult? Function( WebdavEntiryDirectory value)?  directory,}){
final _that = this;
switch (_that) {
case WebdavEntiryFile() when file != null:
return file(_that);case WebdavEntiryDirectory() when directory != null:
return directory(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String? name,  String? path,  int? size,  String? contentType,  String? etag,  DateTime? createdAt,  DateTime? lastModified)?  file,TResult Function( String? name,  String? path,  DateTime? lastModified,  DateTime? createdTime,  String? etag)?  directory,required TResult orElse(),}) {final _that = this;
switch (_that) {
case WebdavEntiryFile() when file != null:
return file(_that.name,_that.path,_that.size,_that.contentType,_that.etag,_that.createdAt,_that.lastModified);case WebdavEntiryDirectory() when directory != null:
return directory(_that.name,_that.path,_that.lastModified,_that.createdTime,_that.etag);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String? name,  String? path,  int? size,  String? contentType,  String? etag,  DateTime? createdAt,  DateTime? lastModified)  file,required TResult Function( String? name,  String? path,  DateTime? lastModified,  DateTime? createdTime,  String? etag)  directory,}) {final _that = this;
switch (_that) {
case WebdavEntiryFile():
return file(_that.name,_that.path,_that.size,_that.contentType,_that.etag,_that.createdAt,_that.lastModified);case WebdavEntiryDirectory():
return directory(_that.name,_that.path,_that.lastModified,_that.createdTime,_that.etag);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String? name,  String? path,  int? size,  String? contentType,  String? etag,  DateTime? createdAt,  DateTime? lastModified)?  file,TResult? Function( String? name,  String? path,  DateTime? lastModified,  DateTime? createdTime,  String? etag)?  directory,}) {final _that = this;
switch (_that) {
case WebdavEntiryFile() when file != null:
return file(_that.name,_that.path,_that.size,_that.contentType,_that.etag,_that.createdAt,_that.lastModified);case WebdavEntiryDirectory() when directory != null:
return directory(_that.name,_that.path,_that.lastModified,_that.createdTime,_that.etag);case _:
  return null;

}
}

}

/// @nodoc


class WebdavEntiryFile implements WebdavEntiry {
  const WebdavEntiryFile({required this.name, required this.path, required this.size, required this.contentType, required this.etag, required this.createdAt, required this.lastModified});
  

@override final  String? name;
@override final  String? path;
 final  int? size;
 final  String? contentType;
@override final  String? etag;
 final  DateTime? createdAt;
@override final  DateTime? lastModified;

/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebdavEntiryFileCopyWith<WebdavEntiryFile> get copyWith => _$WebdavEntiryFileCopyWithImpl<WebdavEntiryFile>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebdavEntiryFile&&(identical(other.name, name) || other.name == name)&&(identical(other.path, path) || other.path == path)&&(identical(other.size, size) || other.size == size)&&(identical(other.contentType, contentType) || other.contentType == contentType)&&(identical(other.etag, etag) || other.etag == etag)&&(identical(other.createdAt, createdAt) || other.createdAt == createdAt)&&(identical(other.lastModified, lastModified) || other.lastModified == lastModified));
}


@override
int get hashCode => Object.hash(runtimeType,name,path,size,contentType,etag,createdAt,lastModified);

@override
String toString() {
  return 'WebdavEntiry.file(name: $name, path: $path, size: $size, contentType: $contentType, etag: $etag, createdAt: $createdAt, lastModified: $lastModified)';
}


}

/// @nodoc
abstract mixin class $WebdavEntiryFileCopyWith<$Res> implements $WebdavEntiryCopyWith<$Res> {
  factory $WebdavEntiryFileCopyWith(WebdavEntiryFile value, $Res Function(WebdavEntiryFile) _then) = _$WebdavEntiryFileCopyWithImpl;
@override @useResult
$Res call({
 String? name, String? path, int? size, String? contentType, String? etag, DateTime? createdAt, DateTime? lastModified
});




}
/// @nodoc
class _$WebdavEntiryFileCopyWithImpl<$Res>
    implements $WebdavEntiryFileCopyWith<$Res> {
  _$WebdavEntiryFileCopyWithImpl(this._self, this._then);

  final WebdavEntiryFile _self;
  final $Res Function(WebdavEntiryFile) _then;

/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = freezed,Object? path = freezed,Object? size = freezed,Object? contentType = freezed,Object? etag = freezed,Object? createdAt = freezed,Object? lastModified = freezed,}) {
  return _then(WebdavEntiryFile(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,path: freezed == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String?,size: freezed == size ? _self.size : size // ignore: cast_nullable_to_non_nullable
as int?,contentType: freezed == contentType ? _self.contentType : contentType // ignore: cast_nullable_to_non_nullable
as String?,etag: freezed == etag ? _self.etag : etag // ignore: cast_nullable_to_non_nullable
as String?,createdAt: freezed == createdAt ? _self.createdAt : createdAt // ignore: cast_nullable_to_non_nullable
as DateTime?,lastModified: freezed == lastModified ? _self.lastModified : lastModified // ignore: cast_nullable_to_non_nullable
as DateTime?,
  ));
}


}

/// @nodoc


class WebdavEntiryDirectory implements WebdavEntiry {
  const WebdavEntiryDirectory({required this.name, required this.path, required this.lastModified, required this.createdTime, required this.etag});
  

@override final  String? name;
@override final  String? path;
@override final  DateTime? lastModified;
 final  DateTime? createdTime;
@override final  String? etag;

/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WebdavEntiryDirectoryCopyWith<WebdavEntiryDirectory> get copyWith => _$WebdavEntiryDirectoryCopyWithImpl<WebdavEntiryDirectory>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WebdavEntiryDirectory&&(identical(other.name, name) || other.name == name)&&(identical(other.path, path) || other.path == path)&&(identical(other.lastModified, lastModified) || other.lastModified == lastModified)&&(identical(other.createdTime, createdTime) || other.createdTime == createdTime)&&(identical(other.etag, etag) || other.etag == etag));
}


@override
int get hashCode => Object.hash(runtimeType,name,path,lastModified,createdTime,etag);

@override
String toString() {
  return 'WebdavEntiry.directory(name: $name, path: $path, lastModified: $lastModified, createdTime: $createdTime, etag: $etag)';
}


}

/// @nodoc
abstract mixin class $WebdavEntiryDirectoryCopyWith<$Res> implements $WebdavEntiryCopyWith<$Res> {
  factory $WebdavEntiryDirectoryCopyWith(WebdavEntiryDirectory value, $Res Function(WebdavEntiryDirectory) _then) = _$WebdavEntiryDirectoryCopyWithImpl;
@override @useResult
$Res call({
 String? name, String? path, DateTime? lastModified, DateTime? createdTime, String? etag
});




}
/// @nodoc
class _$WebdavEntiryDirectoryCopyWithImpl<$Res>
    implements $WebdavEntiryDirectoryCopyWith<$Res> {
  _$WebdavEntiryDirectoryCopyWithImpl(this._self, this._then);

  final WebdavEntiryDirectory _self;
  final $Res Function(WebdavEntiryDirectory) _then;

/// Create a copy of WebdavEntiry
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = freezed,Object? path = freezed,Object? lastModified = freezed,Object? createdTime = freezed,Object? etag = freezed,}) {
  return _then(WebdavEntiryDirectory(
name: freezed == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String?,path: freezed == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String?,lastModified: freezed == lastModified ? _self.lastModified : lastModified // ignore: cast_nullable_to_non_nullable
as DateTime?,createdTime: freezed == createdTime ? _self.createdTime : createdTime // ignore: cast_nullable_to_non_nullable
as DateTime?,etag: freezed == etag ? _self.etag : etag // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}

// dart format on
