import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:reach/WindowsUI.dart';
import 'package:cryptography/cryptography.dart';

void main() async {
  final message = utf8.encode("hello world! this is my plaintext.");

  // AES-CBC with 128 bit keys and HMAC-SHA256 authentication.
  final algorithm = AesCbc.with256bits(
    macAlgorithm: Hmac.sha256(),
  );
  final a = <int>[];
  for (int i = 0; i < 32; i++) {
    a.add(0);
  }
  final secretKey = await algorithm.newSecretKeyFromBytes(a);
  final nonce = <int>[];
  for (int i = 0; i < 16; i++) {
    nonce.add(0);
  }

  // Encrypt
  final secretBox = await algorithm.encrypt(
    message,
    secretKey: secretKey,
    nonce: nonce
  );
  print('Text: ${message}');
  print('Nonce: ${secretBox.nonce}');
  print('Ciphertext: ${secretBox.cipherText}');
  print('MAC: ${secretBox.mac.bytes}');

  // Decrypt
  final clearText = await algorithm.decrypt(
    secretBox,
    secretKey: secretKey,
    aad: nonce    
  );
  print('Cleartext: ${clearText}');


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
