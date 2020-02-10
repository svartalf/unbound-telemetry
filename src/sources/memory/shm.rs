use std::fmt;
use std::io;
use std::ops::Deref;
use std::ptr;
use std::slice;

/// `SHM_RDONLY` is not defined in `libc` for NetBSD.
///
/// TODO: Contribute it later
#[cfg(target_os = "netbsd")]
const SHM_RDONLY: libc::c_int = 0o10000;
#[cfg(not(target_os = "netbsd"))]
const SHM_RDONLY: libc::c_int = libc::SHM_RDONLY;

pub struct Segment<T: Sized> {
    ptr: *const T,
}

impl<T> Segment<T> {
    /// Attach the shared memory to the current process address space
    /// in a read only mode by shm `key` provided.
    pub fn get(key: libc::key_t) -> io::Result<Self> {
        // `shmflg` argument is zero and we are trying to attach
        // to the previously created shared memory segment.
        let id = unsafe { libc::shmget(key, 0, 0) };
        log::trace!("`shmget({})` call returned the segment id {}", key, id);
        if id == -1 {
            return Err(io::Error::last_os_error());
        }

        let result = unsafe { libc::shmat(id, ptr::null(), SHM_RDONLY) };
        log::trace!("`shmat({})` call resulted in code {}", id, result as isize);

        if result as isize == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Segment {
                ptr: result as *const T,
            })
        }
    }

    /// Return slice of `T` from the underline pointer.
    ///
    /// ## Safety
    ///
    /// Same issues as with [std::slice::from_raw_parts](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
    ///
    /// TODO: While it is working, is not really cool to do the `Segment::get().as_slice()`
    /// it might be better to create two different fabric methods,
    /// like `new(key: key_t)` and `slice(key: key_t, len: usize)`?
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        slice::from_raw_parts(self.ptr, len)
    }
}

impl<T> Deref for Segment<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for Segment<T> {
    fn drop(&mut self) {
        let result = unsafe { libc::shmdt(self.ptr as *const libc::c_void) };

        assert_eq!(result, 0, "Unable to detach shared memory segment");
    }
}

impl<T> fmt::Debug for Segment<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Segment").field("ptr", self.deref()).finish()
    }
}
