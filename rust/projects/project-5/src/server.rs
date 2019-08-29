use crate::common::{Request, Response};
use crate::{KvsEngine, KvsError, Result};
use std::net::SocketAddr;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use futures::future;

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    /// Run the server listening on the given address
    pub fn run(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(&addr)?;
        let server = listener
            .incoming()
            .map_err(|e| error!("IO error: {}", e))
            .for_each(move |tcp| {
                let engine = self.engine.clone();
                serve(engine, tcp).map_err(|e| error!("Error on serving client: {}", e))
            });
        tokio::run(server);
        Ok(())
    }
}

fn serve<E: KvsEngine>(engine: E, tcp: TcpStream) -> impl Future<Output = Result<()>> {
    futures::pending()
}
