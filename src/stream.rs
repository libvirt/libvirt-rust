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
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

extern crate libc;

use connect::sys::virConnectPtr;
use connect::Connect;
use std::convert::TryFrom;

use error::Error;

pub mod sys {
    #[repr(C)]
    pub struct virStream {}

    pub type virStreamPtr = *mut virStream;
}

#[link(name = "virt")]
extern "C" {
    fn virStreamNew(c: virConnectPtr, flags: libc::c_uint) -> sys::virStreamPtr;
    fn virStreamSend(
        c: sys::virStreamPtr,
        data: *const libc::c_char,
        nbytes: libc::size_t,
    ) -> libc::c_int;
    fn virStreamRecv(
        c: sys::virStreamPtr,
        data: *mut libc::c_char,
        nbytes: libc::size_t,
    ) -> libc::c_int;
    fn virStreamFree(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamAbort(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamFinish(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamEventAddCallback(
        c: sys::virStreamPtr,
        event: libc::c_int,
        callback: StreamEventCallback,
        opaque: *const libc::c_void,
        ff: FreeCallback,
    ) -> libc::c_int;
    fn virStreamEventUpdateCallback(c: sys::virStreamPtr, events: libc::c_int) -> libc::c_int;
    fn virStreamEventRemoveCallback(c: sys::virStreamPtr) -> libc::c_int;
}

pub type StreamEventType = self::libc::c_uint;
pub const VIR_STREAM_EVENT_READABLE: StreamEventType = (1 << 0);
pub const VIR_STREAM_EVENT_WRITABLE: StreamEventType = (1 << 1);
pub const VIR_STREAM_EVENT_ERROR: StreamEventType = (1 << 2);
pub const VIR_STREAM_EVENT_HANGUP: StreamEventType = (1 << 3);

pub type StreamFlags = self::libc::c_uint;
pub const VIR_STREAM_NONBLOCK: StreamFlags = (1 << 0);

pub type StreamEventCallback = extern "C" fn(sys::virStreamPtr, libc::c_int, *const libc::c_void);
pub type FreeCallback = extern "C" fn(*mut libc::c_void);

// wrapper for callbacks
extern "C" fn event_callback(
    c: sys::virStreamPtr,
    flags: libc::c_int,
    opaque: *const libc::c_void,
) {
    let flags = flags as StreamFlags;
    let shadow_self = unsafe { &mut *(opaque as *mut Stream) };
    if let Some(callback) = &mut shadow_self.callback {
        callback(&Stream::from_ptr(c), flags);
    }
}

extern "C" fn event_free(_opaque: *mut libc::c_void) {}

// #[derive(Debug)]
pub struct Stream {
    ptr: Option<sys::virStreamPtr>,
    callback: Option<Box<dyn FnMut(&Stream, StreamEventType)>>,
}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for Stream, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
        if self.callback.is_some() {
            if let Err(e) = self.event_remove_callback() {
                panic!(
                    "Unable to remove event callback for Stream, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl Stream {
    pub fn new(conn: &Connect, flags: StreamFlags) -> Result<Stream, Error> {
        let ptr = unsafe { virStreamNew(conn.as_ptr(), flags as libc::c_uint) };
        if ptr.is_null() {
            return Err(Error::new());
        }
        return Ok(Stream::from_ptr(ptr));
    }

    fn from_ptr(ptr: sys::virStreamPtr) -> Stream {
        Stream {
            ptr: Some(ptr),
            callback: None,
        }
    }

    pub fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr.unwrap()
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStreamFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
        }
        self.ptr = None;
        return Ok(());
    }

    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            if virStreamFinish(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn abort(self) -> Result<(), Error> {
        unsafe {
            if virStreamAbort(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn send(&self, data: &[u8]) -> Result<usize, Error> {
        let ret = unsafe {
            virStreamSend(
                self.as_ptr(),
                data.as_ptr() as *mut libc::c_char,
                data.len(),
            )
        };
        usize::try_from(ret).map_err(|_| Error::new())
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let ret = unsafe {
            virStreamRecv(
                self.as_ptr(),
                buf.as_mut_ptr() as *mut libc::c_char,
                buf.len(),
            )
        };
        usize::try_from(ret).map_err(|_| Error::new())
    }

    pub fn event_add_callback<F: 'static + FnMut(&Stream, StreamEventType)>(
        &mut self,
        events: StreamEventType,
        cb: F,
    ) -> Result<(), Error> {
        let ret = unsafe {
            let ptr = &*self as *const _ as *const _;
            virStreamEventAddCallback(
                self.as_ptr(),
                events as libc::c_int,
                event_callback,
                ptr,
                event_free,
            )
        };
        if ret == -1 {
            return Err(Error::new());
        }
        self.callback = Some(Box::new(cb));
        return Ok(());
    }

    pub fn event_update_callback(&self, events: StreamEventType) -> Result<(), Error> {
        let ret = unsafe { virStreamEventUpdateCallback(self.as_ptr(), events as libc::c_int) };
        if ret == -1 {
            return Err(Error::new());
        }
        return Ok(());
    }

    pub fn event_remove_callback(&self) -> Result<(), Error> {
        unsafe {
            if virStreamEventRemoveCallback(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
