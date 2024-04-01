pub struct DNSQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl DNSQuestion {
    pub fn encode_question(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        for label in self.qname.split(".") {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0);

        buf.extend(&self.qtype.to_be_bytes());
        buf.extend(&self.qclass.to_be_bytes());

        buf
    }

    pub fn decode_question(buf: &[u8]) -> Self {
        println!("Decoding question");
        let qname = decode_name(buf);
        println!("Decoded name: {}", qname);

        let offset = qname.len() + 12;

        let qtype = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
        let qclass = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);

        Self {
            qname,
            qtype,
            qclass,
        }
    }
}

fn decode_name(buf: &[u8]) -> String {
    let mut offset = 0;
    let mut name = String::new();

    while offset < buf.len() {
        let len = buf[offset];
        if len == 0 {
            offset += 1;
            if offset < buf.len() {
                name.push('.');
            }
            continue;
        }

        if (len & 0xC0) == 0xC0 {
            let byte1 = (len as u16) & 0x3F; // Last 6 bits of first byte
            let byte2 = buf[offset + 1] as u16; // Entire second byte
            offset = ((byte1 << 8) | byte2) as usize; // Calculate offset from start of message
            name.push_str(&decode_name(&buf[offset..]));
        } else {
            if !name.is_empty() {
                name.push('.');
            }

            name.push_str(std::str::from_utf8(&buf[offset + 1..offset + 1 + len as usize]).unwrap());
            offset += len as usize + 1;
        }
    }

    name
}


