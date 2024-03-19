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
    pub fn encode(&self) -> Vec<u8> {
        let mut response = Vec::new();

        let header_bytes = self.encode_header_to_message();
        response.extend_from_slice(&header_bytes);

        let question_bytes = self.encode_question_to_message();
        response.extend_from_slice(&question_bytes);

        response
    }
}