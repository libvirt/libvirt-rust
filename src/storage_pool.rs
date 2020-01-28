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

use std::str;

use connect::sys::virConnectPtr;
use storage_vol::sys::virStorageVolPtr;

use connect::Connect;
use error::Error;
use storage_vol::StorageVol;

pub mod sys {
    extern crate libc;

    #[repr(C)]
    pub struct virStoragePool {}

    pub type virStoragePoolPtr = *mut virStoragePool;

    #[repr(C)]
    #[derive(Default)]
    pub struct virStoragePoolInfo {
        pub state: libc::c_int,
        pub capacity: libc::c_ulonglong,
        pub allocation: libc::c_ulonglong,
        pub available: libc::c_ulonglong,
    }

    pub type virStoragePoolInfoPtr = *mut virStoragePoolInfo;
}

#[link(name = "virt")]
extern "C" {
    fn virStoragePoolDefineXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virStoragePoolPtr;
    fn virStoragePoolCreateXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByName(
        c: virConnectPtr,
        id: *const libc::c_char,
    ) -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virStoragePoolPtr;
    fn virStoragePoolLookupByVolume(v: virStorageVolPtr) -> sys::virStoragePoolPtr;
    fn virStoragePoolCreate(ptr: sys::virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolBuild(ptr: sys::virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolRefresh(ptr: sys::virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolDestroy(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolDelete(ptr: sys::virStoragePoolPtr, flags: libc::c_uint) -> libc::c_int;
    fn virStoragePoolUndefine(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolFree(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsActive(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolIsPersistent(ptr: sys::virStoragePoolPtr) -> libc::c_int;
    fn virStoragePoolGetName(ptr: sys::virStoragePoolPtr) -> *const libc::c_char;
    fn virStoragePoolGetXMLDesc(
        ptr: sys::virStoragePoolPtr,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virStoragePoolGetUUIDString(
        ptr: sys::virStoragePoolPtr,
        uuid: *mut libc::c_char,
    ) -> libc::c_int;
    fn virStoragePoolGetConnect(ptr: sys::virStoragePoolPtr) -> virConnectPtr;
    fn virStoragePoolGetAutostart(
        ptr: sys::virStoragePoolPtr,
        autostart: *mut libc::c_int,
    ) -> libc::c_int;
    fn virStoragePoolSetAutostart(
        ptr: sys::virStoragePoolPtr,
        autostart: libc::c_uint,
    ) -> libc::c_int;
    fn virStoragePoolGetInfo(
        ptr: sys::virStoragePoolPtr,
        info: sys::virStoragePoolInfoPtr,
    ) -> libc::c_int;
    fn virStoragePoolNumOfVolumes(ptr: sys::virStoragePoolPtr) -> libc::c_int;
}

pub type StoragePoolXMLFlags = self::libc::c_uint;
pub const VIR_STORAGE_POOL_XML_INACTIVE: StoragePoolXMLFlags = 1 << 0;

pub type StoragePoolCreateFlags = self::libc::c_uint;
pub const STORAGE_POOL_CREATE_NORMAL: StoragePoolCreateFlags = 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD: StoragePoolCreateFlags = 1 << 0;
pub const STORAGE_POOL_CREATE_WITH_BUILD_OVERWRITE: StoragePoolCreateFlags = 1 << 1;
pub const STORAGE_POOL_CREATE_WITH_BUILD_NO_OVERWRITE: StoragePoolCreateFlags = 1 << 2;

pub type StoragePoolState = self::libc::c_uint;
pub const VIR_STORAGE_POOL_INACTIVE: StoragePoolState = 0;
pub const VIR_STORAGE_POOL_BUILDING: StoragePoolState = 1;
pub const VIR_STORAGE_POOL_RUNNING: StoragePoolState = 2;
pub const VIR_STORAGE_POOL_DEGRADED: StoragePoolState = 3;
pub const VIR_STORAGE_POOL_INACCESSIBLE: StoragePoolState = 4;

#[derive(Clone, Debug)]
pub struct StoragePoolInfo {
    /// A `StoragePoolState` flags
    pub state: u32,
    /// Logical size bytes.
    pub capacity: u64,
    /// Current allocation bytes.
    pub allocation: u64,
    /// Remaining free space bytes.
    pub available: u64,
}

impl StoragePoolInfo {
    pub fn from_ptr(ptr: sys::virStoragePoolInfoPtr) -> StoragePoolInfo {
        unsafe {
            StoragePoolInfo {
                state: (*ptr).state as StoragePoolState,
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).allocation as u64,
                available: (*ptr).available as u64,
            }
        }
    }
}

/// Provides APIs for the management of storage pools.
///
/// See http://libvirt.org/html/libvirt-libvirt-storage.html
#[derive(Debug)]
pub struct StoragePool {
    ptr: Option<sys::virStoragePoolPtr>,
}

impl Drop for StoragePool {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for StoragePool, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl StoragePool {
    pub fn new(ptr: sys::virStoragePoolPtr) -> StoragePool {
        return StoragePool { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virStoragePoolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virStoragePoolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolDefineXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn create_xml(
        conn: &Connect,
        xml: &str,
        flags: StoragePoolCreateFlags,
    ) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolCreateXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(StoragePool::new(ptr));
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

    pub fn lookup_by_volume(vol: &StorageVol) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = virStoragePoolLookupByVolume(vol.as_ptr());
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
            let n = virStoragePoolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn num_of_volumes(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virStoragePoolNumOfVolumes(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virStoragePoolGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: StoragePoolXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virStoragePoolGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self, flags: StoragePoolCreateFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virStoragePoolCreate(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn build(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virStoragePoolBuild(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolDestroy(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if virStoragePoolDelete(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virStoragePoolFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virStoragePoolIsPersistent(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn refresh(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virStoragePoolRefresh(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }
    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut auto = 0;
            let ret = virStoragePoolGetAutostart(self.as_ptr(), &mut auto);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(auto == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<u32, Error> {
        unsafe {
            let ret = virStoragePoolSetAutostart(self.as_ptr(), autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_info(&self) -> Result<StoragePoolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStoragePoolInfo::default();
            let res = virStoragePoolGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(StoragePoolInfo::from_ptr(pinfo));
        }
    }
}
