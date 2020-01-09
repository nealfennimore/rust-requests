use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io;
use tokio::prelude::*;
use std::net::SocketAddr;
use std::net::{Shutdown};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server: &'static str = "127.0.0.1:8080";
    let mut listener = TcpListener::bind(server).await?;

    println!("Server running on {:?}", server);

    loop {
        match listener.accept().await{
            Ok((socket, addr)) => process_socket(socket, addr).await,
            Err(e) => eprintln!("error {:?}", e),
        }
    }
}

const TCP_PACKET_LENGTH: usize = 65535;

async fn process_socket(mut socket: TcpStream, addr: SocketAddr){
    let payload = str::repeat("-", TCP_PACKET_LENGTH * 2);
    let result = socket.write(payload.as_bytes()).await;
    println!("wrote to stream; success={:?}", result.is_ok());
    println!("from={:?}", addr);
}