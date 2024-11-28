use core::str;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::modules::traits::Server;
use std::sync::Arc;

#[derive(Clone)]
pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn new(host: &str, port: u16) -> Self {
        TcpServer {
            host: host.to_string(),
            port,
        }
    }
    pub fn serve(&self, handler: Arc<dyn Server>) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address).expect("Failed to start TCP Server");

        println!("Listening at {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler = Arc::clone(&handler); // Pass the handler by reference
                    thread::spawn(move || handler.handle_connection(stream));
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }
    }
}

impl Server for TcpServer {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request_data = &buffer[..size];
                println!("Received data: {:?}", request_data);

                let response = self.handle_request(request_data);
                if let Err(e) = stream.write_all(&response) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
    }

    fn start(&self) {
        let handler = Arc::new(self.clone());
        self.serve(handler);
    }
}
