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

use std::convert::TryFrom;
use std::str;

use error::Error;

pub mod sys {
    #[repr(C)]
    pub struct virStream {}

    pub type virStreamPtr = *mut virStream;
}

#[link(name = "virt")]
extern "C" {
    fn virStreamSend(c: sys::virStreamPtr,
                     data: *const libc::c_char,
                     nbytes: libc::c_uint)
                     -> libc::c_int;
    fn virStreamRecv(c: sys::virStreamPtr,
                     data: *mut libc::c_char,
                     nbytes: libc::size_t)
                     -> libc::c_int;
    fn virStreamFree(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamAbort(c: sys::virStreamPtr) -> libc::c_int;
    fn virStreamFinish(c: sys::virStreamPtr) -> libc::c_int;
}

pub type StreamEventType = self::libc::c_uint;
pub const VIR_STREAM_EVENT_READABLE: StreamEventType = (1 << 0);
pub const VIR_STREAM_EVENT_WRITABLE: StreamEventType = (1 << 1);
pub const VIR_STREAM_EVENT_ERROR: StreamEventType = (1 << 2);
pub const VIR_STREAM_EVENT_HANGUP: StreamEventType = (1 << 3);

#[derive(Debug)]
pub struct Stream {
    ptr: Option<sys::virStreamPtr>,
}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Stream, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl Stream {
    pub fn new(ptr: sys::virStreamPtr) -> Stream {
        Stream { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr.unwrap()
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStreamFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
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

    pub fn send(&self, data: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virStreamSend(self.as_ptr(),
                                    string_to_c_chars!(data),
                                    data.len() as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
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
}
