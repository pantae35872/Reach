use std::sync::Arc;

use crate::{packet::Packet, encryption::Encryption, CLIENTS_SENDER, enums::S2CCommand};

pub async fn receive_key(id: &i64, packet: &mut Packet) {
    let mut encryption = Encryption::new().await;
    let e_key_length = packet.read_i64();
    let e_key_bytes = packet.read_bytes(&e_key_length);
    let e_iv_length = packet.read_i64();
    let e_iv_bytes = packet.read_bytes(&e_iv_length);
    let key = encryption.set_data(&Arc::from(e_key_bytes.as_slice())).rsa_decrypt().build();
    let mut encryption = Encryption::new().await;
    let iv = encryption.set_data(&Arc::from(e_iv_bytes.as_slice())).rsa_decrypt().build();
    let clients_sender = CLIENTS_SENDER.lock().await;
    let client_sender = clients_sender.get(&id).unwrap();
    let mut f_key: [u8; 16] = [0u8; 16]; 
    let mut f_iv: [u8; 16] = [0u8; 16];
    f_key.copy_from_slice(key);
    f_iv.copy_from_slice(iv);
    client_sender.send(S2CCommand::SetKey(f_key)).await.unwrap();
    client_sender.send(S2CCommand::SetIv(f_iv)).await.unwrap();
}