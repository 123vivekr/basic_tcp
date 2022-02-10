use std::io::{Write, BufRead};
use std::net::TcpStream;
use basic_tcp_server::{FromClient, FromServer};
use serde_json;
use std::io::BufReader;

fn main() {
    let mut stream= TcpStream::connect("127.0.0.1:12345").unwrap();
    
    // let payload= FromClient::Ping;
    // let request = serde_json::to_string(&payload).unwrap();
    // stream.write(request.as_bytes()).unwrap();

    let payload = FromClient::Hi {
        id: 0,
        name: "Vivek".to_string(),
    };
    let request = serde_json::to_string(&payload).unwrap();
    stream.write(request.as_bytes()).unwrap();

    let mut reader = BufReader::new(&stream);
    let mut payload = String::new();

    reader.read_line(&mut payload).unwrap();
    let payload: FromServer= serde_json::from_str(&payload).unwrap(); 
    match payload {
        FromServer::PingResponse => {
            println!("Ping response");
        },
        FromServer::Hello { id, message } => {
            println!("{}", message);
        }
    };
}