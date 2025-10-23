import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/db_add/driver_select.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context).push(
              CupertinoSheetRoute(
                builder: (context) =>
                    CupertinoPageScaffold(child: DeiverSelect()),
              ),
            );
          },
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.add),
        ),
        padding: EdgeInsetsDirectional.zero,
        middle: Text('Demo'),
      ),
      child: SizedBox(
        width: double.infinity,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [Text('Hello, World!')],
        ),
      ),
    );
  }
}
