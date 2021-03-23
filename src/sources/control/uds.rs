//! Data source which receives statistics via Unix Domain Socket.

use std::io;
use std::net::Shutdown;
use std::path::{Path, PathBuf};

use tokio::net::UnixStream;

use super::{RemoteControlSocket, RemoteControlTransport};

#[derive(Debug)]
pub struct UdsTransport {
    path: PathBuf,
}

impl UdsTransport {
    pub fn new(path: impl AsRef<Path>) -> UdsTransport {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[async_trait::async_trait]
impl RemoteControlTransport for UdsTransport {
    type Socket = UnixStream;

    async fn connect(&self) -> io::Result<Self::Socket> {
        UnixStream::connect(&self.path).await
    }
}

#[async_trait::async_trait]
impl RemoteControlSocket for UnixStream {
    async fn close(mut self) -> io::Result<()> {
        self.shutdown(Shutdown::Both)
    }
}
