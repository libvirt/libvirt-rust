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

use libc::{c_char, c_int, c_uchar, c_uint, c_void};
use std::ffi::CString;
use std::{mem, ptr};

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;
use crate::storage_vol::StorageVol;
use crate::util::{check_neg, check_null};

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
    ptr: sys::virStoragePoolPtr,
}

unsafe impl Send for StoragePool {}
unsafe impl Sync for StoragePool {}

impl Drop for StoragePool {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virStoragePoolFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on storage pool: {e}")
        }
    }
}

impl Clone for StoragePool {
    /// Creates a copy of a storage pool.
    ///
    /// Increments the internal reference counter on the given
    /// pool.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virStoragePoolRef(self.as_ptr()) }) {
            panic!("Unable to add reference on storage pool: {e}")
        }
        unsafe { StoragePool::from_ptr(self.as_ptr()) }
    }
}

impl StoragePool {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virStoragePoolPtr) -> StoragePool {
        StoragePool { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virStoragePoolPtr {
        self.ptr
    }

    pub fn connect(&self) -> Result<Connect, Error> {
        let ptr = check_null!(unsafe { sys::virStoragePoolGetConnect(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virConnectRef(ptr) }) {
            panic!("Unable to add reference on connection: {e}")
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    /// Returns the storage volume with the requested name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStorageVolLookupByName>
    pub fn lookup_storage_vol_by_name(&self, name: &str) -> Result<StorageVol, Error> {
        let name_buf = CString::new(name)?;
        let ptr = check_null!(unsafe {
            sys::virStorageVolLookupByName(self.as_ptr(), name_buf.as_ptr())
        })?;
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    /// Returns the storage pool name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virStoragePoolGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the number of storage volumes
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolNumOfVolumes>
    pub fn num_of_volumes(&self) -> Result<u32, Error> {
        let ret = check_neg!(unsafe { sys::virStoragePoolNumOfVolumes(self.as_ptr()) })?;
        Ok(ret as u32)
    }

    /// Returns a list of storage volume names
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolListVolumes>
    #[allow(clippy::needless_range_loop)]
    pub fn list_volumes(&self) -> Result<Vec<String>, Error> {
        let mut names: [*mut c_char; 1024] = [ptr::null_mut(); 1024];
        let size = check_neg!(unsafe {
            sys::virStoragePoolListVolumes(self.as_ptr(), names.as_mut_ptr(), 1024)
        })?;

        let mut array: Vec<String> = Vec::new();
        for x in 0..size as usize {
            array.push(unsafe { c_chars_to_string!(names[x]) });
        }
        Ok(array)
    }

    /// Returns a list of storage volume objects
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolListAllVolumes>
    pub fn list_all_volumes(&self, flags: u32) -> Result<Vec<StorageVol>, Error> {
        let mut volumes: *mut sys::virStorageVolPtr = ptr::null_mut();
        let size = check_neg!(unsafe {
            sys::virStoragePoolListAllVolumes(self.as_ptr(), &mut volumes, flags as c_uint)
        })?;

        let mut array: Vec<StorageVol> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { StorageVol::from_ptr(*volumes.offset(x)) });
        }
        unsafe { libc::free(volumes as *mut c_void) };

        Ok(array)
    }

    /// Returns the storage pool UUID
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetUUID>
    pub fn uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [c_uchar; sys::VIR_UUID_BUFLEN as usize] = [0; sys::VIR_UUID_BUFLEN as usize];
        let _ =
            check_neg!(unsafe { sys::virStoragePoolGetUUID(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(Uuid::from_bytes(uuid))
    }

    /// Returns the storage pool UUID string
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetUUIDString>
    pub fn uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let _ = check_neg!(unsafe {
            sys::virStoragePoolGetUUIDString(self.as_ptr(), uuid.as_mut_ptr())
        })?;
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Returns the storage pool XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetXMLDesc>
    pub fn xml_desc(&self, flags: sys::virStorageXMLFlags) -> Result<String, Error> {
        let xml = check_null!(unsafe { sys::virStoragePoolGetXMLDesc(self.as_ptr(), flags) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Start the storage pool
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolCreate>
    pub fn create(&self, flags: sys::virStoragePoolCreateFlags) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolCreate(self.as_ptr(), flags) })?;
        Ok(())
    }

    /// Formats the storage pool data
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolBuild>
    pub fn build(&self, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolBuild(self.as_ptr(), flags) })?;
        Ok(())
    }

    /// Stop the storage pool
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolDestroy>
    pub fn destroy(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolDestroy(self.as_ptr()) })?;
        Ok(())
    }

    /// Delete the storage pool data
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolDelete>
    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolDelete(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Remove the storage pool configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolUndefine>
    pub fn undefine(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolUndefine(self.as_ptr()) })?;
        Ok(())
    }

    /// Determine whether the storage pool is active
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolIsActive>
    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virStoragePoolIsActive(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Determine whether the storage pool has a persistent configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolIsPersistent>
    pub fn is_persistent(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virStoragePoolIsPersistent(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Refreshes the volumes in a storage pool
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolRefresh>
    pub fn refresh(&self, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virStoragePoolRefresh(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Returns the storage pool autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetAutostart>
    pub fn autostart(&self) -> Result<bool, Error> {
        let mut auto = 0;
        let _ = check_neg!(unsafe { sys::virStoragePoolGetAutostart(self.as_ptr(), &mut auto) })?;
        Ok(auto == 1)
    }

    /// Updates the storage pool autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolSetAutostart>
    pub fn set_autostart(&self, autostart: bool) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virStoragePoolSetAutostart(self.as_ptr(), autostart as c_int)
        })?;
        Ok(())
    }

    /// Returns the storage pool information
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-storage.html#virStoragePoolGetInfo>
    pub fn info(&self) -> Result<StoragePoolInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let _ =
            check_neg!(unsafe { sys::virStoragePoolGetInfo(self.as_ptr(), pinfo.as_mut_ptr()) })?;
        Ok(unsafe { StoragePoolInfo::from_ptr(&mut pinfo.assume_init()) })
    }
}
