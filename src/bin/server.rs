use std::io::{Read, Write, BufRead};
use std::net::{TcpListener, TcpStream};
use basic_tcp_server::{FromClient, FromServer};
use std::thread;
use std::io::BufReader;
use serde_json;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      thread::spawn(|| {
        handle_client(stream);
      });
    }
}

fn handle_client(mut stream: TcpStream) {
  let mut reader = BufReader::new(&stream);
  let mut payload = String::new();

  reader.read_line(&mut payload).unwrap();
  let payload: FromClient = serde_json::from_str(&payload).unwrap(); 
  let response_payload;

  match payload {
    FromClient::Ping => {
      response_payload = FromServer::PingResponse;
    },
    FromClient::Hi { id, name} => {
      response_payload = FromServer::Hello {
        id: 0,
        message: format!("Hello, {}", name).to_string(),
      };
    },
  };

  let request = serde_json::to_string(&response_payload).unwrap();
  stream.write(request.as_bytes()).unwrap();
}
