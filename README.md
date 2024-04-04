# Simple Rust DNS Server

![Static Badge](https://img.shields.io/badge/Rust-Solutions-blue?logo=rust
)

## Table of Contents

1. [About the Project](#about-the-project)
2. [Getting Started](#getting-started)
   - [Prerequisites](#prerequisites)
3. [Author](#author)

## About the Project

This project is a simple DNS forwarder written in Rust. It's designed to handle DNS queries by forwarding them to a specified resolver and then returning the resolver's response to the original requester. The forwarder can handle multiple queries in a single request and supports the basic DNS record types. It's an educational project to demonstrate the basics of UDP networking and DNS protocol handling in Rust.

The forwarder listens on `127.0.0.1:2053` and supports forwarding DNS queries to a specified resolver. It decodes the incoming DNS message, extracts the questions, and then forwards each question to the resolver. Once the resolver responds, the forwarder decodes the response, extracts the answers, and then sends these answers back to the original requester. Additionally, it allows for the specification of the resolver address via command line arguments.

## Getting Started

To get a local copy up and running, follow these simple steps:

### Prerequisites

To run this web server, you will need:

- Rust programming environment setup on your machine. You can follow the official guide to install Rust: [Rust Installation](https://www.rust-lang.org/tools/install).
- Basic understanding of Rust and TCP/IP networking.

Once Rust is installed, you can clone this repository or copy the source code into your own Rust project. Make sure to include all the provided code in your `main.rs` and any module files as required.

To run the server, navigate to the project directory in your terminal and execute:
```rust
cargo run main.rs
```

Alternatively, you can execute the `your_server.sh` shell script.
## Author

Timothy Hwang