import 'dart:typed_data';
import 'package:crypto/crypto.dart';

import 'dart:convert';

String hashU8List(Uint8List data) => base64.encode(sha256.convert(data).bytes);
