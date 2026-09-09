import 'package:flutter/gestures.dart';
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
  return Navigator.of(context, rootNavigator: true).push<T>(
    _AppModalRoute<T>(
      enableDrag: enableDrag,
      isLargeScreen: isLargeScreen,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
      builder: (context) => _maybeNested(
        builder: builder,
        useNestedNavigation: useNestedNavigation,
      ),
    ),
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

class _AppModalRoute<T> extends PopupRoute<T> {
  _AppModalRoute({
    required this.builder,
    required this.isLargeScreen,
    required this.enableDrag,
    required this.maxWidth,
    required this.maxHeight,
  });

  final WidgetBuilder builder;
  final bool isLargeScreen;
  final bool enableDrag;
  final double maxWidth;
  final double maxHeight;

  @override
  Color? get barrierColor => Colors.black54;

  @override
  bool get barrierDismissible => true;

  @override
  String? get barrierLabel => 'Dismiss';

  @override
  Duration get transitionDuration => const Duration(milliseconds: 280);

  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) {
    final theme = Theme.of(context);
    Widget panel = Material(
      color: theme.colorScheme.surface,
      clipBehavior: Clip.antiAlias,
      borderRadius: BorderRadius.circular(12),
      child: builder(context),
    );

    if (enableDrag) {
      panel = _DragToDismiss(child: panel);
    }

    if (isLargeScreen) {
      return Center(
        child: SizedBox(
          width: maxWidth,
          height: maxHeight,
          child: panel,
        ),
      );
    }

    return Align(
      alignment: Alignment.bottomCenter,
      child: SizedBox(
        height: MediaQuery.sizeOf(context).height * 0.92,
        width: double.infinity,
        child: panel,
      ),
    );
  }

  @override
  Widget buildTransitions(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    Widget child,
  ) {
    final curved = CurvedAnimation(
      parent: animation,
      curve: Curves.easeOutCubic,
      reverseCurve: Curves.easeInCubic,
    );
    if (isLargeScreen) {
      return FadeTransition(opacity: curved, child: child);
    }
    return SlideTransition(
      position: Tween<Offset>(
        begin: const Offset(0, 1),
        end: Offset.zero,
      ).animate(curved),
      child: child,
    );
  }
}

class _DragToDismiss extends StatefulWidget {
  const _DragToDismiss({required this.child});

  final Widget child;

  @override
  State<_DragToDismiss> createState() => _DragToDismissState();
}

class _DragToDismissState extends State<_DragToDismiss>
    with SingleTickerProviderStateMixin {
  late final VerticalDragGestureRecognizer _recognizer;
  late final AnimationController _snapController;
  Animation<double>? _snap;
  double _dy = 0;

  @override
  void initState() {
    super.initState();
    _recognizer = VerticalDragGestureRecognizer(debugOwner: this)
      ..onUpdate = _handleUpdate
      ..onEnd = _handleEnd
      ..onCancel = _handleCancel;
    _snapController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 200),
    )..addListener(() {
        final snap = _snap;
        if (snap != null) {
          setState(() => _dy = snap.value);
        }
      });
  }

  @override
  void dispose() {
    _recognizer.dispose();
    _snapController.dispose();
    super.dispose();
  }

  void _handleUpdate(DragUpdateDetails details) {
    _snapController.stop();
    setState(() {
      _dy = (_dy + details.delta.dy).clamp(0.0, double.infinity);
    });
  }

  void _handleEnd(DragEndDetails details) {
    final velocity = details.velocity.pixelsPerSecond.dy;
    final height = MediaQuery.sizeOf(context).height;
    if (_dy > height * 0.12 || velocity > 700) {
      Navigator.of(context, rootNavigator: true).pop();
      return;
    }
    _snapBack();
  }

  void _handleCancel() {
    _snapBack();
  }

  void _snapBack() {
    if (_dy == 0) return;
    _snap = Tween<double>(begin: _dy, end: 0).animate(
      CurvedAnimation(parent: _snapController, curve: Curves.easeOut),
    );
    _snapController.forward(from: 0);
  }

  @override
  Widget build(BuildContext context) {
    return Listener(
      onPointerDown: _recognizer.addPointer,
      behavior: HitTestBehavior.translucent,
      child: Transform.translate(
        offset: Offset(0, _dy),
        child: widget.child,
      ),
    );
  }
}
