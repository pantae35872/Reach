use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{net::{TcpStream, TcpListener}, sync::{Mutex, mpsc::{Sender, Receiver, channel}}, time::sleep};

use crate::{client::Client, MAX_CLIENTS, enums::{S2CCommand, C2SCommand, C2SPacket}, serversend, CLIENTS_SENDER, SERVER_RECEIVER, IS_CLIENT_ACTIVE};

pub struct Server {
    pub clients: HashMap<i64, Arc<Mutex<Client>>>,
    pub clients_receiver: HashMap<i64, Arc<Mutex<Receiver<S2CCommand>>>>,
    pub server_sender: HashMap<i64, Arc<Mutex<Sender<C2SCommand>>>>,
    pub server_handle_map: Arc<Mutex<HashMap<i64, C2SPacket>>>
}
impl Server {
    pub fn new() -> Self {
        Server { 
            clients: HashMap::new(),
            clients_receiver: HashMap::new(),
            server_sender: HashMap::new(),
            server_handle_map: Arc::new(Mutex::new(HashMap::new()))
        }   
    }

    pub async fn initialize_server_handle_map(&mut self) {
        let mut server_handle_map = self.server_handle_map.lock().await;
        server_handle_map.insert(C2SPacket::ReceiveKey as i64, C2SPacket::ReceiveKey);
    }
    
    pub async fn initialize_server(&mut self) {
        self.initialize_server_handle_map().await;
        for i in 1..MAX_CLIENTS {
            let (client_sender, client_receiver ) = channel::<S2CCommand>(32);
            let (server_sender, server_receiver ) = channel::<C2SCommand>(32);
            let client = Arc::new(Mutex::new(Client::new(Arc::clone(&self.server_handle_map))));

            let client_receiver = Arc::new(Mutex::new(client_receiver));
            let server_sender = Arc::new(Mutex::new(server_sender));

            self.clients.insert(i, Arc::clone(&client));
            let mut clients_sender = CLIENTS_SENDER.lock().await;
            clients_sender.insert(i, Arc::from(client_sender));

            self.clients_receiver.insert(i, Arc::clone(&client_receiver));
            self.server_sender.insert(i, Arc::clone(&server_sender));

            let mut server_receivers = SERVER_RECEIVER.lock().await;
            server_receivers.insert(i, Arc::new(Mutex::new(server_receiver)));

            let mut is_client_active = IS_CLIENT_ACTIVE.lock().await;
            is_client_active.insert(i, false);

            let receiver_clone = Arc::clone(&client_receiver);
            let sender_clone = Arc::clone(&server_sender);
            let client_clone = Arc::clone(&client);
            tokio::spawn(async move {
                client_clone.lock().await.main_thread(&receiver_clone, &sender_clone).await;
            });
        }
    }

    pub async fn start_server(&mut self) {
        let addr: &str = "127.0.0.1:8080";
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Listening on: {}", addr);

        tokio::spawn(async {
            Server::loop_is_client_active().await;
        });

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok(stream) => {
                        println!("Received Connection from client address {}", stream.1);
                        Server::set_client(stream.0).await;
                    },
                    Err(_) => {

                    },
                }
            }
        });
    }

    pub async fn loop_is_client_active() {
        loop {
            sleep(Duration::from_secs(10)).await;
            for i in 1..MAX_CLIENTS{ 
                let mut is_client_active = IS_CLIENT_ACTIVE.lock().await;
                let client_sender = CLIENTS_SENDER.lock().await;
                let client_sender = client_sender.get(&i).unwrap();
                let server_receiver = SERVER_RECEIVER.lock().await;
                let mut server_receiver = server_receiver.get(&i).unwrap().lock().await;
                client_sender.send(S2CCommand::GetActive()).await.unwrap();
                match server_receiver.recv().await {
                    Some(C2SCommand::SendActive(active)) => {
                        is_client_active.insert(i, active);
                    },
                    None => {},
                }
            }
        }
    }

    async fn set_client(_stream: TcpStream) {
        let mut is_client_active = IS_CLIENT_ACTIVE.lock().await;
        let client_sender = CLIENTS_SENDER.lock().await;
        for i in 1..MAX_CLIENTS{
            if !*is_client_active.get(&i).unwrap() {
                let client_sender = client_sender.get(&i).unwrap();
                client_sender.send(S2CCommand::SetStream(_stream)).await.unwrap();
                client_sender.send(S2CCommand::SetId(i)).await.unwrap();
                client_sender.send(S2CCommand::SetActive(true)).await.unwrap();
                is_client_active.insert(i, true);
                serversend::get_id(&i).await;
                break;
            }
        }
    }
}