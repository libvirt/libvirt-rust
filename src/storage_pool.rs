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

use std::{str, ptr};

use connect::sys::virConnectPtr;

use connect::Connect;
use error::Error;

pub mod sys {
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virStoragePool {}

    #[allow(non_camel_case_types)]
    pub type virStoragePoolPtr = *mut virStoragePool;
}

#[link(name = "virt")]
extern "C" {
    fn virStoragePoolLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByName(c: virConnectPtr,
                                  id: *const libc::c_char)
                                  -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByUUIDString(c: virConnectPtr,
                                        uuid: *const libc::c_char)
                                        -> sys::virStoragePoolPtr;
    fn virStoragePoolCreate(c: virConnectPtr, flags: libc::c_uint) -> sys::virStoragePoolPtr;
    fn virStoragePoolRefresh(ptr: sys::virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolDestroy(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolUndefine(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolFree(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsActive(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsPersistent(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolGetName(ptr: sys::virStoragePoolPtr) -> *const libc::c_char;
    fn virStoragePoolGetXMLDesc(ptr: sys::virStoragePoolPtr,
                                flags: libc::c_uint)
                                -> *const libc::c_char;
    fn virStoragePoolGetUUIDString(ptr: sys::virStoragePoolPtr,
                                   uuid: *mut libc::c_char)
                                   -> libc::c_int;
    fn virStoragePoolGetConnect(ptr: sys::virStoragePoolPtr) -> virConnectPtr;
}

pub type StoragePoolXMLFlags = self::libc::c_uint;
pub const VIR_STORAGE_POOL_XML_INACTIVE: StoragePoolXMLFlags = 1 << 0;

pub type StoragePoolCreateFlags = self::libc::c_uint;
pub const STORAGE_POOL_CREATE_NORMAL: StoragePoolCreateFlags = 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD: StoragePoolCreateFlags = 1 << 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD_OVERWRITE: StoragePoolCreateFlags = 1 << 1;
pub const STORAGE_POOL_CREATE_WITH_BUILD_NO_OVERWRITE: StoragePoolCreateFlags = 1 << 2;


pub struct StoragePool {
    ptr: sys::virStoragePoolPtr,
}

impl Drop for StoragePool {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for StoragePool")
            }
            return;
        }
    }
}

impl StoragePool {
    pub fn new(ptr: sys::virStoragePoolPtr) -> StoragePool {
        return StoragePool { ptr: ptr };
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virStoragePoolGetConnect(self.ptr);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virStoragePoolGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virStoragePoolGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr()));
        }
    }

    pub fn get_xml_desc(&self, flags: StoragePoolXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virStoragePoolGetXMLDesc(self.ptr, flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(conn: &Connect, flags: StoragePoolCreateFlags) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolCreate(conn.as_ptr(), flags);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolUndefine(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsActive(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsPersistent(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn refresh(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStoragePoolRefresh(self.ptr, flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
