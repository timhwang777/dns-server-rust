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
}