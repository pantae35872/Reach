import 'package:reach/Client.dart';
import 'package:reach/ClientSend.dart';
import 'package:reach/Packet.dart';

class ClientHandle {
  static void GetId(Packet packet) {
    int id = packet.read_int();
    Client.id = id;
    ClientSend.ReceiveKey();
  }
}