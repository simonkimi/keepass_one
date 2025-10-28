import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';

final Animatable<Offset> _kBottomUpTweenWhenCoveringOtherSheet = Tween<Offset>(
  begin: const Offset(0.0, 1.0),
  end: const Offset(0.0, -0.02),
);

final Animatable<Offset> _kBottomUpTween = Tween<Offset>(
  begin: const Offset(0.0, 1.0),
  end: Offset.zero,
);

final Animatable<Offset> _kMidUpTween = Tween<Offset>(
  begin: Offset.zero,
  end: const Offset(0.0, -0.005),
);

final Animatable<double> _kOpacityTween = Tween<double>(begin: 0.0, end: 0.3);

const double _kSheetScaleFactor = 0.0835;

final Animatable<double> _kScaleTween = Tween<double>(
  begin: 1.0,
  end: 1.0 - _kSheetScaleFactor,
);

const double _kTopGapRatio = 0.08;

const double _kMinFlingVelocity = 2.0; // Screen heights per second.

const Duration _kDroppedSheetDragAnimationDuration = Duration(
  milliseconds: 300,
);

/// [CupertinoTabletSheetRoute] 是一个类似于 CupertinoSheetRoute 的实现, 它的作用是
/// 在大屏设备上, 例如PC或者平板电脑上, 提供一个居中的Modal Sheet.
class CupertinoTabletSheetRoute<T> extends PageRoute<T>
    with _CupertinoSheetRouteTransitionMixin<T> {
  CupertinoTabletSheetRoute({
    super.settings,
    required this.builder,
    this.enableDrag = true,
    this.maxWidth = 500,
    this.maxHeight = 500,
  });

  final WidgetBuilder builder;
  @override
  final double maxWidth;
  @override
  final double maxHeight;

  @override
  final bool enableDrag;

  static bool hasParentSheet(BuildContext context) {
    return _CupertinoTabletSheetScope.maybeOf(context) != null;
  }

  @override
  Widget buildContent(BuildContext context) {
    return ClipRSuperellipse(
      borderRadius: const BorderRadius.all(Radius.circular(12)),
      child: CupertinoUserInterfaceLevel(
        data: CupertinoUserInterfaceLevelData.elevated,
        child: _CupertinoTabletSheetScope(child: builder(context)),
      ),
    );
  }

  @override
  Color? get barrierColor => CupertinoColors.transparent;

  @override
  String? get barrierLabel => null;

  @override
  bool get maintainState => true;

  @override
  bool get opaque => false;
}

class _CupertinoTabletSheetScope extends InheritedWidget {
  const _CupertinoTabletSheetScope({required super.child});

  static _CupertinoTabletSheetScope? maybeOf(BuildContext context) {
    return context.getInheritedWidgetOfExactType<_CupertinoTabletSheetScope>();
  }

  @override
  bool updateShouldNotify(_CupertinoTabletSheetScope oldWidget) => false;
}

mixin _CupertinoSheetRouteTransitionMixin<T> on PageRoute<T> {
  @protected
  Widget buildContent(BuildContext context);

  @override
  Duration get transitionDuration => const Duration(milliseconds: 500);

  @override
  DelegatedTransitionBuilder? get delegatedTransition =>
      CupertinoTabletSheetTransition.delegateTransition;

  bool get enableDrag;

  double get maxWidth;
  double get maxHeight;

  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) {
    return buildContent(context);
  }

  @override
  Widget buildTransitions(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    Widget child,
  ) {
    final bool linearTransition = popGestureInProgress;
    return CupertinoTabletSheetTransition(
      primaryRouteAnimation: animation,
      secondaryRouteAnimation: secondaryAnimation,
      linearTransition: linearTransition,
      enableDrag: enableDrag,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
      child: _CupertinoDownGestureDetector(
        enabledCallback: () => enableDrag,
        onStartPopGesture: () => _startPopGesture<T>(this),
        child: child,
      ),
    );
  }

  static _CupertinoDownGestureController<T> _startPopGesture<T>(
    ModalRoute<T> route,
  ) {
    return _CupertinoDownGestureController<T>(
      navigator: route.navigator!,
      getIsCurrent: () => route.isCurrent,
      getIsActive: () => route.isActive,
      controller: route.controller!, // protected access
    );
  }
}

class CupertinoTabletSheetTransition extends StatefulWidget {
  const CupertinoTabletSheetTransition({
    super.key,
    required this.primaryRouteAnimation,
    required this.secondaryRouteAnimation,
    required this.child,
    required this.linearTransition,
    required this.enableDrag,
    required this.maxWidth,
    required this.maxHeight,
  });

  final Animation<double> primaryRouteAnimation;
  final Animation<double> secondaryRouteAnimation;
  final Widget child;
  final bool linearTransition;
  final bool enableDrag;
  final double maxWidth;
  final double maxHeight;

  static Widget delegateTransition(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    bool allowSnapshotting,
    Widget? child,
  ) {
    if (CupertinoTabletSheetRoute.hasParentSheet(context)) {
      return _delegatedCoverSheetSecondaryTransition(secondaryAnimation, child);
    }
    final bool linear = Navigator.of(context).userGestureInProgress;
    final Curve curve = linear ? Curves.linear : Curves.linearToEaseOut;
    final Curve reverseCurve = linear ? Curves.linear : Curves.easeInToLinear;
    final CurvedAnimation curvedAnimation = CurvedAnimation(
      curve: curve,
      reverseCurve: reverseCurve,
      parent: secondaryAnimation,
    );

    final Animation<double> opacityAnimation = curvedAnimation.drive(
      _kOpacityTween,
    );
    final bool isDarkMode =
        CupertinoTheme.brightnessOf(context) == Brightness.dark;
    final Color overlayColor = isDarkMode
        ? const Color(0xFFc8c8c8)
        : const Color(0xFF000000);

    return Stack(
      children: <Widget>[
        ?child,
        FadeTransition(
          opacity: opacityAnimation,
          child: ColoredBox(
            color: overlayColor,
            child: const SizedBox.expand(),
          ),
        ),
      ],
    );
  }

  static Widget _delegatedCoverSheetSecondaryTransition(
    Animation<double> secondaryAnimation,
    Widget? child,
  ) {
    const Curve curve = Curves.linearToEaseOut;
    const Curve reverseCurve = Curves.easeInToLinear;
    final CurvedAnimation curvedAnimation = CurvedAnimation(
      curve: curve,
      reverseCurve: reverseCurve,
      parent: secondaryAnimation,
    );

    final Animation<Offset> slideAnimation = curvedAnimation.drive(
      _kMidUpTween,
    );
    final Animation<double> scaleAnimation = curvedAnimation.drive(
      _kScaleTween,
    );
    curvedAnimation.dispose();

    return SlideTransition(
      position: slideAnimation,
      transformHitTests: false,
      child: ScaleTransition(
        scale: scaleAnimation,
        filterQuality: FilterQuality.medium,
        alignment: Alignment.topCenter,
        child: ClipRSuperellipse(
          borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
          child: child,
        ),
      ),
    );
  }

  @override
  State<CupertinoTabletSheetTransition> createState() =>
      _CupertinoTabletSheetTransitionState();
}

class _CupertinoTabletSheetTransitionState
    extends State<CupertinoTabletSheetTransition> {
  late Animation<Offset> _secondaryPositionAnimation;
  late Animation<double> _secondaryScaleAnimation;
  CurvedAnimation? _primaryPositionCurve;
  CurvedAnimation? _secondaryPositionCurve;

  @override
  void initState() {
    super.initState();
    _setupAnimation();
  }

  @override
  void didUpdateWidget(covariant CupertinoTabletSheetTransition oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.primaryRouteAnimation != widget.primaryRouteAnimation ||
        oldWidget.secondaryRouteAnimation != widget.secondaryRouteAnimation) {
      _disposeCurve();
      _setupAnimation();
    }
  }

  @override
  void dispose() {
    _disposeCurve();
    super.dispose();
  }

  void _setupAnimation() {
    _primaryPositionCurve = CurvedAnimation(
      curve: Curves.fastEaseInToSlowEaseOut,
      reverseCurve: Curves.fastEaseInToSlowEaseOut.flipped,
      parent: widget.primaryRouteAnimation,
    );
    _secondaryPositionCurve = CurvedAnimation(
      curve: Curves.linearToEaseOut,
      reverseCurve: Curves.easeInToLinear,
      parent: widget.secondaryRouteAnimation,
    );
    _secondaryPositionAnimation = _secondaryPositionCurve!.drive(_kMidUpTween);
    _secondaryScaleAnimation = _secondaryPositionCurve!.drive(_kScaleTween);
  }

  void _disposeCurve() {
    _primaryPositionCurve?.dispose();
    _secondaryPositionCurve?.dispose();
    _primaryPositionCurve = null;
    _secondaryPositionCurve = null;
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox.expand(
      child: _coverSheetSecondaryTransition(
        widget.secondaryRouteAnimation,
        _coverSheetPrimaryTransition(
          context,
          widget.primaryRouteAnimation,
          widget.linearTransition,
          buildContent(context),
        ),
      ),
    );
  }

  Widget _coverSheetPrimaryTransition(
    BuildContext context,
    Animation<double> animation,
    bool linearTransition,
    Widget? child,
  ) {
    final Animatable<Offset> offsetTween =
        CupertinoTabletSheetRoute.hasParentSheet(context)
        ? _kBottomUpTweenWhenCoveringOtherSheet
        : _kBottomUpTween;

    final CurvedAnimation curvedAnimation = CurvedAnimation(
      parent: animation,
      curve: linearTransition ? Curves.linear : Curves.fastEaseInToSlowEaseOut,
      reverseCurve: linearTransition
          ? Curves.linear
          : Curves.fastEaseInToSlowEaseOut.flipped,
    );

    final Animation<Offset> positionAnimation = curvedAnimation.drive(
      offsetTween,
    );

    curvedAnimation.dispose();

    return SlideTransition(position: positionAnimation, child: child);
  }

  Widget _coverSheetSecondaryTransition(
    Animation<double> secondaryAnimation,
    Widget? child,
  ) {
    return SlideTransition(
      position: _secondaryPositionAnimation,
      transformHitTests: false,
      child: ScaleTransition(
        scale: _secondaryScaleAnimation,
        filterQuality: FilterQuality.medium,
        alignment: Alignment.topCenter,
        child: child,
      ),
    );
  }

  Widget buildContent(BuildContext context) {
    return Center(
      child: Container(
        constraints: BoxConstraints(
          maxWidth: widget.maxWidth,
          maxHeight: widget.maxHeight,
        ),
        decoration: BoxDecoration(
          boxShadow: [
            BoxShadow(
              color: CupertinoColors.black.withValues(alpha: 0.1),
              blurRadius: 20.0,
              spreadRadius: 0.0,
              offset: const Offset(0, 20),
            ),
          ],
        ),
        child: widget.child,
      ),
    );
  }
}

class _CupertinoDownGestureDetector<T> extends StatefulWidget {
  const _CupertinoDownGestureDetector({
    super.key,
    required this.enabledCallback,
    required this.onStartPopGesture,
    required this.child,
  });

  final Widget child;

  final ValueGetter<bool> enabledCallback;

  final ValueGetter<_CupertinoDownGestureController<T>> onStartPopGesture;

  @override
  _CupertinoDownGestureDetectorState<T> createState() =>
      _CupertinoDownGestureDetectorState<T>();
}

class _CupertinoDownGestureDetectorState<T>
    extends State<_CupertinoDownGestureDetector<T>> {
  _CupertinoDownGestureController<T>? _downGestureController;

  late VerticalDragGestureRecognizer _recognizer;

  @override
  void initState() {
    super.initState();
    _recognizer = VerticalDragGestureRecognizer(debugOwner: this)
      ..onStart = _handleDragStart
      ..onUpdate = _handleDragUpdate
      ..onEnd = _handleDragEnd
      ..onCancel = _handleDragCancel;
  }

  @override
  void dispose() {
    _recognizer.dispose();

    if (_downGestureController != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (_downGestureController?.navigator.mounted ?? false) {
          _downGestureController?.navigator.didStopUserGesture();
        }
        _downGestureController = null;
      });
    }
    super.dispose();
  }

  void _handleDragStart(DragStartDetails details) {
    assert(mounted);
    assert(_downGestureController == null);
    _downGestureController = widget.onStartPopGesture();
  }

  void _handleDragUpdate(DragUpdateDetails details) {
    assert(mounted);
    assert(_downGestureController != null);
    _downGestureController!.dragUpdate(
      details.primaryDelta! /
          (context.size!.height - (context.size!.height * _kTopGapRatio)),
    );
  }

  void _handleDragEnd(DragEndDetails details) {
    assert(mounted);
    assert(_downGestureController != null);
    _downGestureController!.dragEnd(
      details.velocity.pixelsPerSecond.dy / context.size!.height,
    );
    _downGestureController = null;
  }

  void _handleDragCancel() {
    assert(mounted);
    _downGestureController?.dragEnd(0.0);
    _downGestureController = null;
  }

  void _handlePointerDown(PointerDownEvent event) {
    if (widget.enabledCallback()) {
      _recognizer.addPointer(event);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Listener(
      onPointerDown: _handlePointerDown,
      behavior: HitTestBehavior.translucent,
      child: widget.child,
    );
  }
}

class _CupertinoDownGestureController<T> {
  _CupertinoDownGestureController({
    required this.navigator,
    required this.controller,
    required this.getIsActive,
    required this.getIsCurrent,
  }) {
    navigator.didStartUserGesture();
  }

  final AnimationController controller;
  final NavigatorState navigator;
  final ValueGetter<bool> getIsActive;
  final ValueGetter<bool> getIsCurrent;

  void dragUpdate(double delta) {
    controller.value -= delta;
  }

  void dragEnd(double velocity) {
    const Curve animationCurve = Curves.easeOut;
    final bool isCurrent = getIsCurrent();
    final bool animateForward;

    if (!isCurrent) {
      animateForward = getIsActive();
    } else if (velocity.abs() >= _kMinFlingVelocity) {
      animateForward = velocity <= 0;
    } else {
      animateForward = controller.value > 0.52;
    }

    if (animateForward) {
      controller.animateTo(
        1.0,
        duration: _kDroppedSheetDragAnimationDuration,
        curve: animationCurve,
      );
    } else {
      if (isCurrent) {
        final NavigatorState rootNavigator = Navigator.of(
          navigator.context,
          rootNavigator: true,
        );
        rootNavigator.pop();
      }

      if (controller.isAnimating) {
        controller.animateBack(
          0.0,
          duration: _kDroppedSheetDragAnimationDuration,
          curve: animationCurve,
        );
      }
    }

    if (controller.isAnimating) {
      void animationStatusCallback(AnimationStatus status) {
        navigator.didStopUserGesture();
        controller.removeStatusListener(animationStatusCallback);
      }

      controller.addStatusListener(animationStatusCallback);
    } else {
      navigator.didStopUserGesture();
    }
  }
}

/// 无动画的 PageRoute，用于嵌套 Navigator 的初始路由
/// 这样在 pop 初始路由时不会有动画冲突
class _NoAnimationPageRoute<T> extends PageRoute<T> {
  _NoAnimationPageRoute({required this.builder, super.settings});

  final WidgetBuilder builder;

  @override
  Color? get barrierColor => null;

  @override
  String? get barrierLabel => null;

  @override
  bool get maintainState => true;

  @override
  Duration get transitionDuration => Duration.zero;

  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) {
    return builder(context);
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

Future<T?> showCupertinoTabletSheet<T>({
  required BuildContext context,
  required WidgetBuilder builder,
  bool useNestedNavigation = false,
  bool enableDrag = true,
  double maxWidth = 500,
  double maxHeight = 500,
}) {
  final GlobalKey<NavigatorState> nestedNavigatorKey =
      GlobalKey<NavigatorState>();
  final WidgetBuilder widgetBuilder;
  if (!useNestedNavigation) {
    widgetBuilder = builder;
  } else {
    widgetBuilder = (BuildContext context) {
      return NavigatorPopHandler(
        onPopWithResult: (T? result) {
          nestedNavigatorKey.currentState!.maybePop();
        },
        child: Navigator(
          key: nestedNavigatorKey,
          onGenerateInitialRoutes: (navigator, initialRoute) {
            return <Route<void>>[
              CupertinoPageRoute<void>(
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
    };
  }

  return Navigator.of(context, rootNavigator: true).push<T>(
    CupertinoTabletSheetRoute<T>(
      builder: widgetBuilder,
      enableDrag: enableDrag,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
    ),
  );
}

Future<T?> showCupertinoModal<T>({
  required BuildContext context,
  required WidgetBuilder builder,
  bool useNestedNavigation = false,
  bool enableDrag = true,
  double maxWidth = 500,
  double maxHeight = 500,
}) {
  final isLargeScreen = MediaQuery.of(context).size.shortestSide >= 600;
  if (isLargeScreen) {
    return showCupertinoTabletSheet(
      context: context,
      builder: builder,
      useNestedNavigation: useNestedNavigation,
      enableDrag: enableDrag,
      maxWidth: maxWidth,
      maxHeight: maxHeight,
    );
  } else {
    return showCupertinoSheet(
      context: context,
      builder: builder,
      enableDrag: enableDrag,
      useNestedNavigation: useNestedNavigation,
    );
  }
}
