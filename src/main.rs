mod modules;
use modules::http_server::HttpServer;
use modules::tcp_server;
use modules::traits;
use modules::traits::Server;

fn main() {
    let server = HttpServer::new("0.0.0.0", 8000);
    server.start();
}
