use std::io;
//use std::time::Duration;

use super::ffi;
use super::shm::Segment;

use crate::statistics::Statistics;

pub struct SharedMemory {
    server: Segment<ffi::ShmStatInfo>,
    threads: Segment<ffi::StatsInfo>,
}

impl SharedMemory {
    pub fn get(key: libc::key_t) -> io::Result<Self> {
        log::trace!("Acquiring shared memory region access for key {}", key);
        let server = Segment::<ffi::ShmStatInfo>::get(key)?;
        // TODO: `key + 1` might overflow
        let threads = Segment::<ffi::StatsInfo>::get(key + 1)?;
        log::debug!(
            "Successfully acquired an access to the unbound shared memory region with key {}",
            key
        );

        Ok(SharedMemory { server, threads })
    }

    pub fn server(&self) -> &ffi::ShmStatInfo {
        &self.server
    }

    pub fn total(&self) -> &ffi::StatsInfo {
        &self.threads
    }

    pub fn threads(&self) -> &[ffi::StatsInfo] {
        if self.server.num_threads > 0 {
            let threads = unsafe { self.threads.as_slice(self.server.num_threads as usize + 1) };
            &threads[1..]
        } else {
            &[]
        }
    }
}

impl From<SharedMemory> for Statistics {
    fn from(_shm: SharedMemory) -> Statistics {
        //        let server = shm.server();
        //        let total = shm.total();
        //        let threads = shm.threads();

        unimplemented!()
    }
}
