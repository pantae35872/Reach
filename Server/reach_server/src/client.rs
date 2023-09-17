use std::{sync::Arc, time::Duration, collections::HashMap};

use tokio::{sync::{Mutex, mpsc::{Receiver, Sender}}, time::sleep, io::AsyncWriteExt, net::TcpStream};

use crate::{enums::{S2CCommand, C2SCommand, C2SPacket}, packet::Packet, BUFFER_SIZE};

pub struct Client {
    pub id: i64,
    pub active: bool,
    pub stream: Arc<Mutex<Option<tokio::net::TcpStream>>>,
    pub server_handle_map: Arc<Mutex<HashMap<i64, C2SPacket>>>
}

impl Client {
    pub fn new(_server_handle_map: Arc<Mutex<HashMap<i64, C2SPacket>>>) -> Self {
        Client { 
            id: 0,
            active: false,
            stream: Arc::new(Mutex::new(None)),
            server_handle_map: _server_handle_map
        }
    }

    pub async fn main_thread(&mut self, receiver: &Arc<Mutex<Receiver<S2CCommand>>>, sender: &Arc<Mutex<Sender<C2SCommand>>>) -> ! {
        let mut buffer: Vec<u8> = vec![8; BUFFER_SIZE];
        let mut disconnect: bool = false;
        loop {
            sleep(Duration::from_millis(1)).await;
            let stream_opt = self.stream.lock().await;

            match stream_opt.as_ref() {
                Some(stream) => {
                    while let Ok(_length) = stream.try_read(&mut buffer) {
                        if _length == 0 {
                            disconnect = true;
                            break;
                        }
                        let receive_data: &[u8] = &buffer[0.._length];
                        let mut fixed_received_data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                        if receive_data.len() <= BUFFER_SIZE {
                            fixed_received_data[..receive_data.len()].copy_from_slice(receive_data);
                        } else if receive_data.len() == BUFFER_SIZE {
                            fixed_received_data.copy_from_slice(&receive_data[..BUFFER_SIZE]);
                        } else {
                            disconnect = true;
                            break;
                        }
                        
                        let receive_data_clone: [u8; BUFFER_SIZE] = fixed_received_data.clone();
                        let server_handle_map = Arc::clone(&self.server_handle_map);
                        let id = self.id.clone();
                        tokio::spawn(async move {
                            let mut packet = Packet::new_with_data(&receive_data_clone);
                            let packet_id = &packet.read_i64();
                            if let Some(func) = server_handle_map.lock().await.get(&packet_id) {
                                func.call(&id, &mut packet).await;
                            }
                        });
                    }
                },
                None => {
                },
            }
            
            drop(stream_opt);

            if self.active && disconnect {
                self.disconnect().await;
            }

            while let Ok(string_receive) = receiver.lock().await.try_recv() {
                match string_receive {
                    S2CCommand::SetStream(stream) => {
                        disconnect = false;
                        self.stream = Arc::new(Mutex::new(Some(stream)));
                    },
                    S2CCommand::SetActive(active) => {
                        self.active = active;
                    },
                    S2CCommand::GetActive() => {
                        sender.lock().await.send(C2SCommand::SendActive(self.active)).await.unwrap();
                    },
                    S2CCommand::SetId(_id) => {
                        self.id = _id;
                    },
                    S2CCommand::SendData(packet) => {
                        let mut stream_opt = self.stream.lock().await;
                        let stream: Option<&mut TcpStream> = stream_opt.as_mut();
                        match stream {
                            Some(stream) => {
                                stream.write(&packet.build()).await.unwrap();
                            }
                            None => {},
                        }
                    }
                }
            }
        }
    }

    pub async fn disconnect(&mut self) {
        println!("disconnecting from client");
        self.stream = Arc::new(Mutex::new(None));
        self.id = 0;
        self.active = false;
    }
}