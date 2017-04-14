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
use std::{str};

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
    fn virStoragePoolCreate(c: virConnectPtr, flags: libc::c_uint) -> virStoragePoolPtr;
    fn virStoragePoolRefresh(d: virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolDestroy(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolUndefine(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolFree(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsActive(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsPersistent(d: virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolGetName(d: virStoragePoolPtr) -> *const libc::c_char;
    fn virStoragePoolGetXMLDesc(d: virStoragePoolPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virStoragePoolGetUUIDString(d: virStoragePoolPtr, uuid: *mut libc::c_char) -> libc::c_int;

    //TODO(sahid): need to be implemented...
    fn virStorageVolGetInfo() -> ();
    fn virStorageVolLookupByKey() -> ();
    fn virStoragePoolBuild() -> ();
    fn virStoragePoolSetAutostart() -> ();
    fn virStorageVolDelete() -> ();
    fn virStoragePoolCreateXML() -> ();
    fn virStorageVolLookupByName() -> ();
    fn virStorageVolGetKey() -> ();
    fn virStorageVolDownload() -> ();
    fn virStoragePoolListAllVolumes() -> ();
    fn virStorageVolWipe() -> ();
    fn virStorageVolUpload() -> ();
    fn virStorageVolGetName() -> ();
    fn virStoragePoolLookupByVolume() -> ();
    fn virStorageVolLookupByPath() -> ();
    fn virStoragePoolGetAutostart() -> ();
    fn virStoragePoolListVolumes() -> ();
    fn virStorageVolWipePattern() -> ();
    fn virStoragePoolNumOfVolumes() -> ();
    fn virStorageVolCreateXML() -> ();
    fn virStorageVolRef() -> ();
    fn virStorageVolFree() -> ();
    fn virStoragePoolDefineXML() -> ();
    fn virStoragePoolGetConnect() -> ();
    fn virStorageVolGetPath() -> ();
    fn virStorageVolGetXMLDesc() -> ();
    fn virStorageVolGetConnect() -> ();
    fn virStorageVolCreateXMLFrom() -> ();
    fn virStorageVolResize() -> ();
    fn virStoragePoolGetInfo() -> ();
    fn virStoragePoolDelete() -> ();
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

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virStoragePoolGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virStoragePoolGetUUIDString(self.d, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn get_xml_desc(&self, flags:StoragePoolXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virStoragePoolGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn create(conn: &Connect, flags: StoragePoolCreateFlags) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolCreate(conn.as_ptr(), flags);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool{d: ptr});
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

    pub fn refresh(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStoragePoolRefresh(self.d, flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
