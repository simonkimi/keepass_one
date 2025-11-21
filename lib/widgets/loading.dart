import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class LoadingWidget extends StatelessWidget {
  final String message;
  final double loadingProgress;

  const LoadingWidget({
    super.key,
    required this.message,
    required this.loadingProgress,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        LinearProgressIndicator(
          value: loadingProgress < 0.0 ? null : loadingProgress,
          color: CupertinoColors.activeBlue.resolveFrom(context),
        ),
        SizedBox(height: 16.0),
        Text(message),
      ],
    );
  }
}
