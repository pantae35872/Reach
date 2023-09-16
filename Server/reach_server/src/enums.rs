use tokio::net::TcpStream;

use crate::{packet::Packet, server_handle};

pub enum S2CCommand {
    SetStream(TcpStream),
    SetActive(bool),
    SetId(i64),
    SendData(Packet),
    GetActive()
}

pub enum C2SCommand {
    SendActive(bool)
}

pub enum C2SPacket{
    Welcome = 0,
}

pub enum S2CPacket {
    GetId
}

impl C2SPacket {
    pub async fn call(&self, id: &i64, packet: &mut Packet) {
        match self {
            C2SPacket::Welcome => server_handle::welcome(id, packet).await,
        }
    }
}