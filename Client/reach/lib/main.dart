import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:reach/Encryption.dart';
import 'package:reach/WindowsUI.dart';

void main() async {
  final message = utf8.encode("hello world! this is my plaintext.");

  final key = <int>[];
  for (int i = 0; i < 16; i++) {
    key.add(0);
  }
  final iv = <int>[];
  for (int i = 0; i < 16; i++) {
    iv.add(0);
  }

  Encryption encryption = Encryption();
  encryption.set_key(key).set_iv(iv).set_data(message);

  encryption.aes_encrypt();

  print(encryption.build());

  encryption.aes_decrypt();

  print(String.fromCharCodes(encryption.build()));

  if (defaultTargetPlatform == TargetPlatform.windows) 
  {
    runApp(const WindowsMain());
    doWhenWindowReady(() {
      Size initialSize = Size(600, 600);
      appWindow.size = initialSize;
      appWindow.minSize = initialSize;
      appWindow.title = "Reach";
      appWindow.show();
    });
  }
}
