use crate::dns_header::DNSHeader;

#[repr(packed)]
pub struct DNSMessage {
    pub header: DNSHeader,
}

impl DNSMessage {
    pub fn encode_header_to_message(&self) -> [u8; 12] {
        self.header.encode_header()
    }
}