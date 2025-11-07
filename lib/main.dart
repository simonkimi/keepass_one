import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:get_it/get_it.dart';
import 'package:keepass_one/pages/home.dart';
import 'package:keepass_one/services/database/database.dart';
import 'package:keepass_one/src/rust/frb_generated.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

final getIt = GetIt.instance;

Future<void> initializeApp() async {
  await RustLib.init();
  await dotenv.load(fileName: ".env");

  getIt.registerSingleton<AppDatabase>(AppDatabase());

  await getIt.allReady();
}

Future<void> main() async {
  await initializeApp();
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoApp(
      debugShowCheckedModeBanner: false,
      theme: CupertinoThemeData(brightness: Brightness.light),
      home: HomePage(),
    );
  }
}
