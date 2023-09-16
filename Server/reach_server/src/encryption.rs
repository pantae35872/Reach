use crate::{BUFFER_SIZE, server};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes256>;

pub struct Encryption {
    buffer: [u8; BUFFER_SIZE],
    buffer_len: usize,
    pub aes: Aes
}

pub struct Aes {
    key: [u8; 32],
    iv: [u8; 16]
}

struct Rsa {
    
}

impl Encryption {
    pub fn new() -> Self {
        Self { 
            aes: Aes::new(),
            buffer_len: 0,
            buffer: [0u8; 1024]
        }
    }

    pub fn set_data(&mut self, data: &[u8]) -> &mut Self {
        self.buffer[..data.len()].copy_from_slice(data);
        self.buffer_len = data.len();
        self
    } 

    pub fn set_key(&mut self, key: &[u8; 32]) -> &mut Self {
        self.aes.set_key(key);
        self
    }

    pub fn set_iv(&mut self, iv: &[u8; 16]) -> &mut Self {
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

    pub fn build(&self) -> &[u8] {
        &self.buffer[0..BUFFER_SIZE]
    }
}

impl Aes {
    fn new() -> Self {
        Self { 
            key: [0u8; 32],
            iv: [0u8; 16],
        }
    }

    fn set_key(&mut self, key: &[u8; 32]) -> &mut Self{
        self.key = key.clone();
        drop(key);
        self
    }

    fn set_iv(&mut self, iv: &[u8; 16]) -> &mut Self {
        self.iv = iv.clone();
        drop(iv);
        self
    }

    fn get_key(&self) -> &[u8; 32] {
        &self.key
    }

    fn get_iv(&self) -> &[u8; 16] {
        &self.iv
    }
}