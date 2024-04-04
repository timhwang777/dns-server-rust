use std::{
    net::{UdpSocket, ToSocketAddrs},
};

mod dns_message;
mod dns_header;
mod dns_question;
mod dns_answer;

fn forward_question(question: dns_question::DNSQuestion, resolver_addr: impl ToSocketAddrs) -> dns_answer::DNSAnswer {
    let socket = UdpSocket::bind("localhost:0").expect("Failed to bind to address");
    let header = dns_header::DNSHeader {
        id: 123,
        qr: 0,
        opcode: 0,
        aa: false,
        tc: false,
        rd: true,
        ra: false,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    let forward_message = dns_message::DNSMessage {
        header: header,
        question: vec![question],
        answer: Vec::new(),
    };

    socket
        .send_to(&forward_message.encode(), resolver_addr)
        .expect("Failed to send request");

    let mut buf = [0; 512];

    socket
        .recv_from(&mut buf)
        .expect("Failed to receive response");

    let buf = &buf[..];
    let (_question, question_size) = dns_question::DNSQuestion::decode_question(&buf[12..]);
    let buf = &buf[12 + question_size..];

    // Parse the answer from the forwarder
    let forward_answer = dns_answer::DNSAnswer::decode_answer(buf);

    forward_answer
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let args = std::env::args().collect::<Vec<String>>();
    let mut resolver = None;
    if args[1] == "--resolver" {
        resolver = Some(args[2].clone());
    }

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
                
                let mut questions = Vec::new();
                for _ in 0..header.qdcount {
                    let (question, _question_size) = dns_question::DNSQuestion::decode_question(&buf[12..]);
                    questions.push(question);
                }
                
                let mut answers = Vec::new();
                for question in &questions {
                    let answer = forward_question(question.clone(), resolver.as_ref().unwrap());
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
