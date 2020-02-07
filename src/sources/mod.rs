use std::io;

use crate::Statistics;

#[cfg(unix)]
mod memory;
mod tls;
#[cfg(unix)]
mod uds;

#[cfg(unix)]
pub use self::memory::SharedMemorySource;
pub use self::tls::TlsSource;
#[cfg(unix)]
pub use self::uds::UdsSource;

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
