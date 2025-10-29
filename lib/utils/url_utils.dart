/// URL 拼接工具类
///
/// 提供统一的方法来拼接 URL 和 path，自动处理斜杠问题
class UrlUtils {
  UrlUtils._();
  static String join(
    String baseUrl, [
    String? path,
    List<String>? additionalPaths,
  ]) {
    var result = baseUrl.trim();
    if (result.endsWith('/')) {
      result = result.substring(0, result.length - 1);
    }

    final allPaths = <String>[];
    if (path != null && path.isNotEmpty) {
      allPaths.add(path);
    }
    if (additionalPaths != null) {
      allPaths.addAll(additionalPaths);
    }

    for (var i = 0; i < allPaths.length; i++) {
      final segment = allPaths[i];
      final trimmed = segment.trim();
      if (trimmed.isEmpty) continue;

      var normalized = trimmed;
      while (normalized.startsWith('/')) {
        normalized = normalized.substring(1);
      }

      final isLastSegment = i == allPaths.length - 1;
      if (!isLastSegment && normalized.endsWith('/')) {
        normalized = normalized.substring(0, normalized.length - 1);
      }

      if (normalized.isNotEmpty) {
        result = '$result/$normalized';
      }
    }

    return result;
  }
}
