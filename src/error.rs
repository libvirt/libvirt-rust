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

use std::ffi::CStr;

#[allow(non_camel_case_types)]
#[repr(C)]
struct virError {
    code: libc::c_int,
    domain: libc::c_int,
    message: *const libc::c_char,
    level: libc::c_int,
}


#[link(name = "virt")]
extern {
//    fn virGetLastErrorMessage() -> *const libc::c_char;
    fn virGetLastError() -> *const virError;
}

pub struct Error {
    pub code: i32,
    pub domain: i32,
    pub message: String,
    pub level: i32,
}

impl Error {
    pub fn new() -> Error {
        unsafe {
            let ptr: *const virError = virGetLastError();
            Error {
                code: (*ptr).code,
                domain: (*ptr).domain,
                message: CStr::from_ptr(
                    (*ptr).message).to_string_lossy().into_owned(),
                level: (*ptr).level,
            }
        }
    }
}
/*
pub fn last_error_message() -> String {
    unsafe {
        CStr::from_ptr(
            virGetLastErrorMessage()).to_string_lossy().into_owned()
    }
}
*/
