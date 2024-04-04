pub struct DNSAnswer {
    pub name: String,
    pub atype: u16,
    pub aclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl DNSAnswer {
    pub fn encode_answer(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        for label in self.name.split(".") {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0);

        buf.extend(&self.atype.to_be_bytes());
        buf.extend(&self.aclass.to_be_bytes());
        buf.extend(&self.ttl.to_be_bytes());
        buf.extend(&self.rdlength.to_be_bytes());
        buf.extend(&self.rdata);

        buf
    }

    pub fn decode_answer(buf: &[u8]) -> Self {
        let mut cursor = 0;

        let mut name = String::new();
        while buf[cursor] != 0 {
            let len = buf[cursor] as usize;
            cursor += 1;

            let label = String::from_utf8(buf[cursor..cursor+len].to_vec()).unwrap();
            name.push_str(&label);
            name.push('.');

            cursor += len;
        }
        name.pop(); // Remove the trailing dot
        cursor += 1; // Skip the null byte

        let atype = u16::from_be_bytes([buf[cursor], buf[cursor+1]]);
        cursor += 2;

        let aclass = u16::from_be_bytes([buf[cursor], buf[cursor+1]]);
        cursor += 2;

        let ttl = u32::from_be_bytes([buf[cursor], buf[cursor+1], buf[cursor+2], buf[cursor+3]]);
        cursor += 4;

        let rdlength = u16::from_be_bytes([buf[cursor], buf[cursor+1]]);
        cursor += 2;

        let rdata = buf[cursor..cursor+rdlength as usize].to_vec();

        Self {
            name,
            atype,
            aclass,
            ttl,
            rdlength,
            rdata,
        }
    }
}