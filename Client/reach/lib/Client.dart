import 'dart:ffi';
import 'dart:io';

import 'package:reach/ClientHandle.dart';
import 'package:reach/Packet.dart';




class Client {
  static int id = 0;
  static TCP tcp = TCP();
  static Map<S2CPacket, Function?> client_handle_map = {
    S2CPacket.GetId: ClientHandle.GetId
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
      (data) {
        print("object");
        Packet packet = Packet.new_with_data(data);
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