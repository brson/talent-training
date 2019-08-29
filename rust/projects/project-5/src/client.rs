use crate::common::{Request, Response};
use crate::Result;
use std::net::SocketAddr;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::future;

/// Key value store client
pub struct KvsClient {
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect(addr: SocketAddr) -> impl Future<Output = Result<Self>> {
        future::ok::pending()
    }

    /// Get the value of a given key from the server.
    pub fn get(self, key: String) -> impl Future<Output = Result<(Option<String>, Self)>> {
        future::ok::pending()
    }

    /// Set the value of a string key in the server.
    pub fn set(self, key: String, value: String) -> impl Future<Output = Result<Self>> {
        future::ok::pending()
    }

    /// Remove a string key in the server.
    pub fn remove(self, key: String) -> impl Future<Output = Result<Self>> {
        future::ok::pending()
    }
}
