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

class CupertinoTabletSheetRoute<T> extends PageRoute<T>
    with _CupertinoSheetRouteTransitionMixin<T> {
  CupertinoTabletSheetRoute({
    super.settings,
    required this.builder,
    this.enableDrag = true,
  });

  final WidgetBuilder builder;

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
      linearTransition: linearTransition,
      enableDrag: enableDrag,
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
    required this.child,
    required this.linearTransition,
    required this.enableDrag,
  });

  final Animation<double> primaryRouteAnimation;
  final Widget child;
  final bool linearTransition;
  final bool enableDrag;

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
  @override
  Widget build(BuildContext context) {
    return SizedBox.expand(
      child: _coverSheetPrimaryTransition(
        context,
        widget.primaryRouteAnimation,
        widget.linearTransition,
        buildContent(context),
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
        CupertinoSheetRoute.hasParentSheet(context)
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

  Widget buildContent(BuildContext context) {
    return Center(
      child: Container(
        constraints: BoxConstraints(maxWidth: 500, maxHeight: 500),
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

    // If this is disposed during a drag, call navigator.didStopUserGesture.
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
      // Divide by size of the sheet.
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
    // This can be called even if start is not called, paired with the "down" event
    // that we don't consider here.
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
  /// Creates a controller for an iOS-style back gesture.
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

  /// The drag gesture has changed by [delta]. The total range of the drag
  /// should be 0.0 to 1.0.
  void dragUpdate(double delta) {
    controller.value -= delta;
  }

  /// The drag gesture has ended with a vertical motion of [velocity] as a
  /// fraction of screen height per second.
  void dragEnd(double velocity) {
    // Fling in the appropriate direction.
    //
    // This curve has been determined through rigorously eyeballing native iOS
    // animations on a simulator running iOS 18.0.
    const Curve animationCurve = Curves.easeOut;
    final bool isCurrent = getIsCurrent();
    final bool animateForward;

    if (!isCurrent) {
      // If the page has already been navigated away from, then the animation
      // direction depends on whether or not it's still in the navigation stack,
      // regardless of velocity or drag position. For example, if a route is
      // being slowly dragged back by just a few pixels, but then a programmatic
      // pop occurs, the route should still be animated off the screen.
      // See https://github.com/flutter/flutter/issues/141268.
      animateForward = getIsActive();
    } else if (velocity.abs() >= _kMinFlingVelocity) {
      // If the user releases the page before mid screen with sufficient velocity,
      // or after mid screen, we should animate the page out. Otherwise, the page
      // should be animated back in.
      animateForward = velocity <= 0;
    } else {
      // If the drag is dropped with low velocity, the sheet will pop if the
      // the drag goes a little past the halfway point on the screen. This is
      // eyeballed on a simulator running iOS 18.0.
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
        // This route is destined to pop at this point. Reuse navigator's pop.
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
      // Keep the userGestureInProgress in true state so we don't change the
      // curve of the page transition mid-flight since CupertinoPageTransition
      // depends on userGestureInProgress.
      // late AnimationStatusListener animationStatusCallback;
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
