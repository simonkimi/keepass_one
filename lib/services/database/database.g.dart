// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'database.dart';

// ignore_for_file: type=lint
class $KdbxFileTable extends KdbxFile
    with TableInfo<$KdbxFileTable, KdbxFileData> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  $KdbxFileTable(this.attachedDatabase, [this._alias]);
  static const VerificationMeta _idMeta = const VerificationMeta('id');
  @override
  late final GeneratedColumn<int> id = GeneratedColumn<int>(
    'id',
    aliasedName,
    false,
    hasAutoIncrement: true,
    type: DriftSqlType.int,
    requiredDuringInsert: false,
    defaultConstraints: GeneratedColumn.constraintIsAlways(
      'PRIMARY KEY AUTOINCREMENT',
    ),
  );
  static const VerificationMeta _nameMeta = const VerificationMeta('name');
  @override
  late final GeneratedColumn<String> name = GeneratedColumn<String>(
    'name',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _descriptionMeta = const VerificationMeta(
    'description',
  );
  @override
  late final GeneratedColumn<String> description = GeneratedColumn<String>(
    'description',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _typeMeta = const VerificationMeta('type');
  @override
  late final GeneratedColumn<String> type = GeneratedColumn<String>(
    'type',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _configMeta = const VerificationMeta('config');
  @override
  late final GeneratedColumn<String> config = GeneratedColumn<String>(
    'config',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _createdAtMeta = const VerificationMeta(
    'createdAt',
  );
  @override
  late final GeneratedColumn<DateTime> createdAt = GeneratedColumn<DateTime>(
    'created_at',
    aliasedName,
    false,
    type: DriftSqlType.dateTime,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _lastAccessedAtMeta = const VerificationMeta(
    'lastAccessedAt',
  );
  @override
  late final GeneratedColumn<DateTime> lastAccessedAt =
      GeneratedColumn<DateTime>(
        'last_accessed_at',
        aliasedName,
        false,
        type: DriftSqlType.dateTime,
        requiredDuringInsert: true,
      );
  static const VerificationMeta _lastModifiedAtMeta = const VerificationMeta(
    'lastModifiedAt',
  );
  @override
  late final GeneratedColumn<DateTime> lastModifiedAt =
      GeneratedColumn<DateTime>(
        'last_modified_at',
        aliasedName,
        false,
        type: DriftSqlType.dateTime,
        requiredDuringInsert: true,
      );
  static const VerificationMeta _lastSyncedAtMeta = const VerificationMeta(
    'lastSyncedAt',
  );
  @override
  late final GeneratedColumn<DateTime> lastSyncedAt = GeneratedColumn<DateTime>(
    'last_synced_at',
    aliasedName,
    false,
    type: DriftSqlType.dateTime,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _lastHashValueMeta = const VerificationMeta(
    'lastHashValue',
  );
  @override
  late final GeneratedColumn<String> lastHashValue = GeneratedColumn<String>(
    'last_hash_value',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  @override
  List<GeneratedColumn> get $columns => [
    id,
    name,
    description,
    type,
    config,
    createdAt,
    lastAccessedAt,
    lastModifiedAt,
    lastSyncedAt,
    lastHashValue,
  ];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'kdbx_file';
  @override
  VerificationContext validateIntegrity(
    Insertable<KdbxFileData> instance, {
    bool isInserting = false,
  }) {
    final context = VerificationContext();
    final data = instance.toColumns(true);
    if (data.containsKey('id')) {
      context.handle(_idMeta, id.isAcceptableOrUnknown(data['id']!, _idMeta));
    }
    if (data.containsKey('name')) {
      context.handle(
        _nameMeta,
        name.isAcceptableOrUnknown(data['name']!, _nameMeta),
      );
    } else if (isInserting) {
      context.missing(_nameMeta);
    }
    if (data.containsKey('description')) {
      context.handle(
        _descriptionMeta,
        description.isAcceptableOrUnknown(
          data['description']!,
          _descriptionMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_descriptionMeta);
    }
    if (data.containsKey('type')) {
      context.handle(
        _typeMeta,
        type.isAcceptableOrUnknown(data['type']!, _typeMeta),
      );
    } else if (isInserting) {
      context.missing(_typeMeta);
    }
    if (data.containsKey('config')) {
      context.handle(
        _configMeta,
        config.isAcceptableOrUnknown(data['config']!, _configMeta),
      );
    } else if (isInserting) {
      context.missing(_configMeta);
    }
    if (data.containsKey('created_at')) {
      context.handle(
        _createdAtMeta,
        createdAt.isAcceptableOrUnknown(data['created_at']!, _createdAtMeta),
      );
    } else if (isInserting) {
      context.missing(_createdAtMeta);
    }
    if (data.containsKey('last_accessed_at')) {
      context.handle(
        _lastAccessedAtMeta,
        lastAccessedAt.isAcceptableOrUnknown(
          data['last_accessed_at']!,
          _lastAccessedAtMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_lastAccessedAtMeta);
    }
    if (data.containsKey('last_modified_at')) {
      context.handle(
        _lastModifiedAtMeta,
        lastModifiedAt.isAcceptableOrUnknown(
          data['last_modified_at']!,
          _lastModifiedAtMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_lastModifiedAtMeta);
    }
    if (data.containsKey('last_synced_at')) {
      context.handle(
        _lastSyncedAtMeta,
        lastSyncedAt.isAcceptableOrUnknown(
          data['last_synced_at']!,
          _lastSyncedAtMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_lastSyncedAtMeta);
    }
    if (data.containsKey('last_hash_value')) {
      context.handle(
        _lastHashValueMeta,
        lastHashValue.isAcceptableOrUnknown(
          data['last_hash_value']!,
          _lastHashValueMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_lastHashValueMeta);
    }
    return context;
  }

  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  KdbxFileData map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return KdbxFileData(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}id'],
      )!,
      name: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}name'],
      )!,
      description: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}description'],
      )!,
      type: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}type'],
      )!,
      config: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}config'],
      )!,
      createdAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}created_at'],
      )!,
      lastAccessedAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}last_accessed_at'],
      )!,
      lastModifiedAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}last_modified_at'],
      )!,
      lastSyncedAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}last_synced_at'],
      )!,
      lastHashValue: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}last_hash_value'],
      )!,
    );
  }

  @override
  $KdbxFileTable createAlias(String alias) {
    return $KdbxFileTable(attachedDatabase, alias);
  }
}

class KdbxFileData extends DataClass implements Insertable<KdbxFileData> {
  final int id;
  final String name;
  final String description;
  final String type;
  final String config;
  final DateTime createdAt;
  final DateTime lastAccessedAt;
  final DateTime lastModifiedAt;
  final DateTime lastSyncedAt;
  final String lastHashValue;
  const KdbxFileData({
    required this.id,
    required this.name,
    required this.description,
    required this.type,
    required this.config,
    required this.createdAt,
    required this.lastAccessedAt,
    required this.lastModifiedAt,
    required this.lastSyncedAt,
    required this.lastHashValue,
  });
  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    map['id'] = Variable<int>(id);
    map['name'] = Variable<String>(name);
    map['description'] = Variable<String>(description);
    map['type'] = Variable<String>(type);
    map['config'] = Variable<String>(config);
    map['created_at'] = Variable<DateTime>(createdAt);
    map['last_accessed_at'] = Variable<DateTime>(lastAccessedAt);
    map['last_modified_at'] = Variable<DateTime>(lastModifiedAt);
    map['last_synced_at'] = Variable<DateTime>(lastSyncedAt);
    map['last_hash_value'] = Variable<String>(lastHashValue);
    return map;
  }

  KdbxFileCompanion toCompanion(bool nullToAbsent) {
    return KdbxFileCompanion(
      id: Value(id),
      name: Value(name),
      description: Value(description),
      type: Value(type),
      config: Value(config),
      createdAt: Value(createdAt),
      lastAccessedAt: Value(lastAccessedAt),
      lastModifiedAt: Value(lastModifiedAt),
      lastSyncedAt: Value(lastSyncedAt),
      lastHashValue: Value(lastHashValue),
    );
  }

  factory KdbxFileData.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return KdbxFileData(
      id: serializer.fromJson<int>(json['id']),
      name: serializer.fromJson<String>(json['name']),
      description: serializer.fromJson<String>(json['description']),
      type: serializer.fromJson<String>(json['type']),
      config: serializer.fromJson<String>(json['config']),
      createdAt: serializer.fromJson<DateTime>(json['createdAt']),
      lastAccessedAt: serializer.fromJson<DateTime>(json['lastAccessedAt']),
      lastModifiedAt: serializer.fromJson<DateTime>(json['lastModifiedAt']),
      lastSyncedAt: serializer.fromJson<DateTime>(json['lastSyncedAt']),
      lastHashValue: serializer.fromJson<String>(json['lastHashValue']),
    );
  }
  @override
  Map<String, dynamic> toJson({ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return <String, dynamic>{
      'id': serializer.toJson<int>(id),
      'name': serializer.toJson<String>(name),
      'description': serializer.toJson<String>(description),
      'type': serializer.toJson<String>(type),
      'config': serializer.toJson<String>(config),
      'createdAt': serializer.toJson<DateTime>(createdAt),
      'lastAccessedAt': serializer.toJson<DateTime>(lastAccessedAt),
      'lastModifiedAt': serializer.toJson<DateTime>(lastModifiedAt),
      'lastSyncedAt': serializer.toJson<DateTime>(lastSyncedAt),
      'lastHashValue': serializer.toJson<String>(lastHashValue),
    };
  }

  KdbxFileData copyWith({
    int? id,
    String? name,
    String? description,
    String? type,
    String? config,
    DateTime? createdAt,
    DateTime? lastAccessedAt,
    DateTime? lastModifiedAt,
    DateTime? lastSyncedAt,
    String? lastHashValue,
  }) => KdbxFileData(
    id: id ?? this.id,
    name: name ?? this.name,
    description: description ?? this.description,
    type: type ?? this.type,
    config: config ?? this.config,
    createdAt: createdAt ?? this.createdAt,
    lastAccessedAt: lastAccessedAt ?? this.lastAccessedAt,
    lastModifiedAt: lastModifiedAt ?? this.lastModifiedAt,
    lastSyncedAt: lastSyncedAt ?? this.lastSyncedAt,
    lastHashValue: lastHashValue ?? this.lastHashValue,
  );
  KdbxFileData copyWithCompanion(KdbxFileCompanion data) {
    return KdbxFileData(
      id: data.id.present ? data.id.value : this.id,
      name: data.name.present ? data.name.value : this.name,
      description: data.description.present
          ? data.description.value
          : this.description,
      type: data.type.present ? data.type.value : this.type,
      config: data.config.present ? data.config.value : this.config,
      createdAt: data.createdAt.present ? data.createdAt.value : this.createdAt,
      lastAccessedAt: data.lastAccessedAt.present
          ? data.lastAccessedAt.value
          : this.lastAccessedAt,
      lastModifiedAt: data.lastModifiedAt.present
          ? data.lastModifiedAt.value
          : this.lastModifiedAt,
      lastSyncedAt: data.lastSyncedAt.present
          ? data.lastSyncedAt.value
          : this.lastSyncedAt,
      lastHashValue: data.lastHashValue.present
          ? data.lastHashValue.value
          : this.lastHashValue,
    );
  }

  @override
  String toString() {
    return (StringBuffer('KdbxFileData(')
          ..write('id: $id, ')
          ..write('name: $name, ')
          ..write('description: $description, ')
          ..write('type: $type, ')
          ..write('config: $config, ')
          ..write('createdAt: $createdAt, ')
          ..write('lastAccessedAt: $lastAccessedAt, ')
          ..write('lastModifiedAt: $lastModifiedAt, ')
          ..write('lastSyncedAt: $lastSyncedAt, ')
          ..write('lastHashValue: $lastHashValue')
          ..write(')'))
        .toString();
  }

  @override
  int get hashCode => Object.hash(
    id,
    name,
    description,
    type,
    config,
    createdAt,
    lastAccessedAt,
    lastModifiedAt,
    lastSyncedAt,
    lastHashValue,
  );
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      (other is KdbxFileData &&
          other.id == this.id &&
          other.name == this.name &&
          other.description == this.description &&
          other.type == this.type &&
          other.config == this.config &&
          other.createdAt == this.createdAt &&
          other.lastAccessedAt == this.lastAccessedAt &&
          other.lastModifiedAt == this.lastModifiedAt &&
          other.lastSyncedAt == this.lastSyncedAt &&
          other.lastHashValue == this.lastHashValue);
}

class KdbxFileCompanion extends UpdateCompanion<KdbxFileData> {
  final Value<int> id;
  final Value<String> name;
  final Value<String> description;
  final Value<String> type;
  final Value<String> config;
  final Value<DateTime> createdAt;
  final Value<DateTime> lastAccessedAt;
  final Value<DateTime> lastModifiedAt;
  final Value<DateTime> lastSyncedAt;
  final Value<String> lastHashValue;
  const KdbxFileCompanion({
    this.id = const Value.absent(),
    this.name = const Value.absent(),
    this.description = const Value.absent(),
    this.type = const Value.absent(),
    this.config = const Value.absent(),
    this.createdAt = const Value.absent(),
    this.lastAccessedAt = const Value.absent(),
    this.lastModifiedAt = const Value.absent(),
    this.lastSyncedAt = const Value.absent(),
    this.lastHashValue = const Value.absent(),
  });
  KdbxFileCompanion.insert({
    this.id = const Value.absent(),
    required String name,
    required String description,
    required String type,
    required String config,
    required DateTime createdAt,
    required DateTime lastAccessedAt,
    required DateTime lastModifiedAt,
    required DateTime lastSyncedAt,
    required String lastHashValue,
  }) : name = Value(name),
       description = Value(description),
       type = Value(type),
       config = Value(config),
       createdAt = Value(createdAt),
       lastAccessedAt = Value(lastAccessedAt),
       lastModifiedAt = Value(lastModifiedAt),
       lastSyncedAt = Value(lastSyncedAt),
       lastHashValue = Value(lastHashValue);
  static Insertable<KdbxFileData> custom({
    Expression<int>? id,
    Expression<String>? name,
    Expression<String>? description,
    Expression<String>? type,
    Expression<String>? config,
    Expression<DateTime>? createdAt,
    Expression<DateTime>? lastAccessedAt,
    Expression<DateTime>? lastModifiedAt,
    Expression<DateTime>? lastSyncedAt,
    Expression<String>? lastHashValue,
  }) {
    return RawValuesInsertable({
      if (id != null) 'id': id,
      if (name != null) 'name': name,
      if (description != null) 'description': description,
      if (type != null) 'type': type,
      if (config != null) 'config': config,
      if (createdAt != null) 'created_at': createdAt,
      if (lastAccessedAt != null) 'last_accessed_at': lastAccessedAt,
      if (lastModifiedAt != null) 'last_modified_at': lastModifiedAt,
      if (lastSyncedAt != null) 'last_synced_at': lastSyncedAt,
      if (lastHashValue != null) 'last_hash_value': lastHashValue,
    });
  }

  KdbxFileCompanion copyWith({
    Value<int>? id,
    Value<String>? name,
    Value<String>? description,
    Value<String>? type,
    Value<String>? config,
    Value<DateTime>? createdAt,
    Value<DateTime>? lastAccessedAt,
    Value<DateTime>? lastModifiedAt,
    Value<DateTime>? lastSyncedAt,
    Value<String>? lastHashValue,
  }) {
    return KdbxFileCompanion(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      type: type ?? this.type,
      config: config ?? this.config,
      createdAt: createdAt ?? this.createdAt,
      lastAccessedAt: lastAccessedAt ?? this.lastAccessedAt,
      lastModifiedAt: lastModifiedAt ?? this.lastModifiedAt,
      lastSyncedAt: lastSyncedAt ?? this.lastSyncedAt,
      lastHashValue: lastHashValue ?? this.lastHashValue,
    );
  }

  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    if (id.present) {
      map['id'] = Variable<int>(id.value);
    }
    if (name.present) {
      map['name'] = Variable<String>(name.value);
    }
    if (description.present) {
      map['description'] = Variable<String>(description.value);
    }
    if (type.present) {
      map['type'] = Variable<String>(type.value);
    }
    if (config.present) {
      map['config'] = Variable<String>(config.value);
    }
    if (createdAt.present) {
      map['created_at'] = Variable<DateTime>(createdAt.value);
    }
    if (lastAccessedAt.present) {
      map['last_accessed_at'] = Variable<DateTime>(lastAccessedAt.value);
    }
    if (lastModifiedAt.present) {
      map['last_modified_at'] = Variable<DateTime>(lastModifiedAt.value);
    }
    if (lastSyncedAt.present) {
      map['last_synced_at'] = Variable<DateTime>(lastSyncedAt.value);
    }
    if (lastHashValue.present) {
      map['last_hash_value'] = Variable<String>(lastHashValue.value);
    }
    return map;
  }

  @override
  String toString() {
    return (StringBuffer('KdbxFileCompanion(')
          ..write('id: $id, ')
          ..write('name: $name, ')
          ..write('description: $description, ')
          ..write('type: $type, ')
          ..write('config: $config, ')
          ..write('createdAt: $createdAt, ')
          ..write('lastAccessedAt: $lastAccessedAt, ')
          ..write('lastModifiedAt: $lastModifiedAt, ')
          ..write('lastSyncedAt: $lastSyncedAt, ')
          ..write('lastHashValue: $lastHashValue')
          ..write(')'))
        .toString();
  }
}

class $KdbxKeyFileTable extends KdbxKeyFile
    with TableInfo<$KdbxKeyFileTable, KdbxKeyFileData> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  $KdbxKeyFileTable(this.attachedDatabase, [this._alias]);
  static const VerificationMeta _idMeta = const VerificationMeta('id');
  @override
  late final GeneratedColumn<int> id = GeneratedColumn<int>(
    'id',
    aliasedName,
    false,
    hasAutoIncrement: true,
    type: DriftSqlType.int,
    requiredDuringInsert: false,
    defaultConstraints: GeneratedColumn.constraintIsAlways(
      'PRIMARY KEY AUTOINCREMENT',
    ),
  );
  static const VerificationMeta _nameMeta = const VerificationMeta('name');
  @override
  late final GeneratedColumn<String> name = GeneratedColumn<String>(
    'name',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _pathMeta = const VerificationMeta('path');
  @override
  late final GeneratedColumn<String> path = GeneratedColumn<String>(
    'path',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _dataMeta = const VerificationMeta('data');
  @override
  late final GeneratedColumn<Uint8List> data = GeneratedColumn<Uint8List>(
    'data',
    aliasedName,
    false,
    type: DriftSqlType.blob,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _createdAtMeta = const VerificationMeta(
    'createdAt',
  );
  @override
  late final GeneratedColumn<DateTime> createdAt = GeneratedColumn<DateTime>(
    'created_at',
    aliasedName,
    false,
    type: DriftSqlType.dateTime,
    requiredDuringInsert: true,
  );
  static const VerificationMeta _lastModifiedAtMeta = const VerificationMeta(
    'lastModifiedAt',
  );
  @override
  late final GeneratedColumn<DateTime> lastModifiedAt =
      GeneratedColumn<DateTime>(
        'last_modified_at',
        aliasedName,
        false,
        type: DriftSqlType.dateTime,
        requiredDuringInsert: true,
      );
  @override
  List<GeneratedColumn> get $columns => [
    id,
    name,
    path,
    data,
    createdAt,
    lastModifiedAt,
  ];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'kdbx_key_file';
  @override
  VerificationContext validateIntegrity(
    Insertable<KdbxKeyFileData> instance, {
    bool isInserting = false,
  }) {
    final context = VerificationContext();
    final data = instance.toColumns(true);
    if (data.containsKey('id')) {
      context.handle(_idMeta, id.isAcceptableOrUnknown(data['id']!, _idMeta));
    }
    if (data.containsKey('name')) {
      context.handle(
        _nameMeta,
        name.isAcceptableOrUnknown(data['name']!, _nameMeta),
      );
    } else if (isInserting) {
      context.missing(_nameMeta);
    }
    if (data.containsKey('path')) {
      context.handle(
        _pathMeta,
        path.isAcceptableOrUnknown(data['path']!, _pathMeta),
      );
    } else if (isInserting) {
      context.missing(_pathMeta);
    }
    if (data.containsKey('data')) {
      context.handle(
        _dataMeta,
        this.data.isAcceptableOrUnknown(data['data']!, _dataMeta),
      );
    } else if (isInserting) {
      context.missing(_dataMeta);
    }
    if (data.containsKey('created_at')) {
      context.handle(
        _createdAtMeta,
        createdAt.isAcceptableOrUnknown(data['created_at']!, _createdAtMeta),
      );
    } else if (isInserting) {
      context.missing(_createdAtMeta);
    }
    if (data.containsKey('last_modified_at')) {
      context.handle(
        _lastModifiedAtMeta,
        lastModifiedAt.isAcceptableOrUnknown(
          data['last_modified_at']!,
          _lastModifiedAtMeta,
        ),
      );
    } else if (isInserting) {
      context.missing(_lastModifiedAtMeta);
    }
    return context;
  }

  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  KdbxKeyFileData map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return KdbxKeyFileData(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}id'],
      )!,
      name: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}name'],
      )!,
      path: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}path'],
      )!,
      data: attachedDatabase.typeMapping.read(
        DriftSqlType.blob,
        data['${effectivePrefix}data'],
      )!,
      createdAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}created_at'],
      )!,
      lastModifiedAt: attachedDatabase.typeMapping.read(
        DriftSqlType.dateTime,
        data['${effectivePrefix}last_modified_at'],
      )!,
    );
  }

  @override
  $KdbxKeyFileTable createAlias(String alias) {
    return $KdbxKeyFileTable(attachedDatabase, alias);
  }
}

class KdbxKeyFileData extends DataClass implements Insertable<KdbxKeyFileData> {
  final int id;
  final String name;
  final String path;
  final Uint8List data;
  final DateTime createdAt;
  final DateTime lastModifiedAt;
  const KdbxKeyFileData({
    required this.id,
    required this.name,
    required this.path,
    required this.data,
    required this.createdAt,
    required this.lastModifiedAt,
  });
  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    map['id'] = Variable<int>(id);
    map['name'] = Variable<String>(name);
    map['path'] = Variable<String>(path);
    map['data'] = Variable<Uint8List>(data);
    map['created_at'] = Variable<DateTime>(createdAt);
    map['last_modified_at'] = Variable<DateTime>(lastModifiedAt);
    return map;
  }

  KdbxKeyFileCompanion toCompanion(bool nullToAbsent) {
    return KdbxKeyFileCompanion(
      id: Value(id),
      name: Value(name),
      path: Value(path),
      data: Value(data),
      createdAt: Value(createdAt),
      lastModifiedAt: Value(lastModifiedAt),
    );
  }

  factory KdbxKeyFileData.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return KdbxKeyFileData(
      id: serializer.fromJson<int>(json['id']),
      name: serializer.fromJson<String>(json['name']),
      path: serializer.fromJson<String>(json['path']),
      data: serializer.fromJson<Uint8List>(json['data']),
      createdAt: serializer.fromJson<DateTime>(json['createdAt']),
      lastModifiedAt: serializer.fromJson<DateTime>(json['lastModifiedAt']),
    );
  }
  @override
  Map<String, dynamic> toJson({ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return <String, dynamic>{
      'id': serializer.toJson<int>(id),
      'name': serializer.toJson<String>(name),
      'path': serializer.toJson<String>(path),
      'data': serializer.toJson<Uint8List>(data),
      'createdAt': serializer.toJson<DateTime>(createdAt),
      'lastModifiedAt': serializer.toJson<DateTime>(lastModifiedAt),
    };
  }

  KdbxKeyFileData copyWith({
    int? id,
    String? name,
    String? path,
    Uint8List? data,
    DateTime? createdAt,
    DateTime? lastModifiedAt,
  }) => KdbxKeyFileData(
    id: id ?? this.id,
    name: name ?? this.name,
    path: path ?? this.path,
    data: data ?? this.data,
    createdAt: createdAt ?? this.createdAt,
    lastModifiedAt: lastModifiedAt ?? this.lastModifiedAt,
  );
  KdbxKeyFileData copyWithCompanion(KdbxKeyFileCompanion data) {
    return KdbxKeyFileData(
      id: data.id.present ? data.id.value : this.id,
      name: data.name.present ? data.name.value : this.name,
      path: data.path.present ? data.path.value : this.path,
      data: data.data.present ? data.data.value : this.data,
      createdAt: data.createdAt.present ? data.createdAt.value : this.createdAt,
      lastModifiedAt: data.lastModifiedAt.present
          ? data.lastModifiedAt.value
          : this.lastModifiedAt,
    );
  }

  @override
  String toString() {
    return (StringBuffer('KdbxKeyFileData(')
          ..write('id: $id, ')
          ..write('name: $name, ')
          ..write('path: $path, ')
          ..write('data: $data, ')
          ..write('createdAt: $createdAt, ')
          ..write('lastModifiedAt: $lastModifiedAt')
          ..write(')'))
        .toString();
  }

  @override
  int get hashCode => Object.hash(
    id,
    name,
    path,
    $driftBlobEquality.hash(data),
    createdAt,
    lastModifiedAt,
  );
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      (other is KdbxKeyFileData &&
          other.id == this.id &&
          other.name == this.name &&
          other.path == this.path &&
          $driftBlobEquality.equals(other.data, this.data) &&
          other.createdAt == this.createdAt &&
          other.lastModifiedAt == this.lastModifiedAt);
}

class KdbxKeyFileCompanion extends UpdateCompanion<KdbxKeyFileData> {
  final Value<int> id;
  final Value<String> name;
  final Value<String> path;
  final Value<Uint8List> data;
  final Value<DateTime> createdAt;
  final Value<DateTime> lastModifiedAt;
  const KdbxKeyFileCompanion({
    this.id = const Value.absent(),
    this.name = const Value.absent(),
    this.path = const Value.absent(),
    this.data = const Value.absent(),
    this.createdAt = const Value.absent(),
    this.lastModifiedAt = const Value.absent(),
  });
  KdbxKeyFileCompanion.insert({
    this.id = const Value.absent(),
    required String name,
    required String path,
    required Uint8List data,
    required DateTime createdAt,
    required DateTime lastModifiedAt,
  }) : name = Value(name),
       path = Value(path),
       data = Value(data),
       createdAt = Value(createdAt),
       lastModifiedAt = Value(lastModifiedAt);
  static Insertable<KdbxKeyFileData> custom({
    Expression<int>? id,
    Expression<String>? name,
    Expression<String>? path,
    Expression<Uint8List>? data,
    Expression<DateTime>? createdAt,
    Expression<DateTime>? lastModifiedAt,
  }) {
    return RawValuesInsertable({
      if (id != null) 'id': id,
      if (name != null) 'name': name,
      if (path != null) 'path': path,
      if (data != null) 'data': data,
      if (createdAt != null) 'created_at': createdAt,
      if (lastModifiedAt != null) 'last_modified_at': lastModifiedAt,
    });
  }

  KdbxKeyFileCompanion copyWith({
    Value<int>? id,
    Value<String>? name,
    Value<String>? path,
    Value<Uint8List>? data,
    Value<DateTime>? createdAt,
    Value<DateTime>? lastModifiedAt,
  }) {
    return KdbxKeyFileCompanion(
      id: id ?? this.id,
      name: name ?? this.name,
      path: path ?? this.path,
      data: data ?? this.data,
      createdAt: createdAt ?? this.createdAt,
      lastModifiedAt: lastModifiedAt ?? this.lastModifiedAt,
    );
  }

  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    if (id.present) {
      map['id'] = Variable<int>(id.value);
    }
    if (name.present) {
      map['name'] = Variable<String>(name.value);
    }
    if (path.present) {
      map['path'] = Variable<String>(path.value);
    }
    if (data.present) {
      map['data'] = Variable<Uint8List>(data.value);
    }
    if (createdAt.present) {
      map['created_at'] = Variable<DateTime>(createdAt.value);
    }
    if (lastModifiedAt.present) {
      map['last_modified_at'] = Variable<DateTime>(lastModifiedAt.value);
    }
    return map;
  }

  @override
  String toString() {
    return (StringBuffer('KdbxKeyFileCompanion(')
          ..write('id: $id, ')
          ..write('name: $name, ')
          ..write('path: $path, ')
          ..write('data: $data, ')
          ..write('createdAt: $createdAt, ')
          ..write('lastModifiedAt: $lastModifiedAt')
          ..write(')'))
        .toString();
  }
}

abstract class _$AppDatabase extends GeneratedDatabase {
  _$AppDatabase(QueryExecutor e) : super(e);
  $AppDatabaseManager get managers => $AppDatabaseManager(this);
  late final $KdbxFileTable kdbxFile = $KdbxFileTable(this);
  late final $KdbxKeyFileTable kdbxKeyFile = $KdbxKeyFileTable(this);
  late final KdbxFileDao kdbxFileDao = KdbxFileDao(this as AppDatabase);
  late final KdbxKeyFileDao kdbxKeyFileDao = KdbxKeyFileDao(
    this as AppDatabase,
  );
  @override
  Iterable<TableInfo<Table, Object?>> get allTables =>
      allSchemaEntities.whereType<TableInfo<Table, Object?>>();
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [kdbxFile, kdbxKeyFile];
}

typedef $$KdbxFileTableCreateCompanionBuilder =
    KdbxFileCompanion Function({
      Value<int> id,
      required String name,
      required String description,
      required String type,
      required String config,
      required DateTime createdAt,
      required DateTime lastAccessedAt,
      required DateTime lastModifiedAt,
      required DateTime lastSyncedAt,
      required String lastHashValue,
    });
typedef $$KdbxFileTableUpdateCompanionBuilder =
    KdbxFileCompanion Function({
      Value<int> id,
      Value<String> name,
      Value<String> description,
      Value<String> type,
      Value<String> config,
      Value<DateTime> createdAt,
      Value<DateTime> lastAccessedAt,
      Value<DateTime> lastModifiedAt,
      Value<DateTime> lastSyncedAt,
      Value<String> lastHashValue,
    });

class $$KdbxFileTableFilterComposer
    extends Composer<_$AppDatabase, $KdbxFileTable> {
  $$KdbxFileTableFilterComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnFilters<int> get id => $composableBuilder(
    column: $table.id,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get name => $composableBuilder(
    column: $table.name,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get description => $composableBuilder(
    column: $table.description,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get type => $composableBuilder(
    column: $table.type,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get config => $composableBuilder(
    column: $table.config,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get createdAt => $composableBuilder(
    column: $table.createdAt,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get lastAccessedAt => $composableBuilder(
    column: $table.lastAccessedAt,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get lastSyncedAt => $composableBuilder(
    column: $table.lastSyncedAt,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get lastHashValue => $composableBuilder(
    column: $table.lastHashValue,
    builder: (column) => ColumnFilters(column),
  );
}

class $$KdbxFileTableOrderingComposer
    extends Composer<_$AppDatabase, $KdbxFileTable> {
  $$KdbxFileTableOrderingComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnOrderings<int> get id => $composableBuilder(
    column: $table.id,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get name => $composableBuilder(
    column: $table.name,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get description => $composableBuilder(
    column: $table.description,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get type => $composableBuilder(
    column: $table.type,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get config => $composableBuilder(
    column: $table.config,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get createdAt => $composableBuilder(
    column: $table.createdAt,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get lastAccessedAt => $composableBuilder(
    column: $table.lastAccessedAt,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get lastSyncedAt => $composableBuilder(
    column: $table.lastSyncedAt,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get lastHashValue => $composableBuilder(
    column: $table.lastHashValue,
    builder: (column) => ColumnOrderings(column),
  );
}

class $$KdbxFileTableAnnotationComposer
    extends Composer<_$AppDatabase, $KdbxFileTable> {
  $$KdbxFileTableAnnotationComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  GeneratedColumn<int> get id =>
      $composableBuilder(column: $table.id, builder: (column) => column);

  GeneratedColumn<String> get name =>
      $composableBuilder(column: $table.name, builder: (column) => column);

  GeneratedColumn<String> get description => $composableBuilder(
    column: $table.description,
    builder: (column) => column,
  );

  GeneratedColumn<String> get type =>
      $composableBuilder(column: $table.type, builder: (column) => column);

  GeneratedColumn<String> get config =>
      $composableBuilder(column: $table.config, builder: (column) => column);

  GeneratedColumn<DateTime> get createdAt =>
      $composableBuilder(column: $table.createdAt, builder: (column) => column);

  GeneratedColumn<DateTime> get lastAccessedAt => $composableBuilder(
    column: $table.lastAccessedAt,
    builder: (column) => column,
  );

  GeneratedColumn<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => column,
  );

  GeneratedColumn<DateTime> get lastSyncedAt => $composableBuilder(
    column: $table.lastSyncedAt,
    builder: (column) => column,
  );

  GeneratedColumn<String> get lastHashValue => $composableBuilder(
    column: $table.lastHashValue,
    builder: (column) => column,
  );
}

class $$KdbxFileTableTableManager
    extends
        RootTableManager<
          _$AppDatabase,
          $KdbxFileTable,
          KdbxFileData,
          $$KdbxFileTableFilterComposer,
          $$KdbxFileTableOrderingComposer,
          $$KdbxFileTableAnnotationComposer,
          $$KdbxFileTableCreateCompanionBuilder,
          $$KdbxFileTableUpdateCompanionBuilder,
          (
            KdbxFileData,
            BaseReferences<_$AppDatabase, $KdbxFileTable, KdbxFileData>,
          ),
          KdbxFileData,
          PrefetchHooks Function()
        > {
  $$KdbxFileTableTableManager(_$AppDatabase db, $KdbxFileTable table)
    : super(
        TableManagerState(
          db: db,
          table: table,
          createFilteringComposer: () =>
              $$KdbxFileTableFilterComposer($db: db, $table: table),
          createOrderingComposer: () =>
              $$KdbxFileTableOrderingComposer($db: db, $table: table),
          createComputedFieldComposer: () =>
              $$KdbxFileTableAnnotationComposer($db: db, $table: table),
          updateCompanionCallback:
              ({
                Value<int> id = const Value.absent(),
                Value<String> name = const Value.absent(),
                Value<String> description = const Value.absent(),
                Value<String> type = const Value.absent(),
                Value<String> config = const Value.absent(),
                Value<DateTime> createdAt = const Value.absent(),
                Value<DateTime> lastAccessedAt = const Value.absent(),
                Value<DateTime> lastModifiedAt = const Value.absent(),
                Value<DateTime> lastSyncedAt = const Value.absent(),
                Value<String> lastHashValue = const Value.absent(),
              }) => KdbxFileCompanion(
                id: id,
                name: name,
                description: description,
                type: type,
                config: config,
                createdAt: createdAt,
                lastAccessedAt: lastAccessedAt,
                lastModifiedAt: lastModifiedAt,
                lastSyncedAt: lastSyncedAt,
                lastHashValue: lastHashValue,
              ),
          createCompanionCallback:
              ({
                Value<int> id = const Value.absent(),
                required String name,
                required String description,
                required String type,
                required String config,
                required DateTime createdAt,
                required DateTime lastAccessedAt,
                required DateTime lastModifiedAt,
                required DateTime lastSyncedAt,
                required String lastHashValue,
              }) => KdbxFileCompanion.insert(
                id: id,
                name: name,
                description: description,
                type: type,
                config: config,
                createdAt: createdAt,
                lastAccessedAt: lastAccessedAt,
                lastModifiedAt: lastModifiedAt,
                lastSyncedAt: lastSyncedAt,
                lastHashValue: lastHashValue,
              ),
          withReferenceMapper: (p0) => p0
              .map((e) => (e.readTable(table), BaseReferences(db, table, e)))
              .toList(),
          prefetchHooksCallback: null,
        ),
      );
}

typedef $$KdbxFileTableProcessedTableManager =
    ProcessedTableManager<
      _$AppDatabase,
      $KdbxFileTable,
      KdbxFileData,
      $$KdbxFileTableFilterComposer,
      $$KdbxFileTableOrderingComposer,
      $$KdbxFileTableAnnotationComposer,
      $$KdbxFileTableCreateCompanionBuilder,
      $$KdbxFileTableUpdateCompanionBuilder,
      (
        KdbxFileData,
        BaseReferences<_$AppDatabase, $KdbxFileTable, KdbxFileData>,
      ),
      KdbxFileData,
      PrefetchHooks Function()
    >;
typedef $$KdbxKeyFileTableCreateCompanionBuilder =
    KdbxKeyFileCompanion Function({
      Value<int> id,
      required String name,
      required String path,
      required Uint8List data,
      required DateTime createdAt,
      required DateTime lastModifiedAt,
    });
typedef $$KdbxKeyFileTableUpdateCompanionBuilder =
    KdbxKeyFileCompanion Function({
      Value<int> id,
      Value<String> name,
      Value<String> path,
      Value<Uint8List> data,
      Value<DateTime> createdAt,
      Value<DateTime> lastModifiedAt,
    });

class $$KdbxKeyFileTableFilterComposer
    extends Composer<_$AppDatabase, $KdbxKeyFileTable> {
  $$KdbxKeyFileTableFilterComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnFilters<int> get id => $composableBuilder(
    column: $table.id,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get name => $composableBuilder(
    column: $table.name,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<String> get path => $composableBuilder(
    column: $table.path,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<Uint8List> get data => $composableBuilder(
    column: $table.data,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get createdAt => $composableBuilder(
    column: $table.createdAt,
    builder: (column) => ColumnFilters(column),
  );

  ColumnFilters<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => ColumnFilters(column),
  );
}

class $$KdbxKeyFileTableOrderingComposer
    extends Composer<_$AppDatabase, $KdbxKeyFileTable> {
  $$KdbxKeyFileTableOrderingComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnOrderings<int> get id => $composableBuilder(
    column: $table.id,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get name => $composableBuilder(
    column: $table.name,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<String> get path => $composableBuilder(
    column: $table.path,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<Uint8List> get data => $composableBuilder(
    column: $table.data,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get createdAt => $composableBuilder(
    column: $table.createdAt,
    builder: (column) => ColumnOrderings(column),
  );

  ColumnOrderings<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => ColumnOrderings(column),
  );
}

class $$KdbxKeyFileTableAnnotationComposer
    extends Composer<_$AppDatabase, $KdbxKeyFileTable> {
  $$KdbxKeyFileTableAnnotationComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  GeneratedColumn<int> get id =>
      $composableBuilder(column: $table.id, builder: (column) => column);

  GeneratedColumn<String> get name =>
      $composableBuilder(column: $table.name, builder: (column) => column);

  GeneratedColumn<String> get path =>
      $composableBuilder(column: $table.path, builder: (column) => column);

  GeneratedColumn<Uint8List> get data =>
      $composableBuilder(column: $table.data, builder: (column) => column);

  GeneratedColumn<DateTime> get createdAt =>
      $composableBuilder(column: $table.createdAt, builder: (column) => column);

  GeneratedColumn<DateTime> get lastModifiedAt => $composableBuilder(
    column: $table.lastModifiedAt,
    builder: (column) => column,
  );
}

class $$KdbxKeyFileTableTableManager
    extends
        RootTableManager<
          _$AppDatabase,
          $KdbxKeyFileTable,
          KdbxKeyFileData,
          $$KdbxKeyFileTableFilterComposer,
          $$KdbxKeyFileTableOrderingComposer,
          $$KdbxKeyFileTableAnnotationComposer,
          $$KdbxKeyFileTableCreateCompanionBuilder,
          $$KdbxKeyFileTableUpdateCompanionBuilder,
          (
            KdbxKeyFileData,
            BaseReferences<_$AppDatabase, $KdbxKeyFileTable, KdbxKeyFileData>,
          ),
          KdbxKeyFileData,
          PrefetchHooks Function()
        > {
  $$KdbxKeyFileTableTableManager(_$AppDatabase db, $KdbxKeyFileTable table)
    : super(
        TableManagerState(
          db: db,
          table: table,
          createFilteringComposer: () =>
              $$KdbxKeyFileTableFilterComposer($db: db, $table: table),
          createOrderingComposer: () =>
              $$KdbxKeyFileTableOrderingComposer($db: db, $table: table),
          createComputedFieldComposer: () =>
              $$KdbxKeyFileTableAnnotationComposer($db: db, $table: table),
          updateCompanionCallback:
              ({
                Value<int> id = const Value.absent(),
                Value<String> name = const Value.absent(),
                Value<String> path = const Value.absent(),
                Value<Uint8List> data = const Value.absent(),
                Value<DateTime> createdAt = const Value.absent(),
                Value<DateTime> lastModifiedAt = const Value.absent(),
              }) => KdbxKeyFileCompanion(
                id: id,
                name: name,
                path: path,
                data: data,
                createdAt: createdAt,
                lastModifiedAt: lastModifiedAt,
              ),
          createCompanionCallback:
              ({
                Value<int> id = const Value.absent(),
                required String name,
                required String path,
                required Uint8List data,
                required DateTime createdAt,
                required DateTime lastModifiedAt,
              }) => KdbxKeyFileCompanion.insert(
                id: id,
                name: name,
                path: path,
                data: data,
                createdAt: createdAt,
                lastModifiedAt: lastModifiedAt,
              ),
          withReferenceMapper: (p0) => p0
              .map((e) => (e.readTable(table), BaseReferences(db, table, e)))
              .toList(),
          prefetchHooksCallback: null,
        ),
      );
}

typedef $$KdbxKeyFileTableProcessedTableManager =
    ProcessedTableManager<
      _$AppDatabase,
      $KdbxKeyFileTable,
      KdbxKeyFileData,
      $$KdbxKeyFileTableFilterComposer,
      $$KdbxKeyFileTableOrderingComposer,
      $$KdbxKeyFileTableAnnotationComposer,
      $$KdbxKeyFileTableCreateCompanionBuilder,
      $$KdbxKeyFileTableUpdateCompanionBuilder,
      (
        KdbxKeyFileData,
        BaseReferences<_$AppDatabase, $KdbxKeyFileTable, KdbxKeyFileData>,
      ),
      KdbxKeyFileData,
      PrefetchHooks Function()
    >;

class $AppDatabaseManager {
  final _$AppDatabase _db;
  $AppDatabaseManager(this._db);
  $$KdbxFileTableTableManager get kdbxFile =>
      $$KdbxFileTableTableManager(_db, _db.kdbxFile);
  $$KdbxKeyFileTableTableManager get kdbxKeyFile =>
      $$KdbxKeyFileTableTableManager(_db, _db.kdbxKeyFile);
}
