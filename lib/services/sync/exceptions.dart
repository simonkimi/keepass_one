/// 同步驱动异常类
///
/// 此文件定义了同步驱动可能抛出的所有异常类型，用于统一处理不同驱动的异常

/// 基础同步异常
abstract class SyncException implements Exception {
  final String message;
  final String? details;
  final Object? originalError;

  SyncException(this.message, {this.details, this.originalError});

  @override
  String toString() {
    if (details != null) {
      return '$runtimeType: $message (Details: $details)';
    }
    return '$runtimeType: $message';
  }
}

/// 网络连接异常
///
/// 当无法连接到远程服务器时抛出
class SyncConnectionException extends SyncException {
  SyncConnectionException(super.message, {super.details, super.originalError});
}

/// 认证失败异常
///
/// 当用户凭据无效或认证过程失败时抛出
class SyncAuthenticationException extends SyncException {
  SyncAuthenticationException(
    super.message, {
    super.details,
    super.originalError,
  });
}

/// 文件未找到异常
///
/// 当请求的文件或目录不存在时抛出
class SyncNotFoundException extends SyncException {
  SyncNotFoundException(super.message, {super.details, super.originalError});
}

/// 权限不足异常
///
/// 当用户没有足够权限执行操作时抛出
class SyncPermissionException extends SyncException {
  SyncPermissionException(super.message, {super.details, super.originalError});
}

/// 超时异常
///
/// 当操作超时时抛出
class SyncTimeoutException extends SyncException {
  SyncTimeoutException(super.message, {super.details, super.originalError});
}

/// 通用IO异常
///
/// 当发生其他IO相关错误时抛出
class SyncIOException extends SyncException {
  SyncIOException(super.message, {super.details, super.originalError});
}

/// 配置错误异常
///
/// 当驱动配置不正确或缺少必要参数时抛出
class SyncConfigException extends SyncException {
  SyncConfigException(super.message, {super.details, super.originalError});
}
