import 'package:flutter_test/flutter_test.dart';
import 'package:keepass_one/utils/url_utils.dart';

void main() {
  group('UrlUtils.join', () {
    group('基础拼接测试', () {
      test('baseUrl 无末尾斜杠，path 无开头斜杠', () {
        expect(
          UrlUtils.join('https://example.com', 'path/to'),
          equals('https://example.com/path/to'),
        );
      });

      test('baseUrl 有末尾斜杠，path 无开头斜杠', () {
        expect(
          UrlUtils.join('https://example.com/', 'path/to'),
          equals('https://example.com/path/to'),
        );
      });

      test('baseUrl 无末尾斜杠，path 有开头斜杠', () {
        expect(
          UrlUtils.join('https://example.com', '/path/to'),
          equals('https://example.com/path/to'),
        );
      });

      test('baseUrl 有末尾斜杠，path 有开头斜杠', () {
        expect(
          UrlUtils.join('https://example.com/', '/path/to'),
          equals('https://example.com/path/to'),
        );
      });
    });

    group('实际使用场景 - WebDAV', () {
      test('WebDAV 基础 URL 和文件路径拼接', () {
        expect(
          UrlUtils.join(
            'https://webdav.example.com:8080/webdav/backup',
            '/app/settings.json',
          ),
          equals(
            'https://webdav.example.com:8080/webdav/backup/app/settings.json',
          ),
        );
      });

      test('WebDAV URL 末尾有斜杠', () {
        expect(
          UrlUtils.join(
            'https://webdav.example.com:8080/webdav/backup/',
            '/app/settings.json',
          ),
          equals(
            'https://webdav.example.com:8080/webdav/backup/app/settings.json',
          ),
        );
      });

      test('WebDAV path 无开头斜杠', () {
        expect(
          UrlUtils.join(
            'https://webdav.example.com:8080/webdav/backup',
            'app/settings.json',
          ),
          equals(
            'https://webdav.example.com:8080/webdav/backup/app/settings.json',
          ),
        );
      });

      test('WebDAV 两者都有斜杠', () {
        expect(
          UrlUtils.join(
            'https://webdav.example.com:8080/webdav/backup/',
            'app/settings.json',
          ),
          equals(
            'https://webdav.example.com:8080/webdav/backup/app/settings.json',
          ),
        );
      });
    });

    group('多路径片段拼接', () {
      test('拼接多个路径片段', () {
        expect(
          UrlUtils.join('https://example.com', 'base', ['sub', 'file.json']),
          equals('https://example.com/base/sub/file.json'),
        );
      });

      test('多个路径片段，各包含斜杠', () {
        expect(
          UrlUtils.join('https://example.com/', '/base/', [
            '/sub/',
            '/file.json',
          ]),
          equals('https://example.com/base/sub/file.json'),
        );
      });

      test('空路径片段列表', () {
        expect(
          UrlUtils.join('https://example.com', 'path', []),
          equals('https://example.com/path'),
        );
      });
    });

    group('边界情况测试', () {
      test('baseUrl 只有协议和域名', () {
        expect(
          UrlUtils.join('https://example.com', 'path'),
          equals('https://example.com/path'),
        );
      });

      test('path 为空字符串', () {
        expect(
          UrlUtils.join('https://example.com', ''),
          equals('https://example.com'),
        );
      });

      test('path 为 null', () {
        expect(
          UrlUtils.join('https://example.com', null),
          equals('https://example.com'),
        );
      });

      test('path 只有斜杠', () {
        expect(
          UrlUtils.join('https://example.com', '/'),
          equals('https://example.com'),
        );
      });

      test('baseUrl 末尾多个斜杠', () {
        expect(
          UrlUtils.join('https://example.com///', 'path'),
          equals('https://example.com///path'),
        );
      });

      test('path 开头多个斜杠', () {
        expect(
          UrlUtils.join('https://example.com', '///path'),
          equals('https://example.com/path'),
        );
      });

      test('path 包含空格', () {
        expect(
          UrlUtils.join('https://example.com', '  /path/to  '),
          equals('https://example.com/path/to'),
        );
      });

      test('baseUrl 包含空格', () {
        expect(
          UrlUtils.join('  https://example.com/  ', 'path'),
          equals('https://example.com/path'),
        );
      });
    });

    group('复杂路径测试', () {
      test('深层嵌套路径', () {
        expect(
          UrlUtils.join('https://example.com/api/v1', '/users/123/files'),
          equals('https://example.com/api/v1/users/123/files'),
        );
      });

      test('包含特殊字符的文件名', () {
        expect(
          UrlUtils.join('https://example.com', 'files', ['my file (1).json']),
          equals('https://example.com/files/my file (1).json'),
        );
      });

      test('URL 包含端口号', () {
        expect(
          UrlUtils.join('https://example.com:8080/api', '/v1/data'),
          equals('https://example.com:8080/api/v1/data'),
        );
      });

      test('URL 包含路径参数', () {
        expect(
          UrlUtils.join('https://example.com/api?version=1', '/data'),
          equals('https://example.com/api?version=1/data'),
        );
      });
    });

    group('仅 baseUrl 测试', () {
      test('只有 baseUrl，无 path', () {
        expect(
          UrlUtils.join('https://example.com'),
          equals('https://example.com'),
        );
      });

      test('baseUrl 末尾有斜杠', () {
        expect(
          UrlUtils.join('https://example.com/'),
          equals('https://example.com'),
        );
      });
    });

    group('路径片段包含末尾斜杠', () {
      test('path 末尾有斜杠（应该保留）', () {
        expect(
          UrlUtils.join('https://example.com', 'path/to/'),
          equals('https://example.com/path/to/'),
        );
      });

      test('多个路径片段，最后一个有末尾斜杠', () {
        expect(
          UrlUtils.join('https://example.com', 'base', ['sub/']),
          equals('https://example.com/base/sub/'),
        );
      });
    });
  });
}
