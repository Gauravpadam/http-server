use crate::tcp_server::TcpServer;
use crate::traits::Server;
use std::collections::HashMap;
use std::sync::Arc;
use std::{
    io::{Read, Write},
    net::TcpStream,
};
#[derive(Clone)]
pub struct HttpServer {
    server: TcpServer,
    headers: HashMap<String, String>,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server:".to_string(), "Crude Server".to_string());
        headers.insert("Content-Type:".to_string(), "text/html".to_string());

        Self {
            server: TcpServer::new(host, port),
            headers,
        }
    }

    fn response_line(&self, status_code: i32) -> &str {
        match status_code {
            200 => "HTTP/1.1 200 OK",
            404 => "HTTP/1.1 404 Not Found",
            _ => "HTTP/1.1 500 Internal Server Error", // Add a fallback for unexpected codes.
        }
    }

    fn response_headers(
        &self,
        extra_headers: Option<HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut headers = self.headers.clone();

        if let Some(extra) = extra_headers {
            extra.into_iter().for_each(|(key, value)| {
                headers.insert(key, value);
            });
        }

        headers
    }
}

impl Server for HttpServer {
    fn handle_request(&self, _data: &[u8]) -> Vec<u8> {
        let response_line = self.response_line(200);

        let headers: Vec<String> = self
            .response_headers(None)
            .into_iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

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
