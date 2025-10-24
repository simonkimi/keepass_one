import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/db_add/driver_select.dart';
import 'package:keepass_one/widgets/sheet.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context).push(
              CupertinoTabletSheetRoute(builder: (context) => DriverSelect()),
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
