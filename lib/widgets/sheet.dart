import 'package:material_ui/material_ui.dart';

Future<T?> showAppModal<T>({
  required BuildContext context,
  required WidgetBuilder builder,
  bool useNestedNavigation = false,
  bool enableDrag = true,
  double maxWidth = 500,
  double maxHeight = 500,
}) {
  final isLargeScreen = MediaQuery.sizeOf(context).shortestSide >= 600;

  Widget buildContent() {
    return _maybeNested(
      builder: builder,
      useNestedNavigation: useNestedNavigation,
    );
  }

  if (isLargeScreen) {
    return showDialog<T>(
      context: context,
      useRootNavigator: true,
      builder: (context) {
        return Dialog(
          clipBehavior: Clip.antiAlias,
          child: SizedBox(
            width: maxWidth,
            height: maxHeight,
            child: buildContent(),
          ),
        );
      },
    );
  }

  return showModalBottomSheet<T>(
    context: context,
    useRootNavigator: true,
    isScrollControlled: true,
    enableDrag: enableDrag,
    showDragHandle: false,
    useSafeArea: true,
    builder: (context) {
      return SizedBox(
        height: MediaQuery.sizeOf(context).height * 0.92,
        child: buildContent(),
      );
    },
  );
}

Widget _maybeNested({
  required WidgetBuilder builder,
  required bool useNestedNavigation,
}) {
  if (!useNestedNavigation) {
    return Builder(builder: builder);
  }

  final nestedNavigatorKey = GlobalKey<NavigatorState>();
  return NavigatorPopHandler(
    onPopWithResult: (result) {
      nestedNavigatorKey.currentState?.maybePop();
    },
    child: Navigator(
      key: nestedNavigatorKey,
      onGenerateInitialRoutes: (navigator, initialRoute) {
        return <Route<void>>[
          MaterialPageRoute<void>(
            builder: (context) {
              return PopScope(
                canPop: false,
                onPopInvokedWithResult: (didPop, result) {
                  Navigator.of(context, rootNavigator: true).pop(result);
                },
                child: builder(context),
              );
            },
          ),
        ];
      },
    ),
  );
}
