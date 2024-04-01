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
    let mut label = String::new();

    loop {
        let len = buf[offset];

        if len == 0 {
            break;
        } else if len >> 6 == 0b11 {
            // compressed name; byte one is len
            let byte_two = buf[offset + 1];
            offset = ((((len & 0b0011_1111) as u16) << 8) | byte_two as u16) as usize;

            let name = decode_name(&buf[offset..]);

            label.push_str(&name);
            label.push('.');
        } else {
            let content = &buf[offset + 1..offset + 1 + len as usize];
            label.push_str(std::str::from_utf8(content).unwrap());
            label.push('.');

            offset += len as usize + 1;
        }
    }

    if !label.is_empty() {
        label.pop();
    }
    label
}

