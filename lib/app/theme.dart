import 'package:flutter/foundation.dart';
import 'package:material_ui/material_ui.dart';

abstract final class AppTheme {
  static const _seed = Color(0xFF3D5A80);

  static ThemeData get light => _build(Brightness.light);

  static ThemeData get dark => _build(Brightness.dark);

  static ThemeData _build(Brightness brightness) {
    final colorScheme = ColorScheme.fromSeed(
      seedColor: _seed,
      brightness: brightness,
      dynamicSchemeVariant: DynamicSchemeVariant.neutral,
    );

    return ThemeData(
      colorScheme: colorScheme,
      fontFamily: fontFamily,
      fontFamilyFallback: fontFamilyFallback,
      appBarTheme: AppBarTheme(
        centerTitle: true,
        elevation: 0,
        scrolledUnderElevation: 0,
        backgroundColor: colorScheme.surface,
        foregroundColor: colorScheme.onSurface,
        surfaceTintColor: Colors.transparent,
      ),
      cardTheme: CardThemeData(
        elevation: 0,
        color: colorScheme.surfaceContainerLow,
        margin: EdgeInsets.zero,
      ),
    );
  }

  static TextStyle get systemTextStyle => TextStyle(
    fontFamily: fontFamily,
    fontFamilyFallback: fontFamilyFallback,
  );

  static String? get fontFamily => switch (defaultTargetPlatform) {
    TargetPlatform.windows => 'Segoe UI',
    TargetPlatform.linux => 'Noto Sans',
    _ => null,
  };

  static List<String> get fontFamilyFallback =>
      switch (defaultTargetPlatform) {
    TargetPlatform.windows => const [
      'Microsoft YaHei UI',
      'Microsoft YaHei',
      'SimHei',
    ],
    TargetPlatform.macOS || TargetPlatform.iOS => const [
      'PingFang SC',
      'Hiragino Sans GB',
      'Heiti SC',
    ],
    TargetPlatform.linux => const [
      'Noto Sans CJK SC',
      'Noto Sans SC',
      'WenQuanYi Micro Hei',
    ],
    _ => const [],
  };
}
