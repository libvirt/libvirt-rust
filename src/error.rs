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

extern {
    fn virGetLastErrorMessage() -> *const libc::c_char;
    fn virGetLastError() -> *const virError;
}

pub struct VirtError {
    pub code: i32,
    pub domain: i32,
    pub message: String,
    pub level: i32,
}

impl VirtError {
    pub fn get_last_error() -> VirtError {
        unsafe {
            let err: &virError = &*virGetLastError();
            VirtError {
                code: err.code,
                domain: err.domain,
                message: CStr::from_ptr(
                    err.message).to_string_lossy().into_owned(),
                level: err.level,
            }
        }
    }
}

pub fn last_error_message() -> String {
    unsafe {
        CStr::from_ptr(
            virGetLastErrorMessage()).to_string_lossy().into_owned()
    }
}
