use crate::modules::http_request::HttpRequest;
use crate::tcp_server::TcpServer;
use crate::traits::Server;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
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
        headers.insert("Server".to_string(), "Crude Server".to_string());
        headers.insert("Content-Type".to_string(), "text/html".to_string());

        Self {
            server: TcpServer::new(host, port),
            headers,
        }
    }

    fn response_line(&self, status_code: i32) -> &str {
        match status_code {
            200 => "HTTP/1.1 200 OK",
            404 => "HTTP/1.1 404 Not Found",
            501 => "HTTP/1.1 501 Not implemented",
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

    pub fn handle_get(&self, request: HttpRequest) -> String {
        let filename = request
            .uri
            .unwrap()
            .strip_prefix("/")
            .unwrap_or("")
            .to_owned();
        let path = Path::new("static_assets").join(filename);
        let display = path.display();
        let mut response_line = String::new();
        let mut response_headers: Vec<String>;
        let blank_line = "\r\n";
        let mut response_body: String;

        match File::open(path) {
            // Err(why) => panic!("couldn't open {}: {}", display, why), TODO: Implement while logging
            Err(why) => {
                response_line = self.response_line(404).to_string();
                response_headers = self
                    .response_headers(None)
                    .into_iter()
                    .map(|(key, value)| format!("{}:{}", key, value))
                    .collect();
                response_body = "<h1>404 Not found</h1>".to_string();
            }
            Ok(mut file) => {
                response_line = self.response_line(200).to_string();
                response_headers = self
                    .response_headers(None)
                    .into_iter()
                    .map(|(key, value)| format!("{}:{}", key, value))
                    .collect();
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    // Err(why) => panic!("couldn't read {}: {}", display, why), TODO: Implement while logging
                    Err(why) => response_body = "<h1>Server could not read file</h1>".to_string(),
                    Ok(_) => response_body = s,
                }
            }
        };

        let response = format!(
            "{}\r\n{}\r\n{}{}",
            response_line,
            response_headers.join("\r\n"),
            blank_line,
            response_body
        );

        println!("Response in {}", response);

        response
    }

    pub fn http_501_handler(&self, request: HttpRequest) -> String {
        let response_line = self.response_line(501);
        let response_headers: Vec<String> = self
            .response_headers(None)
            .into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect();
        let blank_line = "\r\n";
        let response_body = "<h1>501 Not Implemented</h1>";

        let response = format!(
            "{}\r\n{}\r\n{}{}",
            response_line,
            response_headers.join("\r\n"),
            blank_line,
            response_body
        );

        response
    }
}

impl Server for HttpServer {
    fn handle_request(&self, data: &[u8]) -> Vec<u8> {
        let request = HttpRequest::new(data);
        let mut response = String::new();

        match request.method {
            Some(ref method) => {
                if method == "GET" {
                    response = self.handle_get(request);
                }
            }
            _ => response = self.http_501_handler(request),
        }

        print!("{}", response);

        response.as_bytes().to_vec()

        // let headers: Vec<String> = self
        //     .response_headers(None)
        //     .into_iter()
        //     .map(|(key, value)| format!("{}: {}", key, value))
        //     .collect();

        // let header_string = headers.join("\r\n");
        // let blank_line = "\r\n";
        // let response_body = "
        //     <html>
        //     <body>
        //     <h1>request received!</h1>
        //     </body>
        //     </html>
        //     ";

        // let response = format!(
        //     "{}\r\n{}\r\n{}{}",
        //     response_line, header_string, blank_line, response_body
        // );
        // response.as_bytes().to_vec()
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
