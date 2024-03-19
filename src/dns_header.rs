pub struct DNSHeader {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DNSHeader {
    pub fn encode_header(&self) -> [u8; 12] {
        let mut buf = [0; 12];

        buf[0..2].copy_from_slice(&self.id.to_be_bytes());
        buf[2] = ((self.qr as u8) << 7) 
            | ((self.opcode & 0x0F) << 3) 
            | ((self.aa as u8) << 2) 
            | ((self.tc as u8) << 1) 
            | (self.rd as u8);
        buf[3] = ((self.ra as u8) << 7)
            | (self.z << 4)
            | (self.rcode & 0x0F);
        buf[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buf[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buf[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buf[10..12].copy_from_slice(&self.arcount.to_be_bytes());

        buf
    }
}