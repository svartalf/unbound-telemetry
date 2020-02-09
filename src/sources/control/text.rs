//! Data source which receives statistics via TLS socket.

use std::io;

use tokio::net::TcpStream;

use super::RemoteControlTransport;

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
