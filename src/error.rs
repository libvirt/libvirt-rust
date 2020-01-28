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

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub mod sys {
    extern crate libc;

    #[repr(C)]
    pub struct virError {
        pub code: libc::c_int,
        pub domain: libc::c_int,
        pub message: *mut libc::c_char,
        pub level: libc::c_uint,
    }

    pub type virErrorPtr = *mut virError;
}

#[link(name = "virt")]
extern "C" {
    fn virGetLastError() -> sys::virErrorPtr;
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum ErrorLevel {
    NONE = 0,
    /// A simple warning.
    WARNING = 1,
    /// An error.
    ERROR = 2,
}
impl_from! { u32, ErrorLevel }

/// Error handling
///
/// See: http://libvirt.org/html/libvirt-virterror.html
#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: i32,
    pub domain: i32,
    pub message: String,
    pub level: ErrorLevel,
}

impl Error {
    pub fn new() -> Error {
        unsafe {
            let ptr: sys::virErrorPtr = virGetLastError();
            Error {
                code: (*ptr).code,
                domain: (*ptr).domain,
                message: c_chars_to_string!((*ptr).message, nofree),
                level: ErrorLevel::from((*ptr).level),
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:?}: code: {} domain: {} - {}",
            self.level, self.code, self.domain, self.message
        )
    }
}
