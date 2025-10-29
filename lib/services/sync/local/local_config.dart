import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:keepass_one/services/sync/driver_config.dart';

part 'local_config.freezed.dart';
part 'local_config.g.dart';

@freezed
sealed class LocalConfig with _$LocalConfig implements BaseDriverConfig {
  const LocalConfig._();

  const factory LocalConfig({required String path}) = _LocalConfig;

  factory LocalConfig.fromJson(Map<String, dynamic> json) =>
      _$LocalConfigFromJson(json);
}
