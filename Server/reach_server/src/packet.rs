use crate::BUFFER_SIZE;

pub struct Packet {
    pub buf: Vec<u8>,
    pub readpos: i64
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            buf: Vec::with_capacity(BUFFER_SIZE),
            readpos: 0,
        }
    }

    pub fn new_with_data(data: &[u8]) -> Self {
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        buf.extend_from_slice(data);
        Packet {
            buf,
            readpos: 0,
        }
    }

    pub fn new_with_id(id: i64) -> Self {
        let mut new_packet = Packet {
            buf : Vec::with_capacity(BUFFER_SIZE),
            readpos: 0
        };

        new_packet.write_i64(&id);
        
        new_packet
    }

    // Write
    pub fn write_bytes(&mut self, data: &[u8]) -> &mut Self {
        if data.len() <= BUFFER_SIZE - self.buf.len() {
            self.buf.extend_from_slice(&data);
        } else {
            self.buf.extend_from_slice(&data[..BUFFER_SIZE - self.buf.len()]);
        }
        self
    }

    pub fn insert_bytes(&mut self, data: &[u8]) -> &mut Self {
        if data.len() <= BUFFER_SIZE - self.buf.len() {
            self.buf.splice(0..0, data.iter().cloned());
        } else {
            self.buf.splice(0..0, data[..BUFFER_SIZE - self.buf.len()].iter().cloned());
        }
        self
    }
    
    pub fn write_i32(&mut self, data: &i32) -> &mut Self {
        let i32_bytes: &[u8; 4] = &i32::to_be_bytes(*data);
        self.write_bytes(i32_bytes);
        self
    }

    pub fn write_string(&mut self, data: &String) -> &mut Self {
        let data_bytes: &[u8] = data.as_bytes();
        self.write_i64(&(data_bytes.len() as i64));
        self.write_bytes(data_bytes);
        self
    }

    pub fn write_bool(&mut self, data: &bool) -> &mut Self {
        if *data {
            self.write_bytes(&[1u8]);
        } else {
            self.write_bytes(&[0u8]);
        }
        self
    }
    pub fn insert_bool(&mut self, data: &bool) -> &mut Self {
        if *data {
            self.insert_bytes(&[1u8]);
        } else {
            self.insert_bytes(&[0u8]);
        }
        self
    }

    pub fn write_i64(&mut self, data: &i64) -> &mut Self {
        let i64_bytes: &[u8; 8] = &i64::to_be_bytes(*data);
        self.write_bytes(i64_bytes);
        self
    }

    pub fn write_f32(&mut self, data: &f32) -> &mut Self {
        let f32_bytes: &[u8; 4] = &f32::to_be_bytes(*data);
        self.write_bytes(f32_bytes);
        self
    } 
    
    pub fn write_f64(&mut self, data: &f64) -> &mut Self {
        let f64_bytes: &[u8; 8] = &f64::to_be_bytes(*data);
        self.write_bytes(f64_bytes);
        self
    }

    //Read

    pub fn read_bytes(&mut self, length: &i64) -> Vec<u8> {
        let read_bytes: Vec<u8> = self.buf[self.readpos as usize..(self.readpos + *length) as usize].to_vec();
        self.readpos += length;
        read_bytes
    }

    pub fn read_i32(&mut self) -> i32 {
        let i32_bytes: &[u8] = &self.read_bytes(&4);
        i32::from_be_bytes(<[u8; 4]>::try_from(i32_bytes).expect("cannot convert [u8] to [u8, 4]"))
    }

    pub fn read_string(&mut self) -> String {
        let string_length: i64 = self.read_i64();
        match String::from_utf8(self.read_bytes(&string_length)) {
            Ok(string) => {
                string
            }
            Err(_e) => {
                String::from("Error")
            }
        }
    }

    pub fn read_bool(&mut self) -> bool {
        if let Some(result) = match &self.read_bytes(&1)[0]  {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        } {
            result
        } else {
            false
        }
    }

    pub fn read_i64(&mut self) -> i64{
        let i64_bytes: &[u8] = &self.read_bytes(&8);
        i64::from_be_bytes(<[u8; 8]>::try_from(i64_bytes).expect("cannot convert [u8] to [u8, 8]"))
    }

    pub fn read_f32(&mut self) -> f32 {
        let f32_bytes: &[u8] = &self.read_bytes(&4);
        f32::from_be_bytes(<[u8; 4]>::try_from(f32_bytes).expect("cannot convert [u8] to [u8, 4]"))
    }

    pub fn read_f64(&mut self) -> f64 {
        let f64_bytes: &[u8] = &self.read_bytes(&8);
        f64::from_be_bytes(<[u8; 8]>::try_from(f64_bytes).expect("cannot convert [u8] to [u8, 8]"))
    }

    //etc

    pub fn build(&self) -> &[u8] {
        &self.buf.as_slice()
    }

    pub fn length(&self) -> (i64, &self::Packet) {
        (self.buf.len() as i64, self)
    }
}