use std::io;

use crate::Statistics;

mod control;
#[cfg(unix)]
mod memory;

#[cfg(unix)]
pub use self::control::UdsTransport;
pub use self::control::{RemoteControlSource, TextTransport, TlsTransport};
#[cfg(unix)]
pub use self::memory::SharedMemorySource;

/// Source to fetch `unbound` statistics from.
///
/// It could be shared memory region or TLS socket, for example.
#[async_trait::async_trait]
pub trait Source {
    /// Check if connection to `unbound` can be established.
    async fn healthcheck(&self) -> io::Result<()>;

    /// Attempt to fetch the `unbound` statistics from this source.
    async fn observe(&self) -> io::Result<Statistics>;
}
