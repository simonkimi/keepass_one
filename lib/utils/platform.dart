import 'dart:io';

import 'package:flutter/widgets.dart';

List<Widget> platformActionsBaseOnMobile(List<Widget> widgets) {
  if (Platform.isWindows || Platform.isLinux) {
    return widgets.reversed.toList();
  }
  return widgets;
}
