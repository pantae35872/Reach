import 'dart:typed_data';

import 'package:reach/Client.dart';
import 'package:reach/Encryption.dart';
import 'package:reach/Packet.dart';

class ClientSend {
  static void SendData(Packet _packet, bool encrypt) {
    List<int> data = _packet.to_array();
    Packet packet = Packet();
    if (encrypt) {
      if (Client.key.isNotEmpty && Client.iv.isNotEmpty) {
        Encryption encryption = Encryption().set_key(Client.key).set_iv(Client.iv).aes_encrypt();
        data = encryption.build();
      }
    }
    packet.write_bool(encrypt);
    packet.write_bytes(data);
    Client.tcp.socket.add(Uint8List.fromList(packet.to_array()));
  }

  static void Welcome() {
    Packet packet = Packet.new_with_id(C2SPacket.Welcome.index);
    packet.write_string("a");
    SendData(packet, false);
  }
}