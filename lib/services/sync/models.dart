/// 同步驱动模型类
///
/// 此文件定义了同步驱动使用的模型类，包括文件和目录的元信息

/// 基础实体（文件和目录的共同属性）
abstract class SyncEntity {
  /// 实体名称（不包含路径）
  final String name;

  /// 实体完整路径
  final String path;

  /// 最后修改时间
  final DateTime? lastModified;

  SyncEntity({required this.name, required this.path, this.lastModified});

  /// 是否为文件
  bool get isFile;

  /// 是否为目录
  bool get isDirectory;
}

/// 文件元信息
class SyncFile extends SyncEntity {
  /// 文件大小（字节）
  final int? size;

  /// 文件哈希值（可能是ETag、MD5等）
  final String? hash;

  /// 文件MIME类型
  final String? mimeType;

  SyncFile({
    required super.name,
    required super.path,
    super.lastModified,
    this.size,
    this.hash,
    this.mimeType,
  });

  @override
  bool get isFile => true;

  @override
  bool get isDirectory => false;

  @override
  String toString() =>
      'SyncFile{name: $name, path: $path, size: $size, hash: $hash}';
}

/// 目录元信息
class SyncDirectory extends SyncEntity {
  SyncDirectory({required super.name, required super.path, super.lastModified});

  @override
  bool get isFile => false;

  @override
  bool get isDirectory => true;

  @override
  String toString() => 'SyncDirectory{name: $name, path: $path}';
}

/// list方法的返回结果
class SyncListResult {
  /// 文件列表
  final List<SyncFile> files;

  /// 目录列表
  final List<SyncDirectory> directories;

  SyncListResult({required this.files, required this.directories});

  /// 获取所有实体（目录在前，文件在后）
  List<SyncEntity> get all => [...directories, ...files];

  @override
  String toString() =>
      'SyncListResult{directories: ${directories.length}, files: ${files.length}}';
}
