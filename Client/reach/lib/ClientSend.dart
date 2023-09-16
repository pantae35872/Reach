import 'dart:typed_data';

import 'package:reach/Client.dart';
import 'package:reach/Packet.dart';

class ClientSend {
  static void SendData(Packet packet) {
    print(packet.bytes);
    Client.tcp.socket.add(Uint8List.fromList(packet.to_array()));
  }

  static void Welcome() {
    Packet packet = Packet.new_with_id(C2SPacket.Welcome.index);
    packet.write_string("a");
    SendData(packet);
  }
}