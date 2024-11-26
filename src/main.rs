mod tcp_server;
use log::error;

fn main() {
    let tcp_server = tcp_server::server::TcpServer::new("127.0.0.1", "8000");

    tcp_server::server::TcpServer::serve(tcp_server).unwrap_or_else(|e| error!("{}", e))
}
