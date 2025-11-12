import 'package:collection/collection.dart';

enum DriverType {
  local('local'),
  webdav('webdav');

  final String key;
  const DriverType(this.key);

  static DriverType? fromKey(String key) {
    return values.firstWhereOrNull((e) => e.key == key);
  }
}

abstract interface class BaseDriverConfig {
  Map<String, dynamic> toJson();

  String get name;

  String get description;

  DriverType get type;
}
