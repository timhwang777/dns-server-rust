#[repr(packed)]
pub struct DNSQuestion {
    pub qname: Vec<String>,
    pub qtype: u16,
    pub qclass: u16,
}

impl DNSQuestion {
    pub fn encode_question(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        for label in &self.qname {
            buf.push(label.len() as u8);
            buf.extend(label.as_bytes());
        }
        buf.push(0);

        buf.extend(&self.qtype.to_be_bytes());
        buf.extend(&self.qclass.to_be_bytes());

        buf
    }
}

