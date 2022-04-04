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

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

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

extern "C" fn noop(_data: *mut libc::c_void, _error: sys::virErrorPtr) {}

impl Error {
    pub fn new() -> Error {
        unsafe {
            let ptr: sys::virErrorPtr = sys::virGetLastError();
            Error {
                code: (*ptr).code,
                domain: (*ptr).domain,
                message: c_chars_to_string!((*ptr).message, nofree),
                level: ErrorLevel::from((*ptr).level),
            }
        }
    }

    /// Removes the current virt error function
    /// Use this to disable the default printing to stdout of all errors by virt
    pub fn clear_error_func() {
        unsafe {
            sys::virSetErrorFunc(std::ptr::null_mut(), Some(noop));
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

impl Default for Error {
    fn default() -> Self {
        Self::new()
    }
}
