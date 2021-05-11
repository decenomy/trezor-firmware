use core::{convert::TryFrom, ops::Deref, ptr, slice};

use crate::{error::Error, micropython::obj::Obj};

use super::ffi;

/// Represents an immutable slice of bytes stored on the MicroPython heap and
/// owned by values that obey the buffer protocol, such as `bytes`, `str`,
/// `bytearray` or `memoryview`.
///
/// # Safety
///
/// In most cases, it is unsound to store `Buffer` values in a GC-unreachable
/// location, such as static data.
pub struct Buffer {
    ptr: *mut u8,
    len: usize,
}

impl TryFrom<Obj> for Buffer {
    type Error = Error;

    fn try_from(obj: Obj) -> Result<Self, Self::Error> {
        let mut bufinfo = ffi::mp_buffer_info_t {
            buf: ptr::null_mut(),
            len: 0,
            typecode: 0,
        };
        // SAFETY: We assume that if `ffi::mp_get_buffer` returns successfully,
        // `bufinfo.buf` contains a pointer to data of `bufinfo.len` bytes. Here
        // we consider this data either GC-allocated or effectively 'static, and
        // store the pointer directly in `Buffer`. It is unsound to store
        // `Buffer` values in a GC-unreachable location, such as static data.
        if unsafe { ffi::mp_get_buffer(obj, &mut bufinfo, ffi::MP_BUFFER_READ as _) } {
            Ok(Self {
                ptr: bufinfo.buf as _,
                len: bufinfo.len as _,
            })
        } else {
            Err(Error::NotBuffer)
        }
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        if self.ptr.is_null() {
            // `self.ptr` can be null if len == 0.
            &[]
        } else {
            // SAFETY: We assume that `self.ptr` is pointing to memory:
            //  - without any mutable references,
            //  - immutable for the whole lifetime of `&self`,
            //  - with at least `self.len` bytes.
            unsafe { slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

impl Buffer {
    /// Convert to a mutable slice of bytes.
    ///
    /// # Safety
    ///
    /// This method is unsafe because the caller has to guarantee that the bytes
    /// referenced by `self` are unique, without any other immutable or mutable
    /// references.
    pub unsafe fn as_mut(&mut self) -> &mut [u8] {
        if self.ptr.is_null() {
            // `ptr` can be null if len == 0.
            &mut []
        } else {
            // SAFETY: We assume that `ptr` is pointing to memory:
            //  - without any live references.
            //  - of length `len` bytes.
            unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }
}
