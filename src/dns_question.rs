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
        let mut qname = String::new();
        let mut offset = 0;

        loop {
            let len = buf[offset] as usize;
            if len == 0 {
                break;
            }

            if offset > 0 {
                qname.push('.');
            }

            qname.push_str(std::str::from_utf8(&buf[offset + 1..offset + 1 + len]).unwrap());
            offset += len + 1;
        }

        let qtype = u16::from_be_bytes([buf[offset + 1], buf[offset + 2]]);
        let qclass = u16::from_be_bytes([buf[offset + 3], buf[offset + 4]]);

        Self {
            qname,
            qtype,
            qclass,
        }
    }
}

