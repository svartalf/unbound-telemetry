use std::io;
use std::marker::Unpin;
use std::str::FromStr;

use super::Source;
use crate::Statistics;
use tokio::prelude::{AsyncRead, AsyncWrite, *};

mod text;
mod tls;
#[cfg(unix)]
mod uds;

pub use self::text::TextTransport;
pub use self::tls::TlsTransport;
#[cfg(unix)]
pub use self::uds::UdsTransport;

#[async_trait::async_trait]
pub trait RemoteControlTransport: Sized + Send + Sync {
    type Socket: AsyncRead + AsyncWrite + Send + Unpin;

    async fn connect(&self) -> io::Result<Self::Socket>;
}

pub struct RemoteControlSource<T> {
    transport: T,
}

impl<T> RemoteControlSource<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

#[async_trait::async_trait]
impl<T> Source for RemoteControlSource<T>
where
    T: RemoteControlTransport,
{
    async fn healthcheck(&self) -> io::Result<()> {
        let _ = self.transport.connect().await?;

        Ok(())
    }

    async fn observe(&self) -> io::Result<Statistics> {
        let mut socket = self.transport.connect().await?;

        socket.write_all(b"UBCT1 stats_noreset\n").await?;
        let mut buffer = String::new();

        let _ = socket.read_to_string(&mut buffer).await?;

        Statistics::from_str(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
