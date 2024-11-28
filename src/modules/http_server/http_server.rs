use crate::tcp_server::TcpServer;
use crate::traits::Server;
use std::sync::Arc;
use std::{
    io::{Read, Write},
    net::TcpStream,
};
#[derive(Clone)]
pub struct HttpServer {
    server: TcpServer,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            server: TcpServer::new(host, port),
        }
    }
}

impl Server for HttpServer {
    fn handle_request(&self, _data: &[u8]) -> Vec<u8> {
        let response_line = "HTTP/1.1 200 OK";

        let headers = vec![
            "Server: Crude Server".to_string(),
            "Content-Type: text/html".to_string(),
        ];
        let header_string = headers.join("\r\n");
        let blank_line = "\r\n";
        let response_body = "
            <html>
            <body>
            <h1>request received!</h1>
            </body>
            </html>
            ";

        let response = format!(
            "{}\r\n{}\r\n{}{}",
            response_line, header_string, blank_line, response_body
        );
        response.as_bytes().to_vec()
    }

    fn start(&self) {
        let handler = Arc::new(self.clone());
        self.server.serve(handler);
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(size) => {
                let request_data = &buffer[..size];
                let response_data = self.handle_request(request_data);
                if let Err(e) = stream.write_all(&response_data) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
    }
}
