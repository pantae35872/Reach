import 'dart:io';
import 'dart:math';

import 'package:reach/ClientHandle.dart';
import 'package:reach/Encryption.dart';
import 'package:reach/Packet.dart';


List<int> generateRandomList(int length) {
  Random random = Random();
  List<int> list = [];

  for (int i = 0; i < length; i++) {
    list.add(random.nextInt(256));
  }

  return list;
}

class Client {
  static int id = 0;
  static TCP tcp = TCP();
  static List<int> key = generateRandomList(16);
  static List<int> iv = generateRandomList(16);
  static Map<S2CPacket, Function?> client_handle_map = {
    S2CPacket.GetId: ClientHandle.GetId,
  };
  static connect() {
    tcp.connect();
  }
}

class TCP {
  late Socket socket;
  void connect() async {
    socket = await Socket.connect('localhost', 8080);
    ReadData();
  }

  void ReadData() async {
    await socket.listen(
      (_data) {
        Packet packet = Packet.new_with_data(_data);
        bool encrypt = packet.read_bool();
        List<int> data = packet.read_bytes(packet.length() - 1);
        if (encrypt) {
          if (Client.key.isNotEmpty && Client.iv.isNotEmpty) {
            Encryption encryption = Encryption().set_key(Client.key).set_iv(Client.iv).set_data(data).aes_decrypt();
            data = encryption.build();
          } 
        }
        packet = Packet.new_with_data(data);
        int packet_id = packet.read_int();
        final fn = Client.client_handle_map[S2CPacket.values[packet_id]];
        if (fn != null) {
          fn(packet);
        }
      },
      onDone: () {
        ReadData();
      },
      onError: (e) {

      }
    );
  }
}