use std::{collections::HashMap, sync::Arc, time::Duration, fs};

use enums::{S2CCommand, C2SCommand};
use server::Server;
use tokio::{sync::{Mutex, mpsc::{Sender, Receiver}}, time::sleep};
use lazy_static::lazy_static;

pub const MAX_CLIENTS: i64 = (10)+1;
pub const BUFFER_SIZE: usize = 4096;

mod client;
mod server;
mod packet;
mod enums;
mod server_handle;
mod serversend;
mod encryption;

lazy_static! {
    static ref CLIENTS_SENDER: Mutex<HashMap<i64, Arc<Sender<S2CCommand>>>> = Mutex::new(HashMap::new());
    static ref SERVER_RECEIVER: Mutex<HashMap<i64, Arc<Mutex<Receiver<C2SCommand>>>>> = Mutex::new(HashMap::new());
    static ref IS_CLIENT_ACTIVE: Mutex<HashMap<i64, bool>> = Mutex::new(HashMap::new());
    static ref PRIVATE_KEY: Mutex<String> = Mutex::new(fs::read_to_string("D:\\Reach\\key\\decrypted_private_key.pem").unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new();
    server.initialize_server().await;
    server.start_server().await;
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
