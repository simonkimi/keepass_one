/// 文件系统实体抽象基类
///
/// 表示文件系统中的一个项目（文件或目录）
class FileSystemEntity {
  /// 文件或目录名称
  final String name;

  /// 完整路径
  final String path;

  /// 是否为目录
  final bool isDirectory;

  /// 最后修改时间
  final DateTime? lastModified;

  /// 文件大小（字节）
  ///
  /// 对于目录，此值可能为null
  final int? size;

  /// 文件扩展名
  ///
  /// 对于目录，此值为null
  final String? extension;

  /// 是否为隐藏文件
  final bool isHidden;

  FileSystemEntity({
    required this.name,
    required this.path,
    required this.isDirectory,
    this.lastModified,
    this.size,
    this.extension,
    this.isHidden = false,
  });

  /// 创建文件实体
  factory FileSystemEntity.file({
    required String name,
    required String path,
    DateTime? lastModified,
    int? size,
    bool isHidden = false,
  }) {
    final ext = name.contains('.')
        ? name.substring(name.lastIndexOf('.'))
        : null;

    return FileSystemEntity(
      name: name,
      path: path,
      isDirectory: false,
      lastModified: lastModified,
      size: size,
      extension: ext,
      isHidden: isHidden,
    );
  }

  /// 创建目录实体
  factory FileSystemEntity.directory({
    required String name,
    required String path,
    DateTime? lastModified,
    bool isHidden = false,
  }) {
    return FileSystemEntity(
      name: name,
      path: path,
      isDirectory: true,
      lastModified: lastModified,
      size: null,
      extension: null,
      isHidden: isHidden,
    );
  }

  /// 判断文件是否为图片
  bool get isImage {
    if (extension == null) return false;
    final imageExtensions = [
      '.jpg',
      '.jpeg',
      '.png',
      '.gif',
      '.webp',
      '.bmp',
      '.heic',
    ];
    return imageExtensions.contains(extension!.toLowerCase());
  }

  /// 判断文件是否为文档
  bool get isDocument {
    if (extension == null) return false;
    final docExtensions = [
      '.pdf',
      '.doc',
      '.docx',
      '.xls',
      '.xlsx',
      '.ppt',
      '.pptx',
      '.txt',
      '.rtf',
      '.md',
    ];
    return docExtensions.contains(extension!.toLowerCase());
  }

  /// 判断文件是否为音频
  bool get isAudio {
    if (extension == null) return false;
    final audioExtensions = ['.mp3', '.wav', '.ogg', '.m4a', '.flac', '.aac'];
    return audioExtensions.contains(extension!.toLowerCase());
  }

  /// 判断文件是否为视频
  bool get isVideo {
    if (extension == null) return false;
    final videoExtensions = [
      '.mp4',
      '.mov',
      '.avi',
      '.mkv',
      '.wmv',
      '.flv',
      '.webm',
    ];
    return videoExtensions.contains(extension!.toLowerCase());
  }

  /// 判断文件是否为压缩文件
  bool get isArchive {
    if (extension == null) return false;
    final archiveExtensions = ['.zip', '.rar', '.7z', '.tar', '.gz', '.bz2'];
    return archiveExtensions.contains(extension!.toLowerCase());
  }

  /// 判断文件是否为KDBX文件
  bool get isKdbx => extension?.toLowerCase() == '.kdbx';

  @override
  String toString() =>
      'FileSystemEntity(name: $name, path: $path, isDirectory: $isDirectory)';
}
