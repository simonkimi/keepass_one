import 'dart:ui';
import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';

/// InheritedWidget to track if we're inside a tablet sheet route.
class _CupertinoTabletSheetScope extends InheritedWidget {
  const _CupertinoTabletSheetScope({required super.child});

  static bool isInsideSheet(BuildContext context) {
    return context
            .dependOnInheritedWidgetOfExactType<_CupertinoTabletSheetScope>() !=
        null;
  }

  @override
  bool updateShouldNotify(_CupertinoTabletSheetScope oldWidget) => false;
}

/// A modal route that displays a centered sheet on tablet/large screen devices.
///
/// This route mimics the iPad sheet behavior where the sheet appears centered
/// on screen with a semi-transparent backdrop, and can be dismissed by dragging
/// down or tapping outside.
class CupertinoTabletSheetRoute<T> extends PopupRoute<T> {
  CupertinoTabletSheetRoute({
    super.settings,
    required this.builder,
    this.enableDrag = true,
    this.backgroundColor,
    this.borderRadius = 12.0,
    this.maxWidth = 540.0,
    this.maxHeight = 620.0,
    this.barrierDismissible = true,
  });

  final WidgetBuilder builder;
  final bool enableDrag;
  final Color? backgroundColor;
  final double borderRadius;
  final double maxWidth;
  final double maxHeight;

  @override
  final bool barrierDismissible;

  /// Check if we're already inside a sheet to avoid stacking barriers
  bool _isInsideSheet = false;

  @override
  Color? get barrierColor =>
      _isInsideSheet ? null : CupertinoColors.black.withValues(alpha: 0.4);

  @override
  String? get barrierLabel => 'Dismiss';

  @override
  Duration get transitionDuration => const Duration(milliseconds: 350);

  @override
  Duration get reverseTransitionDuration => const Duration(milliseconds: 300);

  /// Pops the current sheet route if it exists in the navigation stack.
  ///
  /// This is useful when you have nested navigation within a sheet and want
  /// to dismiss the entire sheet at once.
  static void popSheet<T>(BuildContext context, [T? result]) {
    // Find the nearest CupertinoTabletSheetRoute and pop it
    Navigator.of(context).popUntil((route) {
      if (route is CupertinoTabletSheetRoute) {
        Navigator.of(context).pop(result);
        return true;
      }
      return false;
    });
  }

  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) {
    // Check if we're already inside a sheet
    _isInsideSheet = _CupertinoTabletSheetScope.isInsideSheet(context);

    return _CupertinoTabletSheetScope(
      child: _CupertinoTabletSheetContent<T>(
        route: this,
        animation: animation,
        builder: builder,
        enableDrag: enableDrag,
        backgroundColor: backgroundColor,
        borderRadius: borderRadius,
        maxWidth: maxWidth,
        maxHeight: maxHeight,
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
    return child;
  }
}

class _CupertinoTabletSheetContent<T> extends StatefulWidget {
  const _CupertinoTabletSheetContent({
    required this.route,
    required this.animation,
    required this.builder,
    required this.enableDrag,
    this.backgroundColor,
    required this.borderRadius,
    required this.maxWidth,
    required this.maxHeight,
  });

  final CupertinoTabletSheetRoute<T> route;
  final Animation<double> animation;
  final WidgetBuilder builder;
  final bool enableDrag;
  final Color? backgroundColor;
  final double borderRadius;
  final double maxWidth;
  final double maxHeight;

  @override
  State<_CupertinoTabletSheetContent<T>> createState() =>
      _CupertinoTabletSheetContentState<T>();
}

class _CupertinoTabletSheetContentState<T>
    extends State<_CupertinoTabletSheetContent<T>>
    with SingleTickerProviderStateMixin {
  late AnimationController _dragController;
  late Animation<double> _scaleAnimation;
  late Animation<double> _opacityAnimation;
  late Animation<Offset> _slideAnimation;

  VerticalDragGestureRecognizer? _dragRecognizer;
  double _dragOffset = 0.0;
  bool _isDragging = false;

  static const double _kMinFlingVelocity = 700.0; // pixels per second
  static const double _kDismissThreshold = 0.15; // 15% of sheet height

  @override
  void initState() {
    super.initState();
    _dragController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );

    if (widget.enableDrag) {
      _dragRecognizer = VerticalDragGestureRecognizer()
        ..onStart = _handleDragStart
        ..onUpdate = _handleDragUpdate
        ..onEnd = _handleDragEnd;
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    final Animation<double> routeAnimation = widget.animation;

    // Scale animation: starts at 0.9 and animates to 1.0
    _scaleAnimation = Tween<double>(begin: 0.9, end: 1.0).animate(
      CurvedAnimation(parent: routeAnimation, curve: Curves.easeOutCubic),
    );

    // Opacity animation: fades in from 0 to 1
    _opacityAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: routeAnimation,
        curve: const Interval(0.0, 0.6, curve: Curves.easeOut),
      ),
    );

    // Slide animation: slides up slightly from bottom
    _slideAnimation =
        Tween<Offset>(begin: const Offset(0.0, 0.1), end: Offset.zero).animate(
          CurvedAnimation(parent: routeAnimation, curve: Curves.easeOutCubic),
        );
  }

  @override
  void dispose() {
    _dragController.dispose();
    _dragRecognizer?.dispose();
    super.dispose();
  }

  void _handleDragStart(DragStartDetails details) {
    _isDragging = true;
    widget.route.navigator?.didStartUserGesture();
  }

  void _handleDragUpdate(DragUpdateDetails details) {
    if (!_isDragging) return;

    setState(() {
      // Only allow dragging downward
      _dragOffset = (_dragOffset + details.primaryDelta!).clamp(
        0.0,
        double.infinity,
      );
    });
  }

  void _handleDragEnd(DragEndDetails details) {
    if (!_isDragging) return;
    _isDragging = false;

    final double velocity = details.velocity.pixelsPerSecond.dy;
    final double screenHeight = MediaQuery.of(context).size.height;
    final double dragPercent = _dragOffset / screenHeight;

    // Dismiss if dragged down enough or flung down with sufficient velocity
    final bool shouldDismiss =
        dragPercent > _kDismissThreshold || velocity > _kMinFlingVelocity;

    if (shouldDismiss) {
      // Animate out and pop
      Navigator.of(context).pop();
    } else {
      // Animate back to original position
      setState(() {
        _dragOffset = 0.0;
      });
    }

    widget.route.navigator?.didStopUserGesture();
  }

  void _handlePointerDown(PointerDownEvent event) {
    if (widget.enableDrag) {
      _dragRecognizer?.addPointer(event);
    }
  }

  @override
  Widget build(BuildContext context) {
    final Color effectiveBackgroundColor =
        widget.backgroundColor ??
        CupertinoTheme.of(context).scaffoldBackgroundColor;

    return AnimatedBuilder(
      animation: widget.animation,
      builder: (context, child) {
        final double dragProgress =
            _dragOffset / MediaQuery.of(context).size.height;

        return Center(
          child: Transform.translate(
            offset: Offset(
              _slideAnimation.value.dx * MediaQuery.of(context).size.width,
              (_slideAnimation.value.dy * MediaQuery.of(context).size.height) +
                  _dragOffset,
            ),
            child: Transform.scale(
              scale: _scaleAnimation.value * (1.0 - dragProgress * 0.15),
              child: Opacity(
                opacity:
                    _opacityAnimation.value *
                    (1.0 - dragProgress * 2.0).clamp(0.0, 1.0),
                child: Listener(
                  onPointerDown: _handlePointerDown,
                  behavior: HitTestBehavior.translucent,
                  child: Container(
                    constraints: BoxConstraints(
                      maxWidth: widget.maxWidth,
                      maxHeight: widget.maxHeight,
                    ),
                    margin: const EdgeInsets.symmetric(
                      horizontal: 32.0,
                      vertical: 48.0,
                    ),
                    decoration: BoxDecoration(
                      color: effectiveBackgroundColor,
                      borderRadius: BorderRadius.circular(widget.borderRadius),
                      boxShadow: [
                        BoxShadow(
                          color: CupertinoColors.black.withValues(alpha: 0.3),
                          blurRadius: 40.0,
                          spreadRadius: 0.0,
                          offset: const Offset(0, 20),
                        ),
                      ],
                    ),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(widget.borderRadius),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          // Drag handle
                          if (widget.enableDrag)
                            Padding(
                              padding: const EdgeInsets.only(top: 8.0),
                              child: Container(
                                width: 36.0,
                                height: 5.0,
                                decoration: BoxDecoration(
                                  color: CupertinoColors.systemGrey3,
                                  borderRadius: BorderRadius.circular(2.5),
                                ),
                              ),
                            ),
                          // Content
                          Flexible(child: widget.builder(context)),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

/// Shows a tablet-optimized sheet that appears centered on the screen.
///
/// Returns a [Future] that resolves to the value passed to [Navigator.pop]
/// when the sheet is closed.
///
/// The `useNestedNavigation` parameter allows new routes to be pushed inside
/// of the sheet by adding a new [Navigator] inside. When set to `true`, any
/// route pushed from within the sheet will display within that sheet.
///
/// The whole sheet can be popped at once by calling [CupertinoTabletSheetRoute.popSheet].
Future<T?> showCupertinoTabletSheet<T>({
  required BuildContext context,
  required WidgetBuilder builder,
  bool enableDrag = true,
  Color? backgroundColor,
  double borderRadius = 12.0,
  double maxWidth = 540.0,
  double maxHeight = 620.0,
  bool barrierDismissible = true,
  bool useRootNavigator = true,
  bool useNestedNavigation = false,
}) {
  final GlobalKey<NavigatorState> nestedNavigatorKey =
      GlobalKey<NavigatorState>();
  final WidgetBuilder effectiveBuilder;

  if (!useNestedNavigation) {
    effectiveBuilder = builder;
  } else {
    effectiveBuilder = (BuildContext context) {
      return PopScope(
        canPop: false,
        onPopInvokedWithResult: (bool didPop, dynamic result) {
          if (didPop) return;

          // If there are routes to pop in the nested navigator, pop them
          if (nestedNavigatorKey.currentState?.canPop() ?? false) {
            nestedNavigatorKey.currentState!.pop();
          } else {
            // Otherwise pop the whole sheet
            Navigator.of(context).pop();
          }
        },
        child: Navigator(
          key: nestedNavigatorKey,
          onGenerateRoute: (RouteSettings settings) {
            return CupertinoPageRoute<dynamic>(
              settings: settings,
              builder: (BuildContext context) => builder(context),
            );
          },
        ),
      );
    };
  }

  return Navigator.of(context, rootNavigator: useRootNavigator).push<T>(
    CupertinoTabletSheetRoute<T>(
      builder: effectiveBuilder,
      enableDrag: enableDrag,
      backgroundColor: backgroundColor,
      borderRadius: borderRadius,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
      barrierDismissible: barrierDismissible,
    ),
  );
}

/// Breakpoint for determining when to use tablet layout (in logical pixels).
const double _kTabletBreakpoint = 600.0;

/// Shows an adaptive sheet that automatically chooses the appropriate style
/// based on screen size.
///
/// On mobile devices (width < 600dp), uses [CupertinoSheetRoute] which slides
/// up from the bottom.
///
/// On tablets and desktop devices (width >= 600dp), uses
/// [CupertinoTabletSheetRoute] which shows a centered modal.
///
/// Returns a [Future] that resolves to the value passed to [Navigator.pop]
/// when the sheet is closed.
///
/// Example:
/// ```dart
/// showCupertinoAdaptiveSheet(
///   context: context,
///   builder: (context) => YourSheetContent(),
/// );
/// ```
Future<T?> showCupertinoAdaptiveSheet<T>({
  required BuildContext context,
  required WidgetBuilder builder,
  bool enableDrag = true,
  Color? backgroundColor,
  double borderRadius = 12.0,
  double maxWidth = 540.0,
  double maxHeight = 620.0,
  bool barrierDismissible = true,
  bool useRootNavigator = true,
  bool useNestedNavigation = false,
}) {
  final double screenWidth = MediaQuery.of(context).size.width;
  final bool isTabletOrDesktop = screenWidth >= _kTabletBreakpoint;

  if (isTabletOrDesktop) {
    // Use tablet/desktop centered modal sheet
    return showCupertinoTabletSheet<T>(
      context: context,
      builder: builder,
      enableDrag: enableDrag,
      backgroundColor: backgroundColor,
      borderRadius: borderRadius,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
      barrierDismissible: barrierDismissible,
      useRootNavigator: useRootNavigator,
      useNestedNavigation: useNestedNavigation,
    );
  } else {
    // Use mobile bottom sheet
    return showCupertinoSheet<T>(
      context: context,
      builder: builder,
      enableDrag: enableDrag,
      useNestedNavigation: useNestedNavigation,
    );
  }
}
