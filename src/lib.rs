use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Ping,
    Hi {
        id: u8,
        name: String,
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    PingResponse,
    Hello {
        id: u8,
        message: String,
    }
}