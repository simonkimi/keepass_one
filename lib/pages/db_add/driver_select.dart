import 'package:flutter/cupertino.dart';
import 'package:keepass_one/pages/db_add/webdav_settings.dart';

class DriverSelect extends StatelessWidget {
  const DriverSelect({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text('数据源'),
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context, rootNavigator: true).pop();
          },
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.chevron_left),
        ),
        padding: EdgeInsetsDirectional.zero,
        transitionBetweenRoutes: false,
      ),
      child: Container(
        color: CupertinoColors.systemGroupedBackground,
        child: SafeArea(
          child: Column(
            mainAxisSize: MainAxisSize.max,
            children: [
              CupertinoListSection.insetGrouped(
                children: [
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.folder),
                    title: Text('本地文件'),
                    onTap: () {
                      Navigator.of(context).push(
                        CupertinoPageRoute(
                          builder: (context) => DriverSelect(),
                        ),
                      );
                    },
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.link_circle),
                    title: Text('WebDAV'),
                    onTap: () {
                      Navigator.of(context).push(
                        CupertinoPageRoute(
                          builder: (context) => WebdavSettingsPage(),
                        ),
                      );
                    },
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
