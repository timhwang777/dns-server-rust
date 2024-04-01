use bytes::Buf;

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

        let qtype = if buf.len() >= offset + 2 {
            u16::from_be_bytes([buf[offset], buf[offset + 1]])
        } else {
            1
        };
    
        let qclass = if buf.len() >= offset + 4 {
            u16::from_be_bytes([buf[offset + 2], buf[offset + 3]])
        } else {
            1
        };

        Self {
            qname,
            qtype,
            qclass,
        }
    }
}

fn decode_name(buf: &[u8]) -> String {
    let mut bytes = bytes::Bytes::copy_from_slice(buf);
    let orig = bytes.clone();

    let mut label = String::new();

    loop {
        let len = bytes.get_u8();

        if len == 0 {
            break;
        } else if len >> 6 == 0b11 {
            // compressed name; byte one is len
            let byte_two = bytes.get_u8();
            let offset: usize = ((((len & 0b0011_1111) as u16) << 8) | byte_two as u16) as usize;

            let name = decode_name(&orig.slice(offset..));

            label.push_str(name.as_str());
            label.push('.');
        } else {
            let content = bytes.copy_to_bytes(len as usize);
            label.push_str(std::str::from_utf8(&content[..]).unwrap());
            label.push('.');
        }
    }

    label.pop();
    label
}
