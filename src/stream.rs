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

use std::convert::TryFrom;

use crate::connect::Connect;
use crate::error::Error;

// wrapper for callbacks
extern "C" fn event_callback(c: sys::virStreamPtr, flags: libc::c_int, opaque: *mut libc::c_void) {
    let flags = flags as sys::virStreamFlags;
    let shadow_self = unsafe { &mut *(opaque as *mut Stream) };
    if let Some(callback) = &mut shadow_self.callback {
        callback(unsafe { &Stream::from_ptr(c) }, flags);
    }
}

extern "C" fn event_free(_opaque: *mut libc::c_void) {}

type StreamCallback = dyn FnMut(&Stream, sys::virStreamEventType);

// #[derive(Debug)]
pub struct Stream {
    ptr: Option<sys::virStreamPtr>,
    callback: Option<Box<StreamCallback>>,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Stream: {}", e)
            }
        }
        if self.callback.is_some() {
            if let Err(e) = self.event_remove_callback() {
                panic!("Unable to remove event callback for Stream: {}", e)
            }
        }
    }
}

impl Clone for Stream {
    /// Creates a copy of a stream.
    ///
    /// Increments the internal reference counter on the given
    /// stream. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: Stream::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl Stream {
    pub fn new(conn: &Connect, flags: sys::virStreamFlags) -> Result<Stream, Error> {
        let ptr = unsafe { sys::virStreamNew(conn.as_ptr(), flags as libc::c_uint) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Stream::from_ptr(ptr) })
    }

    fn add_ref(&self) -> Result<Stream, Error> {
        unsafe {
            if sys::virStreamRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { Stream::from_ptr(self.as_ptr()) })
    }

    unsafe fn from_ptr(ptr: sys::virStreamPtr) -> Stream {
        Stream {
            ptr: Some(ptr),
            callback: None,
        }
    }

    pub fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr.unwrap()
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virStreamFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn finish(self) -> Result<(), Error> {
        let ret = unsafe { sys::virStreamFinish(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn abort(self) -> Result<(), Error> {
        let ret = unsafe { sys::virStreamAbort(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn send(&self, data: &[u8]) -> Result<usize, Error> {
        let ret = unsafe {
            sys::virStreamSend(
                self.as_ptr(),
                data.as_ptr() as *mut libc::c_char,
                data.len(),
            )
        };
        usize::try_from(ret).map_err(|_| Error::last_error())
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let ret = unsafe {
            sys::virStreamRecv(
                self.as_ptr(),
                buf.as_mut_ptr() as *mut libc::c_char,
                buf.len(),
            )
        };
        usize::try_from(ret).map_err(|_| Error::last_error())
    }

    pub fn event_add_callback<F: 'static + FnMut(&Stream, sys::virStreamEventType)>(
        &mut self,
        events: sys::virStreamEventType,
        cb: F,
    ) -> Result<(), Error> {
        let ret = unsafe {
            let ptr = self as *mut _ as *mut _;
            sys::virStreamEventAddCallback(
                self.as_ptr(),
                events as libc::c_int,
                Some(event_callback),
                ptr,
                Some(event_free),
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.callback = Some(Box::new(cb));
        Ok(())
    }

    pub fn event_update_callback(&self, events: sys::virStreamEventType) -> Result<(), Error> {
        let ret =
            unsafe { sys::virStreamEventUpdateCallback(self.as_ptr(), events as libc::c_int) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn event_remove_callback(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virStreamEventRemoveCallback(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }
}
