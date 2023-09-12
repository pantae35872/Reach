use crate::{packet::Packet, serversend};


pub async fn welcome(id: &i64, packet: &mut Packet) {
    let st = packet.read_string();
    println!("data from id: {} data: {}", id, st);
    serversend::get_id(id).await;
}
