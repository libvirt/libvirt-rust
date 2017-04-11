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

#![allow(improper_ctypes)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::str;

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virStoragePool {
}

#[allow(non_camel_case_types)]
pub type virStoragePoolPtr = *const virStoragePool;

#[link(name = "virt")]
extern {
    fn virStoragePoolLookupByID(c: virConnectPtr, id: libc::c_int) -> virStoragePoolPtr;
    fn virStoragePoolLookupByName(c: virConnectPtr, id: *const libc::c_char) -> virStoragePoolPtr;
    fn virStoragePoolLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virStoragePoolPtr;
    fn virStoragePoolDestroy(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsActive(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolGetName(d: virStoragePoolPtr) -> *const libc::c_char;
}


pub struct StoragePool {
    pub d: virStoragePoolPtr
}

impl StoragePool {

    pub fn as_ptr(&self) -> virStoragePoolPtr {
        self.d
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<&str, Error> {
        unsafe {
            let n = virStoragePoolGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }


    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolIsActive(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
