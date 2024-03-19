use std::net::UdpSocket;

mod dns_message;
mod dns_header;
mod dns_question;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let expected = dns_message::DNSMessage {
                    header: dns_header::DNSHeader {
                        id: 1234,
                        qr: 1,
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
                    },
                    question: dns_question::DNSQuestion {
                        qname: "codecrafters.io".to_string(),
                        qtype: 1,
                        qclass: 1,
                    },
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
