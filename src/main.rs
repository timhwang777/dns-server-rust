use std::net::UdpSocket;

mod dns_message;
mod dns_header;
mod dns_question;
mod dns_answer;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let buf = &buf[..size];
                let mut header = dns_header::DNSHeader::decode_header(&buf[0..12]);
                header.qr = 1;
                if header.opcode != 0 {
                    header.rcode = 4;
                }
                
                let mut offset = 12 as usize;
                let mut questions = Vec::new();
                for _ in 0..header.qdcount {
                    let (question, end_of_question) = dns_question::DNSQuestion::decode_question(&buf[offset..size], offset);
                    questions.push(question);
                    offset = end_of_question;
                }
                
                let mut answers = Vec::new();
                for question in &questions {
                    let answer = dns_answer::DNSAnswer {
                        name: question.qname.clone(),
                        atype: 1,
                        aclass: 1,
                        ttl: 60,
                        rdlength: 4,
                        rdata: vec![8, 8, 8, 8],
                    };
                    answers.push(answer);
                }
                
                header.ancount = answers.len() as u16;
                
                let expected = dns_message::DNSMessage {
                    header: header,
                    question: questions,
                    answer: answers,
                };
                let response = expected.encode();
                
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
