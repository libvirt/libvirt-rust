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
 * <https://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

use std::ffi::CString;
use std::{mem, ptr, str};

use crate::connect::Connect;
use crate::error::Error;
use crate::storage_vol::StorageVol;

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
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virStoragePoolInfoPtr) -> StoragePoolInfo {
        StoragePoolInfo {
            state: (*ptr).state as sys::virStoragePoolState,
            capacity: (*ptr).capacity,
            allocation: (*ptr).allocation,
            available: (*ptr).available,
        }
    }
}

/// Provides APIs for the management of storage pools.
///
/// See <https://libvirt.org/html/libvirt-libvirt-storage.html>
#[derive(Debug)]
pub struct StoragePool {
    ptr: Option<sys::virStoragePoolPtr>,
}

unsafe impl Send for StoragePool {}
unsafe impl Sync for StoragePool {}

impl Drop for StoragePool {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StoragePool: {}", e)
            }
        }
    }
}

impl StoragePool {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virStoragePoolPtr) -> StoragePool {
        StoragePool { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virStoragePoolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virStoragePoolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::from_ptr(ptr))
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<StoragePool, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virStoragePoolDefineXML(
                conn.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(StoragePool::from_ptr(ptr))
        }
    }

    pub fn create_xml(
        conn: &Connect,
        xml: &str,
        flags: sys::virStoragePoolCreateFlags,
    ) -> Result<StoragePool, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virStoragePoolCreateXML(
                conn.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(StoragePool::from_ptr(ptr))
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<StoragePool, Error> {
        unsafe {
            let id_buf = CString::new(id).unwrap();
            let ptr = sys::virStoragePoolLookupByName(conn.as_ptr(), id_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(StoragePool::from_ptr(ptr))
        }
    }

    pub fn lookup_by_volume(vol: &StorageVol) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolLookupByVolume(vol.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(StoragePool::from_ptr(ptr))
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<StoragePool, Error> {
        unsafe {
            let uuid_buf = CString::new(uuid).unwrap();
            let ptr = sys::virStoragePoolLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(StoragePool::from_ptr(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStoragePoolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn num_of_volumes(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolNumOfVolumes(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn list_volumes(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virStoragePoolListVolumes(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    pub fn list_all_volumes(&self, flags: u32) -> Result<Vec<StorageVol>, Error> {
        unsafe {
            let mut volumes: *mut sys::virStorageVolPtr = ptr::null_mut();
            let size = sys::virStoragePoolListAllVolumes(
                self.as_ptr(),
                &mut volumes,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<StorageVol> = Vec::new();
            for x in 0..size as isize {
                array.push(StorageVol::from_ptr(*volumes.offset(x)));
            }
            libc::free(volumes as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virStoragePoolGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(uuid.as_ptr(), nofree))
        }
    }

    pub fn get_xml_desc(&self, flags: sys::virStorageXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virStoragePoolGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn create(&self, flags: sys::virStoragePoolCreateFlags) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolCreate(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn build(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolBuild(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolDestroy(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolDelete(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virStoragePoolIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virStoragePoolIsPersistent(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn refresh(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolRefresh(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }
    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut auto = 0;
            let ret = sys::virStoragePoolGetAutostart(self.as_ptr(), &mut auto);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(auto == 1)
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolSetAutostart(self.as_ptr(), autostart as libc::c_int);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn get_info(&self) -> Result<StoragePoolInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let res = sys::virStoragePoolGetInfo(self.as_ptr(), pinfo.as_mut_ptr());
            if res == -1 {
                return Err(Error::last_error());
            }
            Ok(StoragePoolInfo::from_ptr(&mut pinfo.assume_init()))
        }
    }
}
