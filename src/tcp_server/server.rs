use core::str;
use log::error;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub struct TcpServer {
    host: String,
    port: String,
}

impl TcpServer {
    pub fn new(host: &str, port: &str) -> TcpServer {
        let host = host.to_string();
        let port = port.to_string();
        TcpServer { host, port }
    }

    pub fn serve(tcp_server: TcpServer) -> Result<(), failure::Error> {
        let listener = TcpListener::bind(tcp_server.host + ":" + tcp_server.port.as_str())?;
        loop {
            let (stream, _) = listener.accept()?;
            thread::spawn(move || {
                Self::handler(stream).unwrap_or_else(|error| error!("{:?}", error));
            });
        }
    }

    pub fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
        let mut buffer = [0u8; 1024];

        loop {
            let nbytes = stream.read(&mut buffer)?;
            if nbytes == 0 {
                return Ok(());
            }
            print!("{}", str::from_utf8(&buffer[..nbytes])?);
            stream.write_all(&buffer[..nbytes])?;
        }
    }
}
