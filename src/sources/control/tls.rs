//! Data source which receives statistics via TLS socket.

use std::fs;
use std::io;
use std::path::Path;

use native_tls::{Certificate, Identity, TlsConnector as NativeTlsConnector};
use tokio::net::TcpStream;
use tokio_tls::{TlsConnector, TlsStream};

use super::RemoteControlTransport;

pub struct TlsTransport {
    connector: TlsConnector,
    host: String,
}

impl TlsTransport {
    pub fn new(ca: impl AsRef<Path>, cert: impl AsRef<Path>, key: impl AsRef<Path>, host: String) -> io::Result<Self> {
        let ca = fs::read(ca)?;
        let ca = Certificate::from_pem(&ca).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let cert = fs::read(cert)?;
        let key = fs::read(key)?;

        let identity = Identity::from_pkcs8(&cert, &key).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let connector = NativeTlsConnector::builder()
            .add_root_certificate(ca)
            .identity(identity)
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(TlsTransport {
            connector: TlsConnector::from(connector),
            host,
        })
    }
}

#[async_trait::async_trait]
impl RemoteControlTransport for TlsTransport {
    type Socket = TlsStream<TcpStream>;

    async fn connect(&self) -> io::Result<Self::Socket> {
        let socket = TcpStream::connect(&self.host).await?;

        let stream = self
            .connector
            .connect("unbound", socket)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, e))?;

        Ok(stream)
    }
}
