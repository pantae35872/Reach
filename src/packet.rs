
const BUFFER_LIMIT: usize = 1024;

pub struct Packet {
    pub buf: Vec<u8>,
    pub readpos: i64
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            buf: Vec::with_capacity(BUFFER_LIMIT),
            readpos: 0,
        }
    }

    pub fn new_with_data(data: &[u8; 1024]) -> Self {
        let mut buf = Vec::with_capacity(BUFFER_LIMIT);
        buf.extend_from_slice(data);
        Packet {
            buf,
            readpos: 0,
        }
    }

    pub fn new_with_id(id: i64) -> Self {
        let mut new_packet = Packet {
            buf : Vec::with_capacity(BUFFER_LIMIT),
            readpos: 0
        };

        new_packet.write_i64(&id);
        
        return new_packet;
    }

    // Write
    pub fn write_bytes(&mut self, data: &[u8]) -> &mut self::Packet {
        if data.len() <= BUFFER_LIMIT - self.buf.len() {
            self.buf.extend_from_slice(&data);
        } else {
            self.buf.extend_from_slice(&data[..BUFFER_LIMIT - self.buf.len()]);
        }
        return self;
    }
    
    pub fn write_i32(&mut self, data: &i32) -> &mut self::Packet {
        let i32_bytes: &[u8; 4] = &i32::to_be_bytes(*data);
        self.write_bytes(i32_bytes);
        return self;
    }

    pub fn write_string(&mut self, data: &String) -> &mut self::Packet {
        let data_bytes: &[u8] = data.as_bytes();
        self.write_i64(&(data_bytes.len() as i64));
        self.write_bytes(data_bytes);
        return self;
    }

    pub fn write_bool(&mut self, data: &bool) -> &mut self::Packet {
        if *data {
            self.write_bytes(&[1u8]);
        } else {
            self.write_bytes(&[0u8]);
        }
        return self;
    }

    pub fn write_i64(&mut self, data: &i64) -> &mut self::Packet {
        let i64_bytes: &[u8; 8] = &i64::to_be_bytes(*data);
        self.write_bytes(i64_bytes);
        return self;
    }

    pub fn write_f32(&mut self, data: &f32) -> &mut self::Packet {
        let f32_bytes: &[u8; 4] = &f32::to_be_bytes(*data);
        self.write_bytes(f32_bytes);
        return self;
    } 
    
    pub fn write_f64(&mut self, data: &f64) -> &mut self::Packet {
        let f64_bytes: &[u8; 8] = &f64::to_be_bytes(*data);
        self.write_bytes(f64_bytes);
        return self;
    }

    //Read

    pub fn read_bytes(&mut self, length: &i64) -> Vec<u8> {
        let read_bytes: Vec<u8> = self.buf[self.readpos as usize..(self.readpos + *length) as usize].to_vec();
        self.readpos += length;
        return read_bytes;
    }

    pub fn read_i32(&mut self) -> i32 {
        let i32_bytes: &[u8] = &self.read_bytes(&4);
        return i32::from_be_bytes(<[u8; 4]>::try_from(i32_bytes).expect("cannot convert [u8] to [u8, 4]"));
    }

    pub fn read_string(&mut self) -> String {
        let string_length: i64 = self.read_i64();
        match String::from_utf8(self.read_bytes(&string_length)) {
            Ok(string) => {
                return string;
            }
            Err(_e) => {
                return String::from("Error");
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
        return i64::from_be_bytes(<[u8; 8]>::try_from(i64_bytes).expect("cannot convert [u8] to [u8, 8]"));
    }

    pub fn read_f32(&mut self) -> f32 {
        let f32_bytes: &[u8] = &self.read_bytes(&4);
        return f32::from_be_bytes(<[u8; 4]>::try_from(f32_bytes).expect("cannot convert [u8] to [u8, 4]"));
    }

    pub fn read_f64(&mut self) -> f64 {
        let f64_bytes: &[u8] = &self.read_bytes(&8);
        return f64::from_be_bytes(<[u8; 8]>::try_from(f64_bytes).expect("cannot convert [u8] to [u8, 8]"));
    }

    //etc

    pub fn build(&self) -> [u8; 1024] {
        let data = &self.buf;
        let mut return_byte: [u8; 1024] = [0; 1024];
        if data.len() <= 1024 {
            return_byte[..data.len()].copy_from_slice(data);
        } else if data.len() == 1024 {
            return_byte.copy_from_slice(&data[..1024]);
        } else {
        }
        return return_byte;
    }

    pub fn length(&self) -> i64 {
        self.buf.len() as i64
    }
}