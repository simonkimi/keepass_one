import 'package:flutter/foundation.dart';
import 'package:keepass_one/app/theme.dart';
import 'package:keepass_one/di.dart';
import 'package:keepass_one/pages/kdbx_selector/kdbx_selector.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/src/rust/frb_generated.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:material_ui/material_ui.dart';

Future<void> initializeApp() async {
  await RustLib.init();
  if (kDebugMode) {
    await dotenv.load(fileName: ".env");
  }

  getIt.registerSingleton<AppDatabase>(AppDatabase());

  await getIt.allReady();
}

Future<void> main() async {
  await initializeApp();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: ThemeMode.system,
      builder: (context, child) => MaterialUiCompatibilityBridge(child: child!),
      home: const KdbxSelectorPage(),
    );
  }
}
