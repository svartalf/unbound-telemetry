use std::io;

mod ffi;
mod shm;
mod types;
mod wrappers;

use self::shm::Segment;
use self::wrappers::SharedMemory;
use super::Source;
use crate::Statistics;

/// Data source which loads `unbound` statistics via the shared memory.
///
/// TODO: It feels like shm source is racy right now, as there is no locking anywhere
/// and read data is changed by `unbound` on the fly during the reading
#[derive(Debug, Default)]
pub struct SharedMemorySource {
    shm_key: libc::key_t,
}

impl SharedMemorySource {
    pub fn new(shm_key: libc::key_t) -> SharedMemorySource {
        SharedMemorySource { shm_key }
    }
}

#[async_trait::async_trait]
impl Source for SharedMemorySource {
    async fn healthcheck(&self) -> io::Result<()> {
        Segment::<ffi::ShmStatInfo>::get(self.shm_key).map(|_| ())
    }

    async fn observe(&self) -> io::Result<Statistics> {
        let memory = SharedMemory::get(self.shm_key)?;

        Ok(Statistics::from(memory))
    }
}
