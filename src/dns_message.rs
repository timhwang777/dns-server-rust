use crate::dns_header::DNSHeader;
use crate::dns_question::DNSQuestion;

#[repr(packed)]
pub struct DNSMessage {
    pub header: DNSHeader,
    pub question: DNSQuestion,
}

impl DNSMessage {
    pub fn encode_header_to_message(&self) -> [u8; 12] {
        self.header.encode_header()
    }
    pub fn encode_question_to_message(&self) -> Vec<u8> {
        self.question.encode_question()
    }
}