use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

mod eth_client;
mod responses;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    let client = Arc::new(eth_client::EthClient::new().await);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let client_cpy = client.clone();

        tokio::spawn(async move { handle_connection(stream, client_cpy).await });
    }
}

async fn handle_connection(mut stream: TcpStream, client: Arc<eth_client::EthClient>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /currentBlockTime HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        match client.get_eth_ts().await {
            Some(v) => responses::get_block_time(&v),
            None => responses::get_block_time("Unknown"),
        }
    } else {
        responses::get_not_found()
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
