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
    fn virStoragePoolCreate(d: virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolDestroy(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolUndefine(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolFree(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsActive(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsPersistent(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolGetName(d: virStoragePoolPtr) -> *const libc::c_char;
    fn virStoragePoolGetXMLDesc(d: virStoragePoolPtr, flags: libc::c_uint) -> *const libc::c_char;
}

pub type StoragePoolXMLFlags = self::libc::c_uint;
pub const VIR_STORAGE_POOL_XML_INACTIVE:StoragePoolXMLFlags = 1 << 0;

pub type StoragePoolCreateFlags = self::libc::c_uint;
pub const STORAGE_POOL_CREATE_NORMAL:StoragePoolCreateFlags = 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD: StoragePoolCreateFlags = 1 << 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD_OVERWRITE: StoragePoolCreateFlags = 1 << 1;
pub const STORAGE_POOL_CREATE_WITH_BUILD_NO_OVERWRITE: StoragePoolCreateFlags = 1 << 2;



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

    pub fn get_xml_desc(&self, flags:StoragePoolXMLFlags) -> Result<&str, Error> {
        unsafe {
            let xml = virStoragePoolGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(xml).to_bytes()).unwrap())
        }
    }

    pub fn create(&self, flags: StoragePoolCreateFlags) -> Result<(), Error> {
        unsafe {
            if virStoragePoolCreate(self.d, flags) == -1 {
                return Err(Error::new());
            }
            return Ok(());
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

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsActive(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsPersistent(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
