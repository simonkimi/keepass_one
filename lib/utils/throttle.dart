import 'dart:async';

typedef ThrottleCallback = void Function();

ThrottleCallback throttle(Duration duration, ThrottleCallback callback) {
  Timer? timer;

  return () {
    timer?.cancel();
    timer = Timer(duration, callback);
  };
}
