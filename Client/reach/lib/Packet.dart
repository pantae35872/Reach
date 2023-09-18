import 'dart:convert';
import 'dart:typed_data';

const BUFFER_LIMIT = 4096;

enum C2SPacket {
  ReceiveKey,
}
enum S2CPacket {
  GetId,
}
class Packet {
  List<int> bytes = [];
  int readPos = 0;

  Packet.new() {

  }

  Packet.new_with_data(List<int> data) {
    if (data.length <= BUFFER_LIMIT - bytes.length) {
      bytes.addAll(data);
    }
  }

  Packet.new_with_id(int id) {
    write_int(id);
  }

  void write_bytes(List<int> _bytes) {
    if (_bytes.length <= BUFFER_LIMIT - bytes.length) {
      this.bytes.addAll(_bytes);
    }
  }

  void write_int(int _int) {
    final int_bytes = ByteData(8);
    int_bytes.setInt64(0, _int, Endian.big);
    write_bytes(int_bytes.buffer.asUint8List().toList());
  }

  void write_string(String data) {
    final string_bytes = utf8.encode(data);
    write_int(string_bytes.length);
    write_bytes(string_bytes);
  }

  void write_bool(bool _bool) {
    if (_bool) {
      bytes.add(0x01);
    } else {
      bytes.add(0x00);
    }
  }

  void write_double(double _double) {
    final double_bytes = ByteData(8);
    double_bytes.setFloat64(0, _double, Endian.big);
    write_bytes(double_bytes.buffer.asUint8List().toList());
  }

  List<int> read_bytes(int length) {
    List<int> read_bytes = [];
    for (int i = readPos; i < length + readPos; i++) {
      read_bytes.add(bytes[i]);
    }
    readPos += length;
    return read_bytes;
  }

  int read_int() {
    int result = 0;
    List<int> read_bytes = this.read_bytes(8);
    for (int i = 0; i < read_bytes.length; i++) {
      result = (result << 8) | read_bytes[i];
    }
    return result;
  } 

  String read_string() {
    int string_length = read_int();
    List<int> read_bytes = this.read_bytes(string_length);    
    return String.fromCharCodes(read_bytes);
  }

  bool read_bool() {
    List<int> read_byte = this.read_bytes(1);  
    if (read_byte[0] == 1) {
      return true;
    } else {
      return false;
    }
  }

  double read_double() {
    List<int> read_bytes = this.read_bytes(8);  
    ByteData byteData = ByteData.sublistView(Uint8List.fromList(read_bytes));
    return byteData.getFloat64(0, Endian.big);
  }

  List<int> to_array() {
    return bytes;
  }

  int length() {
    return bytes.length;
  }
}
