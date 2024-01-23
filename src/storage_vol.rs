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
use std::{mem, str};

use crate::connect::Connect;
use crate::error::Error;
use crate::storage_pool::StoragePool;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub struct StorageVolInfo {
    /// See: `virStorageVolType` flags
    pub kind: u32,
    /// Logical size bytes.
    pub capacity: u64,
    /// Current allocation bytes
    pub allocation: u64,
}

impl StorageVolInfo {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virStorageVolInfoPtr) -> StorageVolInfo {
        StorageVolInfo {
            kind: (*ptr).type_ as sys::virStorageVolType,
            capacity: (*ptr).capacity,
            allocation: (*ptr).allocation,
        }
    }
}

/// Provides APIs for the management of storage volumes.
///
/// See <https://libvirt.org/html/libvirt-libvirt-storage.html>
#[derive(Debug)]
pub struct StorageVol {
    ptr: Option<sys::virStorageVolPtr>,
}

unsafe impl Send for StorageVol {}
unsafe impl Sync for StorageVol {}

impl Drop for StorageVol {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StorageVol: {}", e)
            }
        }
    }
}

impl Clone for StorageVol {
    /// Creates a copy of a storage pool.
    ///
    /// Increments the internal reference counter on the given
    /// volume. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: StorageVol::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl StorageVol {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virStorageVolPtr) -> StorageVol {
        StorageVol { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<StorageVol, Error> {
        unsafe {
            if sys::virStorageVolRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { StorageVol::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virStorageVolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virStorageVolGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn create_xml(
        pool: &StoragePool,
        xml: &str,
        flags: sys::virStorageVolCreateFlags,
    ) -> Result<StorageVol, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virStorageVolCreateXML(pool.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    pub fn create_xml_from(
        pool: &StoragePool,
        xml: &str,
        vol: &StorageVol,
        flags: sys::virStorageVolCreateFlags,
    ) -> Result<StorageVol, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virStorageVolCreateXMLFrom(
                pool.as_ptr(),
                xml_buf.as_ptr(),
                vol.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    pub fn lookup_by_name(pool: &StoragePool, name: &str) -> Result<StorageVol, Error> {
        let name_buf = CString::new(name).unwrap();
        let ptr = unsafe { sys::virStorageVolLookupByName(pool.as_ptr(), name_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    pub fn lookup_by_key(conn: &Connect, key: &str) -> Result<StorageVol, Error> {
        let key_buf = CString::new(key).unwrap();
        let ptr = unsafe { sys::virStorageVolLookupByKey(conn.as_ptr(), key_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    pub fn lookup_by_path(conn: &Connect, path: &str) -> Result<StorageVol, Error> {
        let path_buf = CString::new(path).unwrap();
        let ptr = unsafe { sys::virStorageVolLookupByPath(conn.as_ptr(), path_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVol::from_ptr(ptr) })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virStorageVolGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_key(&self) -> Result<String, Error> {
        let n = unsafe { sys::virStorageVolGetKey(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_path(&self) -> Result<String, Error> {
        let n = unsafe { sys::virStorageVolGetPath(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = unsafe { sys::virStorageVolGetXMLDesc(self.as_ptr(), flags) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        let ret = unsafe { sys::virStorageVolDelete(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn wipe(&self, flags: u32) -> Result<(), Error> {
        let ret = unsafe { sys::virStorageVolWipe(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn wipe_pattern(
        &self,
        algo: sys::virStorageVolWipeAlgorithm,
        flags: u32,
    ) -> Result<(), Error> {
        let ret = unsafe {
            sys::virStorageVolWipePattern(
                self.as_ptr(),
                algo as libc::c_uint,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virStorageVolFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn resize(&self, capacity: u64, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virStorageVolResize(
                self.as_ptr(),
                capacity as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn get_info(&self) -> Result<StorageVolInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let res = unsafe { sys::virStorageVolGetInfo(self.as_ptr(), pinfo.as_mut_ptr()) };
        if res == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVolInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    pub fn get_info_flags(&self, flags: u32) -> Result<StorageVolInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let res = unsafe {
            sys::virStorageVolGetInfoFlags(self.as_ptr(), pinfo.as_mut_ptr(), flags as libc::c_uint)
        };
        if res == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { StorageVolInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    pub fn download(
        &self,
        stream: &Stream,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        let ret = unsafe {
            sys::virStorageVolDownload(
                self.as_ptr(),
                stream.as_ptr(),
                offset as libc::c_ulonglong,
                length as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn upload(
        &self,
        stream: &Stream,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        let ret = unsafe {
            sys::virStorageVolUpload(
                self.as_ptr(),
                stream.as_ptr(),
                offset as libc::c_ulonglong,
                length as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }
}
