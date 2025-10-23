import 'package:flutter/cupertino.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';

class DeiverSelect extends StatelessWidget {
  const DeiverSelect({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text('数据源'),
        leading: CupertinoButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.chevron_left),
        ),
        padding: EdgeInsetsDirectional.zero,
        backgroundColor: const Color.fromARGB(255, 255, 255, 255),
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
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.globe),
                    title: Text('WebDAV'),
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
