import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:reach/Client.dart';
import 'package:reach/WindowsUI.dart';

void main() async {
  Client.connect();
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
