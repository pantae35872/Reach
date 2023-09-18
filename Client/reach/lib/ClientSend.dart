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
        Encryption encryption = Encryption().set_key(Client.key).set_iv(Client.iv).set_data(data).aes_encrypt();
        data = encryption.build();
      }
    }
    packet.write_bool(encrypt);
    packet.write_bytes(data);
    Client.tcp.socket.add(Uint8List.fromList(packet.to_array()));
  }

  static Future<void> ReceiveKey() async {
    Packet packet = Packet.new_with_id(C2SPacket.ReceiveKey.index);
    Encryption encryption = Encryption();
    List<int> e_key = (await encryption.set_data(Client.key).rsa_encrypt()).build();
    List<int> e_iv = (await encryption.set_data(Client.iv).rsa_encrypt()).build();
    packet.write_int(e_key.length);
    packet.write_bytes(e_key);
    packet.write_int(e_iv.length);
    packet.write_bytes(e_iv);
    SendData(packet, false);
  }
}