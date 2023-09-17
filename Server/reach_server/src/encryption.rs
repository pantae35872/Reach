use std::sync::Arc;

use crate::{BUFFER_SIZE, PRIVATE_KEY};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, pkcs8::DecodePrivateKey};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub struct Encryption {
    buffer: [u8; BUFFER_SIZE],
    buffer_len: usize,
    pub aes: Aes,
    pub rsa: Rsa
}

pub struct Aes {
    key: [u8; 16],
    iv: [u8; 16]
}

pub struct Rsa {
    priv_key: RsaPrivateKey
}

impl Encryption {
    pub async fn new() -> Self {
        Self { 
            aes: Aes::new(),
            rsa: Rsa::new(PRIVATE_KEY.lock().await.to_string()),
            buffer_len: 0,
            buffer: [0u8; 1024]
        }
    }

    pub fn set_data(&mut self, data: &Arc<&[u8]>) -> &mut Self {
        let len = data.len().min(self.buffer.len());
        self.buffer[..len].copy_from_slice(&data[..len]);
        self.buffer_len = len;
        self
    } 

    pub fn set_key(&mut self, key: &Arc<[u8; 16]>) -> &mut Self {
        self.aes.set_key(key);
        self
    }

    pub fn set_iv(&mut self, iv: &Arc<[u8; 16]>) -> &mut Self {
        self.aes.set_iv(iv);
        self
    }

    pub fn aes_encrypt(&mut self) -> &mut Self {
        let end = Aes128CbcEnc::new(self.aes.get_key().into(), self.aes.get_iv().into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&self.buffer.clone()[..self.buffer_len], &mut self.buffer)
        .unwrap();
        self.buffer_len = end.len();
        self
    }

    pub fn aes_decrypt(&mut self) -> &mut Self {
        let ded = Aes128CbcDec::new(self.aes.get_key().into(), self.aes.get_iv().into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&self.buffer.clone()[..self.buffer_len], &mut self.buffer)
        .unwrap();
        self.buffer_len = ded.len();
        self
    }

    pub fn rsa_decrypt(&mut self) -> &mut Self {
        let priv_key = &self.rsa.priv_key;
        let e_buffer = &self.buffer.clone()[..self.buffer_len]; 
        self.buffer.copy_from_slice(priv_key.decrypt(Pkcs1v15Encrypt, e_buffer).expect("failed to decrypt").as_slice());
        self
    }

    pub fn build(&self) -> &[u8] {
        &self.buffer[0..self.buffer_len]
    }
}

impl Aes {
    fn new() -> Self {
        Self { 
            key: [0u8; 16],
            iv: [0u8; 16],
        }
    }

    fn set_key(&mut self, key: &Arc<[u8; 16]>) -> &mut Self{
        self.key = *Arc::clone(key);
        self
    }

    fn set_iv(&mut self, iv: &Arc<[u8; 16]>) -> &mut Self {
        self.iv = *Arc::clone(iv);
        self
    }

    fn get_key(&self) -> &[u8; 16] {
        &self.key
    }

    fn get_iv(&self) -> &[u8; 16] {
        &self.iv
    }
}

impl Rsa {
    fn new(key: String) -> Self {
        Self {  
            priv_key: RsaPrivateKey::from_pkcs8_pem(&key).unwrap()
        }
    }
}