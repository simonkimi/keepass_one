import 'package:flutter/cupertino.dart';
import 'package:keepass_one/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoApp(
      home: CupertinoPageScaffold(
        navigationBar: CupertinoNavigationBar(middle: Text('Keepass One')),
        child: Center(child: Text('Hello, World!')),
      ),
    );
  }
}
