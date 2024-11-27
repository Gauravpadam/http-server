use crate::tcp_server::TcpServer;
use crate::traits::Server;

pub struct HttpServer {
    server: TcpServer,
}

impl HttpServer {
    fn new(host: &str, port: u16) -> Self {
        Self {
            server: TcpServer::new(host, port),
        }
    }
}

impl Server for HttpServer {
    fn handle_request(&self, data: &[u8]) -> &[u8; 14] {
        return b"Received data!";
    }

    fn
}
