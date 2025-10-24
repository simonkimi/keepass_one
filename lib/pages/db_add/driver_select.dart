import 'package:flutter/cupertino.dart';
import 'package:keepass_one/widgets/sheet.dart';

class DriverSelect extends StatelessWidget {
  const DriverSelect({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(
        middle: Text('数据源'),
        leading: CupertinoButton(
          onPressed: () {
            // 优先使用内部 Navigator 返回，如果不存在则关闭整个 Sheet
            final nav = CupertinoTabletSheetRoute.navigatorOf(context);
            if (nav != null && nav.canPop()) {
              nav.pop();
            } else {
              Navigator.of(context, rootNavigator: true).pop();
            }
          },
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.chevron_left),
        ),
        padding: EdgeInsetsDirectional.zero,
        backgroundColor: const Color.fromARGB(255, 255, 255, 255),
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
                      // 使用内部 Navigator 进行导航，而不是创建新的 Modal
                      final nav = CupertinoTabletSheetRoute.navigatorOf(
                        context,
                      );
                      if (nav != null) {
                        nav.push(
                          CupertinoPageRoute(
                            builder: (context) => DriverSelect(),
                          ),
                        );
                      }
                    },
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.link_circle),
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
