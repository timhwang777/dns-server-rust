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

    pub fn decode_question(buf: &[u8], start: usize) -> (Self, usize) {
        let mut qname = String::new();
        let mut offset = start;
        let mut jumped = false;
        let mut jump_offset = 0;
    
        loop {
            let len = buf[offset];
            if len == 0 {
                break;
            }
    
            if (len & 0xC0) == 0xC0 {
                // This is a pointer to a previous name
                if !jumped {
                    jump_offset = offset + 2;
                }
                let byte1 = (len as u16) & 0x3F; // Last 6 bits of first byte
                let byte2 = buf[offset + 1] as u16; // Entire second byte
                offset = ((byte1 << 8) | byte2) as usize; // Calculate offset from start of message
                jumped = true;
            } else {
                if offset > 0 {
                    qname.push('.');
                }
    
                qname.push_str(std::str::from_utf8(&buf[offset + 1..offset + 1 + len as usize]).unwrap());
                offset += len as usize + 1;
            }
        }
    
        if !jumped {
            jump_offset = offset + 1;
        }
    
        let qtype = u16::from_be_bytes([buf[jump_offset], buf[jump_offset + 1]]);
        let qclass = u16::from_be_bytes([buf[jump_offset + 2], buf[jump_offset + 3]]);
    
        (Self {
            qname,
            qtype,
            qclass,
        }, jump_offset + 4)
    }
}

