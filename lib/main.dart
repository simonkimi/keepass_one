import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/home.dart';
import 'package:keepass_one/src/rust/frb_generated.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

Future<void> main() async {
  await RustLib.init();
  await dotenv.load(fileName: ".env");
  runApp(const MyApp());
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
