use core::str;
use log::error;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::modules::traits::Server;

pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn new(host: &str, port: u16) -> TcpServer {
        let host = host.to_string();
        let port = port;
        TcpServer { host, port }
    }

    pub fn serve(&self, handler: impl Server + Send + Sync + 'static) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address).expect("Failed to start TCP Server");

        println!("Listening at {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || handler.handle_connection(stream));
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(size) => {
                let request_data = &buffer[..size];
                println!("Received data: {:?}", request_data);

                let response = self.handle_request(request_data);
                stream
                    .write_all(&response)
                    .expect("Failed to send response");
            }
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
    }
}

impl Server for TcpServer {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    fn start(&self) {
        self.serve(self.clone());
    }
}
impl Clone for TcpServer {
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

// pub fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
//     let mut buffer = [0u8; 1024];

//     loop {
//         let nbytes = stream.read(&mut buffer)?;
//         if nbytes == 0 {
//             return Ok(());
//         }
//         print!("{}", str::from_utf8(&buffer[..nbytes])?);
//         stream.write_all(&buffer[..nbytes])?;
//     }
// }
