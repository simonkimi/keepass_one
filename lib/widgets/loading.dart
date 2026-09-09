import 'package:material_ui/material_ui.dart';

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
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 32),
          child: LinearProgressIndicator(
            value: loadingProgress < 0.0 ? null : loadingProgress,
          ),
        ),
        const SizedBox(height: 16.0),
        Text(message),
      ],
    );
  }
}
