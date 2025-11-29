import 'dart:math';

import 'package:keepass_one/services/webdav/file.dart';
import 'package:xml/xml.dart';

const kPropfindXml = '''
<propfind xmlns="DAV:">
  <prop>
    <d:displayname/>
    <d:resourcetype/>
    <d:getcontentlength/>
    <d:getcontenttype/>
    <d:getetag/>
    <d:getlastmodified/>
  </prop>
</propfind>
''';

List<WebdavEntiry> parsePropfindXml(
  String path,
  String xml, {
  bool skipSelf = true,
}) {
  final entities = <WebdavEntiry>[];
  final document = XmlDocument.parse(xml);
  final response = document.findAll('response');
  for (final element in response) {
    final href = element.find('href').firstOrNull?.innerText ?? '';
    for (final porpstat in element.find('propstat')) {
      if (porpstat.find('status').firstOrNull?.innerText.contains('200') !=
          true) {
        continue;
      }
      for (final prop in porpstat.find('prop')) {
        final isDir =
            prop
                .find('resourcetype')
                .firstOrNull
                ?.find('collection')
                .isNotEmpty ??
            false;

        if (skipSelf) {
          skipSelf = false;
          if (isDir) {
            break;
          }
          return entities;
        }

        final createTimeStr = prop.find('creationdate').firstOrNull?.innerText;
        final createTime = createTimeStr != null
            ? DateTime.tryParse(createTimeStr)
            : null;

        final lastModifiedStr = prop
            .find('getlastmodified')
            .firstOrNull
            ?.innerText;
        final lastModified = lastModifiedStr != null
            ? DateTime.tryParse(lastModifiedStr)
            : null;

        final hrefUri = Uri.decodeFull(href);
        final name = _path2Name(hrefUri);
        final filePath = path + name + (isDir ? '/' : '');

        final etag = prop.find('getetag').firstOrNull?.innerText;

        if (isDir) {
          entities.add(
            WebdavEntiry.directory(
              name: name,
              path: filePath,
              lastModified: lastModified,
              createdTime: createTime,
              etag: etag,
            ),
          );
        } else {
          final sizeStr = prop.find('getcontentlength').firstOrNull?.innerText;
          final size = sizeStr != null ? int.tryParse(sizeStr) : 0;

          final contentType = prop
              .find('getcontenttype')
              .firstOrNull
              ?.innerText;

          entities.add(
            WebdavEntiry.file(
              name: name,
              path: filePath,
              size: size,
              contentType: contentType,
              etag: etag,
              createdAt: createTime,
              lastModified: lastModified,
            ),
          );
        }
      }
    }
  }
  return entities;
}

extension _XmlHelper on XmlNode {
  Iterable<XmlElement> find(String name, {String? namespace}) {
    return findElements(name, namespace: namespace ?? '*');
  }

  Iterable<XmlElement> findAll(String name, {String? namespace}) {
    return findAllElements(name, namespace: namespace ?? '*');
  }
}

String _path2Name(String path) {
  var str = _rtrim(path, '/');
  var index = str.lastIndexOf('/');
  if (index > -1) {
    str = str.substring(index + 1);
  }
  if (str == '') {
    return '/';
  }
  return str;
}

final _rtrimPattern = RegExp(r'\s+$');

String _rtrim(String str, [String? chars]) {
  var pattern = chars != null ? RegExp('[$chars]+\$') : _rtrimPattern;
  return str.replaceAll(pattern, '');
}
