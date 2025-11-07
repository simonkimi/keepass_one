// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'database.dart';

// ignore_for_file: type=lint
class $KdbxItemsTable extends KdbxItems
    with TableInfo<$KdbxItemsTable, KdbxItem> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  $KdbxItemsTable(this.attachedDatabase, [this._alias]);
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
  static const String $name = 'kdbx_items';
  @override
  VerificationContext validateIntegrity(
    Insertable<KdbxItem> instance, {
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
  KdbxItem map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return KdbxItem(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}id'],
      )!,
      name: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}name'],
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
  $KdbxItemsTable createAlias(String alias) {
    return $KdbxItemsTable(attachedDatabase, alias);
  }
}

class KdbxItem extends DataClass implements Insertable<KdbxItem> {
  final int id;
  final String name;
  final String config;
  final DateTime createdAt;
  final DateTime lastAccessedAt;
  final DateTime lastModifiedAt;
  final DateTime lastSyncedAt;
  final String lastHashValue;
  const KdbxItem({
    required this.id,
    required this.name,
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
    map['config'] = Variable<String>(config);
    map['created_at'] = Variable<DateTime>(createdAt);
    map['last_accessed_at'] = Variable<DateTime>(lastAccessedAt);
    map['last_modified_at'] = Variable<DateTime>(lastModifiedAt);
    map['last_synced_at'] = Variable<DateTime>(lastSyncedAt);
    map['last_hash_value'] = Variable<String>(lastHashValue);
    return map;
  }

  KdbxItemsCompanion toCompanion(bool nullToAbsent) {
    return KdbxItemsCompanion(
      id: Value(id),
      name: Value(name),
      config: Value(config),
      createdAt: Value(createdAt),
      lastAccessedAt: Value(lastAccessedAt),
      lastModifiedAt: Value(lastModifiedAt),
      lastSyncedAt: Value(lastSyncedAt),
      lastHashValue: Value(lastHashValue),
    );
  }

  factory KdbxItem.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return KdbxItem(
      id: serializer.fromJson<int>(json['id']),
      name: serializer.fromJson<String>(json['name']),
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
      'config': serializer.toJson<String>(config),
      'createdAt': serializer.toJson<DateTime>(createdAt),
      'lastAccessedAt': serializer.toJson<DateTime>(lastAccessedAt),
      'lastModifiedAt': serializer.toJson<DateTime>(lastModifiedAt),
      'lastSyncedAt': serializer.toJson<DateTime>(lastSyncedAt),
      'lastHashValue': serializer.toJson<String>(lastHashValue),
    };
  }

  KdbxItem copyWith({
    int? id,
    String? name,
    String? config,
    DateTime? createdAt,
    DateTime? lastAccessedAt,
    DateTime? lastModifiedAt,
    DateTime? lastSyncedAt,
    String? lastHashValue,
  }) => KdbxItem(
    id: id ?? this.id,
    name: name ?? this.name,
    config: config ?? this.config,
    createdAt: createdAt ?? this.createdAt,
    lastAccessedAt: lastAccessedAt ?? this.lastAccessedAt,
    lastModifiedAt: lastModifiedAt ?? this.lastModifiedAt,
    lastSyncedAt: lastSyncedAt ?? this.lastSyncedAt,
    lastHashValue: lastHashValue ?? this.lastHashValue,
  );
  KdbxItem copyWithCompanion(KdbxItemsCompanion data) {
    return KdbxItem(
      id: data.id.present ? data.id.value : this.id,
      name: data.name.present ? data.name.value : this.name,
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
    return (StringBuffer('KdbxItem(')
          ..write('id: $id, ')
          ..write('name: $name, ')
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
      (other is KdbxItem &&
          other.id == this.id &&
          other.name == this.name &&
          other.config == this.config &&
          other.createdAt == this.createdAt &&
          other.lastAccessedAt == this.lastAccessedAt &&
          other.lastModifiedAt == this.lastModifiedAt &&
          other.lastSyncedAt == this.lastSyncedAt &&
          other.lastHashValue == this.lastHashValue);
}

class KdbxItemsCompanion extends UpdateCompanion<KdbxItem> {
  final Value<int> id;
  final Value<String> name;
  final Value<String> config;
  final Value<DateTime> createdAt;
  final Value<DateTime> lastAccessedAt;
  final Value<DateTime> lastModifiedAt;
  final Value<DateTime> lastSyncedAt;
  final Value<String> lastHashValue;
  const KdbxItemsCompanion({
    this.id = const Value.absent(),
    this.name = const Value.absent(),
    this.config = const Value.absent(),
    this.createdAt = const Value.absent(),
    this.lastAccessedAt = const Value.absent(),
    this.lastModifiedAt = const Value.absent(),
    this.lastSyncedAt = const Value.absent(),
    this.lastHashValue = const Value.absent(),
  });
  KdbxItemsCompanion.insert({
    this.id = const Value.absent(),
    required String name,
    required String config,
    required DateTime createdAt,
    required DateTime lastAccessedAt,
    required DateTime lastModifiedAt,
    required DateTime lastSyncedAt,
    required String lastHashValue,
  }) : name = Value(name),
       config = Value(config),
       createdAt = Value(createdAt),
       lastAccessedAt = Value(lastAccessedAt),
       lastModifiedAt = Value(lastModifiedAt),
       lastSyncedAt = Value(lastSyncedAt),
       lastHashValue = Value(lastHashValue);
  static Insertable<KdbxItem> custom({
    Expression<int>? id,
    Expression<String>? name,
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
      if (config != null) 'config': config,
      if (createdAt != null) 'created_at': createdAt,
      if (lastAccessedAt != null) 'last_accessed_at': lastAccessedAt,
      if (lastModifiedAt != null) 'last_modified_at': lastModifiedAt,
      if (lastSyncedAt != null) 'last_synced_at': lastSyncedAt,
      if (lastHashValue != null) 'last_hash_value': lastHashValue,
    });
  }

  KdbxItemsCompanion copyWith({
    Value<int>? id,
    Value<String>? name,
    Value<String>? config,
    Value<DateTime>? createdAt,
    Value<DateTime>? lastAccessedAt,
    Value<DateTime>? lastModifiedAt,
    Value<DateTime>? lastSyncedAt,
    Value<String>? lastHashValue,
  }) {
    return KdbxItemsCompanion(
      id: id ?? this.id,
      name: name ?? this.name,
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
    return (StringBuffer('KdbxItemsCompanion(')
          ..write('id: $id, ')
          ..write('name: $name, ')
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

abstract class _$AppDatabase extends GeneratedDatabase {
  _$AppDatabase(QueryExecutor e) : super(e);
  $AppDatabaseManager get managers => $AppDatabaseManager(this);
  late final $KdbxItemsTable kdbxItems = $KdbxItemsTable(this);
  @override
  Iterable<TableInfo<Table, Object?>> get allTables =>
      allSchemaEntities.whereType<TableInfo<Table, Object?>>();
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [kdbxItems];
}

typedef $$KdbxItemsTableCreateCompanionBuilder =
    KdbxItemsCompanion Function({
      Value<int> id,
      required String name,
      required String config,
      required DateTime createdAt,
      required DateTime lastAccessedAt,
      required DateTime lastModifiedAt,
      required DateTime lastSyncedAt,
      required String lastHashValue,
    });
typedef $$KdbxItemsTableUpdateCompanionBuilder =
    KdbxItemsCompanion Function({
      Value<int> id,
      Value<String> name,
      Value<String> config,
      Value<DateTime> createdAt,
      Value<DateTime> lastAccessedAt,
      Value<DateTime> lastModifiedAt,
      Value<DateTime> lastSyncedAt,
      Value<String> lastHashValue,
    });

class $$KdbxItemsTableFilterComposer
    extends Composer<_$AppDatabase, $KdbxItemsTable> {
  $$KdbxItemsTableFilterComposer({
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

class $$KdbxItemsTableOrderingComposer
    extends Composer<_$AppDatabase, $KdbxItemsTable> {
  $$KdbxItemsTableOrderingComposer({
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

class $$KdbxItemsTableAnnotationComposer
    extends Composer<_$AppDatabase, $KdbxItemsTable> {
  $$KdbxItemsTableAnnotationComposer({
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

class $$KdbxItemsTableTableManager
    extends
        RootTableManager<
          _$AppDatabase,
          $KdbxItemsTable,
          KdbxItem,
          $$KdbxItemsTableFilterComposer,
          $$KdbxItemsTableOrderingComposer,
          $$KdbxItemsTableAnnotationComposer,
          $$KdbxItemsTableCreateCompanionBuilder,
          $$KdbxItemsTableUpdateCompanionBuilder,
          (KdbxItem, BaseReferences<_$AppDatabase, $KdbxItemsTable, KdbxItem>),
          KdbxItem,
          PrefetchHooks Function()
        > {
  $$KdbxItemsTableTableManager(_$AppDatabase db, $KdbxItemsTable table)
    : super(
        TableManagerState(
          db: db,
          table: table,
          createFilteringComposer: () =>
              $$KdbxItemsTableFilterComposer($db: db, $table: table),
          createOrderingComposer: () =>
              $$KdbxItemsTableOrderingComposer($db: db, $table: table),
          createComputedFieldComposer: () =>
              $$KdbxItemsTableAnnotationComposer($db: db, $table: table),
          updateCompanionCallback:
              ({
                Value<int> id = const Value.absent(),
                Value<String> name = const Value.absent(),
                Value<String> config = const Value.absent(),
                Value<DateTime> createdAt = const Value.absent(),
                Value<DateTime> lastAccessedAt = const Value.absent(),
                Value<DateTime> lastModifiedAt = const Value.absent(),
                Value<DateTime> lastSyncedAt = const Value.absent(),
                Value<String> lastHashValue = const Value.absent(),
              }) => KdbxItemsCompanion(
                id: id,
                name: name,
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
                required String config,
                required DateTime createdAt,
                required DateTime lastAccessedAt,
                required DateTime lastModifiedAt,
                required DateTime lastSyncedAt,
                required String lastHashValue,
              }) => KdbxItemsCompanion.insert(
                id: id,
                name: name,
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

typedef $$KdbxItemsTableProcessedTableManager =
    ProcessedTableManager<
      _$AppDatabase,
      $KdbxItemsTable,
      KdbxItem,
      $$KdbxItemsTableFilterComposer,
      $$KdbxItemsTableOrderingComposer,
      $$KdbxItemsTableAnnotationComposer,
      $$KdbxItemsTableCreateCompanionBuilder,
      $$KdbxItemsTableUpdateCompanionBuilder,
      (KdbxItem, BaseReferences<_$AppDatabase, $KdbxItemsTable, KdbxItem>),
      KdbxItem,
      PrefetchHooks Function()
    >;

class $AppDatabaseManager {
  final _$AppDatabase _db;
  $AppDatabaseManager(this._db);
  $$KdbxItemsTableTableManager get kdbxItems =>
      $$KdbxItemsTableTableManager(_db, _db.kdbxItems);
}
