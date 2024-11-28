use std::net::TcpStream;

pub trait Server: Send + Sync {
    fn handle_request(&self, data: &[u8]) -> Vec<u8>;
    fn handle_connection(&self, stream: TcpStream);
    fn start(&self);
}
