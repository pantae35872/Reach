use std::{sync::Arc, time::Duration, collections::HashMap};

use tokio::{sync::{Mutex, mpsc::{Receiver, Sender}}, time::sleep, io::AsyncWriteExt, net::TcpStream};

use crate::{enums::{S2CCommand, C2SCommand, C2SPacket}, packet::Packet, BUFFER_SIZE, encryption::Encryption};

pub struct Client {
    pub id: i64,
    pub active: bool,
    pub stream: Arc<Mutex<Option<tokio::net::TcpStream>>>,
    pub server_handle_map: Arc<Mutex<HashMap<i64, C2SPacket>>>,
    pub key: Arc<Mutex<Option<[u8; 16]>>>,
    pub iv: Arc<Mutex<Option<[u8; 16]>>>,
}

impl Client {
    pub fn new(_server_handle_map: Arc<Mutex<HashMap<i64, C2SPacket>>>) -> Self {
        Client { 
            id: 0,
            active: false,
            stream: Arc::new(Mutex::new(None)),
            server_handle_map: _server_handle_map,
            key: Arc::new(Mutex::new(None)),
            iv: Arc::new(Mutex::new(None)),
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
                        let receive_data_len = receive_data.len();
                        let server_handle_map = Arc::clone(&self.server_handle_map);
                        let id = self.id.clone();
                        let key = Arc::clone(&self.key);
                        let iv = Arc::clone(&self.iv);
                        tokio::spawn(async move {
                            let key = key.lock().await;
                            let iv = iv.lock().await;
                            let mut packet = Packet::new_with_data(&receive_data_clone[..receive_data_len]);
                            let encrypt = &packet.read_bool();
                            let data_vec = packet.read_bytes(&(packet.length().0 - 1));
                            let data = &data_vec.as_slice()[..data_vec.len()];
                            if *encrypt {
                                if key.is_some() && iv.is_some() {
                                    let mut encryption = Encryption::new().await;
                                    let data = encryption.set_key(&Arc::from(key.unwrap())).set_iv(&Arc::from(iv.unwrap())).set_data(&Arc::from(data)).aes_decrypt().build();
                                    packet = Packet::new_with_data(data);
                                }
                            } else {
                                packet = Packet::new_with_data(data);
                            }
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
                    S2CCommand::SendData(_packet, encrypt) => {
                        let mut stream_opt = self.stream.lock().await;
                        let key = self.key.lock().await;
                        let iv = self.iv.lock().await;
                        let stream: Option<&mut TcpStream> = stream_opt.as_mut();
                        let mut packet = Packet::new();
                        packet.insert_bool(&encrypt);
                        let data = _packet.build();
                        if encrypt {
                            if key.is_some() && iv.is_some() {
                                let mut encryption = Encryption::new().await;
                                let key = key.unwrap();
                                let iv = iv.unwrap();
                                let data = encryption.set_data(&Arc::from(data)).set_iv(&Arc::from(iv)).set_key(&Arc::from(key)).aes_encrypt().build();
                                packet.write_bytes(data);
                            }
                        } else {
                            packet.write_bytes(&data);
                        }
                        match stream {
                            Some(stream) => {
                                stream.write(&packet.build()).await.unwrap();
                            }
                            None => {},
                        }
                    },
                    S2CCommand::SetKey(_key) => {
                        let key = Arc::clone(&self.key);
                        let mut key = key.lock().await;
                        *key = Some(_key);
                    },
                    S2CCommand::SetIv(_iv) => {
                        let iv = Arc::clone(&self.iv);
                        let mut iv = iv.lock().await;
                        *iv = Some(_iv);
                    }
                }
            }
        }
    }

    pub async fn disconnect(&mut self) {
        println!("disconnecting from client id: {}", self.id);
        self.id = 0;
        self.active = false;
        let stream = Arc::clone(&self.stream);
        let key = Arc::clone(&self.key);
        let iv = Arc::clone(&self.iv);
        let mut iv = iv.lock().await;
        let mut key = key.lock().await;
        let mut stream = stream.lock().await;
        *key = None;
        *iv = None;
        *stream = None;
    }
}