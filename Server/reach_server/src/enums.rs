use tokio::net::TcpStream;

use crate::{packet::Packet, server_handle};

pub enum S2CCommand {
    SetStream(TcpStream),
    SetActive(bool),
    SetId(i64),
    SendData(Packet, bool),
    GetActive(),
    SetKey([u8; 16]),
    SetIv([u8; 16])
}

pub enum C2SCommand {
    SendActive(bool)
}

pub enum C2SPacket{
    ReceiveKey = 0,
}

pub enum S2CPacket {
    GetId = 0,
}

impl C2SPacket {
    pub async fn call(&self, id: &i64, packet: &mut Packet) {
        match self {
            C2SPacket::ReceiveKey => server_handle::receive_key(id, packet).await,
        }
    }
}