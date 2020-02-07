//! Data source which receives statistics via Unix Domain Socket.

use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use super::Source;
use crate::Statistics;

#[derive(Debug)]
pub struct UdsSource {
    path: PathBuf,
}

impl UdsSource {
    pub fn new(path: impl AsRef<Path>) -> UdsSource {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    async fn connect(&self) -> io::Result<UnixStream> {
        match UnixStream::connect(&self.path).await {
            Ok(stream) => {
                log::trace!("Successfully connected to {}", self.path.display());
                Ok(stream)
            }
            Err(e) => {
                log::warn!("Unable to connect to {}: {}", self.path.display(), e);
                Err(e)
            }
        }
    }
}

#[async_trait::async_trait]
impl Source for UdsSource {
    async fn healthcheck(&self) -> io::Result<()> {
        self.connect().await.map(|_| ())
    }

    async fn observe(&self) -> io::Result<Statistics> {
        let mut socket = self.connect().await?;
        socket.write_all(b"UBCT1 stats_noreset\n").await?;
        log::trace!("Executed 'UBCT1 stats_noreset' command");
        let mut buffer = String::new();

        socket.read_to_string(&mut buffer).await?;
        log::trace!("{} bytes received via UDS", buffer.len());

        Statistics::from_str(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
