//! Data source which receives statistics via TLS socket.

use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use native_tls::{Certificate, Identity, TlsConnector as NativeTlsConnector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::{TlsConnector, TlsStream};

use super::Source;
use crate::Statistics;

pub struct TlsSource {
    connector: TlsConnector,
    host: String,
}

impl TlsSource {
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

        Ok(TlsSource {
            connector: TlsConnector::from(connector),
            host,
        })
    }

    async fn connect(&self) -> io::Result<TlsStream<TcpStream>> {
        let socket = TcpStream::connect(&self.host).await?;

        let stream = self
            .connector
            .connect("unbound", socket)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, e))?;

        Ok(stream)
    }
}

#[async_trait::async_trait]
impl Source for TlsSource {
    async fn healthcheck(&self) -> io::Result<()> {
        self.connect().await.map(|_| ())
    }

    async fn observe(&self) -> io::Result<Statistics> {
        let mut socket = self.connect().await?;
        socket.write_all(b"UBCT1 stats_noreset\n").await?;
        let mut buffer = String::new();

        socket.read_to_string(&mut buffer).await?;

        Statistics::from_str(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
