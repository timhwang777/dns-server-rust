use crate::dns_header::DNSHeader;
use crate::dns_question::DNSQuestion;
use crate::dns_answer::DNSAnswer;


pub struct DNSMessage {
    pub header: DNSHeader,
    pub question: Vec<DNSQuestion>,
    pub answer: Vec<DNSAnswer>,
}

impl DNSMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut response = Vec::new();

        let header_bytes = self.header.encode_header();
        response.extend_from_slice(&header_bytes);

        for question in &self.question {
            let question_bytes = question.encode_question();
            response.extend_from_slice(&question_bytes);
        }

        for answer in &self.answer {
            let answer_bytes = answer.encode_answer();
            response.extend_from_slice(&answer_bytes);
        }

        response
    }
}