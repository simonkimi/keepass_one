import 'file_system_models.dart';

/// 文件系统提供者抽象接口
///
/// 定义了与文件系统交互的基本操作，允许不同的文件系统实现（本地、WebDAV等）
abstract interface class FileSystemProvider {
  /// 获取指定路径下的文件和目录列表
  ///
  /// [path] 要列出内容的目录路径
  /// 返回该目录下的文件和子目录列表
  Future<List<FileSystemEntity>> listDirectory(String path);

  /// 获取根路径
  ///
  /// 返回此文件系统的根路径
  String getRootPath();
}
