import 'package:flutter/cupertino.dart';
import 'package:flutter/widgets.dart';

class DbIndexPage extends StatelessWidget {
  const DbIndexPage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(middle: Text('添加数据源')),
      child: Column(children: [Text('Hello, World!')]),
    );
  }
}
