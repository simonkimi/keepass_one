import 'package:flutter/cupertino.dart';

class KdbxFileFilePage extends StatelessWidget {
  const KdbxFileFilePage({super.key});

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      navigationBar: CupertinoNavigationBar(middle: Text('密钥文件')),
      child: Container(
        color: CupertinoColors.systemGroupedBackground,
        child: SafeArea(
          child: Column(
            children: [
              CupertinoListSection.insetGrouped(
                children: [
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.nosign),
                    title: Text('没有密钥文件'),
                    onTap: () {},
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.doc_on_doc),
                    title: Text('导入密钥文件'),
                    onTap: () {},
                  ),
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.doc_text_search),
                    title: Text('选择密钥文件'),
                    onTap: () {},
                  ),
                ],
              ),

              CupertinoListSection.insetGrouped(
                children: [
                  CupertinoListTile(
                    leading: Icon(CupertinoIcons.doc),
                    title: Text('密钥文件1'),
                    subtitle: Text('密钥文件1路径'),
                    onTap: () {},
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
