import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:keepass_one/services/database/database.dart';

final databaseProvider = Provider((ref) => AppDatabase());
