//! Data source which receives statistics via TLS socket.

use std::io;
use std::net::Shutdown;

use tokio::net::TcpStream;

use super::{RemoteControlSocket, RemoteControlTransport};

pub struct TextTransport {
    host: String,
}

impl TextTransport {
    pub fn new(host: String) -> io::Result<Self> {
        Ok(TextTransport { host })
    }
}

#[async_trait::async_trait]
impl RemoteControlTransport for TextTransport {
    type Socket = TcpStream;

    async fn connect(&self) -> io::Result<Self::Socket> {
        TcpStream::connect(&self.host).await
    }
}

#[async_trait::async_trait]
impl RemoteControlSocket for TcpStream {
    async fn close(mut self) -> io::Result<()> {
        self.shutdown(Shutdown::Both)
    }
}
