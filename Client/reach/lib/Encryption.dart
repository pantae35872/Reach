import 'dart:io';
import 'dart:typed_data';

import 'package:encrypt/encrypt.dart' as Encrypt;
import 'package:pointycastle/export.dart';
import 'package:basic_utils/basic_utils.dart';
import 'package:flutter/services.dart';

class Encryption {
  List<int> buffer = [];
  late Aes aes;
  Encryption.new() {
    aes = Aes();
  }

  Encryption set_iv(List<int> _iv) {
    aes.set_iv(_iv);
    return this;
  }

  Encryption set_key(List<int> _key) {
    aes.set_key(_key);
    return this;
  }

  Encryption set_data(List<int> _data) {
    buffer = _data;
    return this;
  }

  Encryption aes_encrypt() {
    final encryptService = Encrypt.Encrypter(Encrypt.AES(Encrypt.Key(Uint8List.fromList(aes.key)), mode: Encrypt.AESMode.cbc));
    buffer = encryptService.encryptBytes(buffer, iv: Encrypt.IV(Uint8List.fromList(aes.iv))).bytes;
    return this;
  }

  Encryption aes_decrypt() {
    final encryptService = Encrypt.Encrypter(Encrypt.AES(Encrypt.Key(Uint8List.fromList(aes.key)), mode: Encrypt.AESMode.cbc));
    buffer = encryptService.decryptBytes(Encrypt.Encrypted(Uint8List.fromList(buffer)), iv: Encrypt.IV(Uint8List.fromList(aes.iv)));
    return this;
  }

  Future<Encryption> rsa_encrypt() async {
    await File('D:\\Reach\\key\\public_key.pem').readAsString().then((String contents) async {
      var public = CryptoUtils.rsaPublicKeyFromPem(contents);
      var cipher = PKCS1Encoding(RSAEngine());
      cipher.init(true, PublicKeyParameter<RSAPublicKey>(public));
      final cipherText = cipher.process(Uint8List.fromList(buffer));
      buffer = cipherText.toList();
    });
    return this;
  }

  List<int> build() {
    return buffer;
  }
}

class Aes {
  late List<int> key;
  late List<int> iv;

  Aes set_key(List<int> _key) {
    key = _key;
    return this;
  }

  Aes set_iv(List<int> _iv) {
    iv = _iv;
    return this;
  } 
}