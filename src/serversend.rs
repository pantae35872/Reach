use crate::{packet::Packet, enums::{S2CCommand, S2CPacket}, CLIENTS_SENDER};

pub async fn send_data(to_client: &i64, packet: Packet) {
    let to_client = to_client.clone();
    tokio::spawn(async move {
        let clients_sender = CLIENTS_SENDER.lock().await;
        let client_sender = clients_sender.get(&to_client).unwrap();
        client_sender.send(S2CCommand::SendData(packet)).await.unwrap();
    });
}

pub async fn get_id(to_client: &i64) {
    let mut packet = Packet::new_with_id(S2CPacket::GetId as i64);
    packet.write_i64(&to_client);
    send_data(to_client, packet).await;
}