/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <https://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

use libc::{c_char, c_int, c_uint, c_void};

use crate::connect::Connect;
use crate::error::Error;
use crate::util::{check_neg, check_null};

// wrapper for callbacks
extern "C" fn event_callback(c: sys::virStreamPtr, flags: c_int, opaque: *mut c_void) {
    let flags = flags as sys::virStreamFlags;
    let shadow_self = unsafe { &mut *(opaque as *mut Stream) };
    if let Some(callback) = &mut shadow_self.callback {
        callback(
            unsafe {
                sys::virStreamRef(c);
                &Stream::from_ptr(c)
            },
            flags,
        );
    }
}

extern "C" fn event_free(_opaque: *mut c_void) {}

type StreamCallback = dyn FnMut(&Stream, sys::virStreamEventType);

// #[derive(Debug)]
pub struct Stream {
    ptr: sys::virStreamPtr,
    callback: Option<Box<StreamCallback>>,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.callback.is_some() {
            if let Err(e) = self.event_remove_callback() {
                panic!("Unable to remove event callback for Stream: {e}")
            }
        }
        if let Err(e) = check_neg!(unsafe { sys::virStreamFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on stream: {e}")
        }
    }
}

impl Clone for Stream {
    /// Creates a copy of a stream.
    ///
    /// Increments the internal reference counter on the given
    /// stream.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virStreamRef(self.as_ptr()) }) {
            panic!("Unable to add reference on stream: {e}")
        }
        unsafe { Stream::from_ptr(self.as_ptr()) }
    }
}

impl Stream {
    /// Create a new stream object
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamNew>
    pub fn new(conn: &Connect, flags: sys::virStreamFlags) -> Result<Stream, Error> {
        let ptr = check_null!(unsafe { sys::virStreamNew(conn.as_ptr(), flags as c_uint) })?;
        Ok(unsafe { Stream::from_ptr(ptr) })
    }

    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virStreamPtr) -> Stream {
        Stream {
            ptr,
            callback: None,
        }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr
    }

    /// Complete I/O on the stream
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamFinish>
    pub fn finish(self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStreamFinish(self.as_ptr()) })?;
        Ok(())
    }

    /// Abort I/O on the stream
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamAbort>
    pub fn abort(self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStreamAbort(self.as_ptr()) })?;
        Ok(())
    }

    /// Send data to the stream
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamSend>
    pub fn send(&self, data: &[u8]) -> Result<isize, Error> {
        let ret = check_neg!(unsafe {
            sys::virStreamSend(self.as_ptr(), data.as_ptr() as *mut c_char, data.len())
        })?;
        Ok(ret as isize)
    }

    /// Receive data from the stream
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamRecv>
    pub fn recv(&self, buf: &mut [u8]) -> Result<isize, Error> {
        let ret = check_neg!(unsafe {
            sys::virStreamRecv(self.as_ptr(), buf.as_mut_ptr() as *mut c_char, buf.len())
        })?;
        Ok(ret as isize)
    }

    /// Add a stream event callback
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamEventAddCallback>
    pub fn event_add_callback<F: 'static + FnMut(&Stream, sys::virStreamEventType)>(
        &mut self,
        events: sys::virStreamEventType,
        cb: F,
    ) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            let ptr = self as *mut _ as *mut _;
            sys::virStreamEventAddCallback(
                self.as_ptr(),
                events as c_int,
                Some(event_callback),
                ptr,
                Some(event_free),
            )
        })?;
        self.callback = Some(Box::new(cb));
        Ok(())
    }

    /// Update the stream event callback
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamEventUpdateCallback>
    pub fn event_update_callback(&self, events: sys::virStreamEventType) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virStreamEventUpdateCallback(self.as_ptr(), events as c_int)
        })?;
        Ok(())
    }

    /// Remove the stream event callback
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-stream.html#virStreamEventRemoveCallback>
    pub fn event_remove_callback(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStreamEventRemoveCallback(self.as_ptr()) })?;
        Ok(())
    }
}
