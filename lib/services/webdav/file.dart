import 'package:freezed_annotation/freezed_annotation.dart';

part 'file.freezed.dart';

@freezed
sealed class WebdavEntiry with _$WebdavEntiry {
  const factory WebdavEntiry.file({
    required String? name,
    required String? path,
    required int? size,
    required String? contentType,
    required String? etag,
    required DateTime? createdAt,
    required DateTime? lastModified,
  }) = WebdavEntiryFile;

  const factory WebdavEntiry.directory({
    required String? name,
    required String? path,
    required DateTime? lastModified,
    required DateTime? createdTime,
    required String? etag,
  }) = WebdavEntiryDirectory;
}
