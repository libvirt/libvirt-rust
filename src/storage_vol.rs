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
            capacity: (*ptr).capacity as u64,
            allocation: (*ptr).allocation as u64,
        }
    }
}

/// Provides APIs for the management of storage volumes.
///
/// See http://libvirt.org/html/libvirt-libvirt-storage.html
#[derive(Debug)]
pub struct StorageVol {
    ptr: Option<sys::virStorageVolPtr>,
}

impl Drop for StorageVol {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StorageVol: {}", e)
            }
        }
    }
}

impl StorageVol {
    pub fn new(ptr: sys::virStorageVolPtr) -> StorageVol {
        StorageVol { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virStorageVolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virStorageVolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn create_xml(
        pool: &StoragePool,
        xml: &str,
        flags: sys::virStorageVolCreateFlags,
    ) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolCreateXML(
                pool.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(StorageVol::new(ptr))
        }
    }

    pub fn create_xml_from(
        pool: &StoragePool,
        xml: &str,
        vol: &StorageVol,
        flags: sys::virStorageVolCreateFlags,
    ) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolCreateXMLFrom(
                pool.as_ptr(),
                string_to_c_chars!(xml),
                vol.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(StorageVol::new(ptr))
        }
    }

    pub fn lookup_by_name(pool: &StoragePool, name: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolLookupByName(pool.as_ptr(), string_to_c_chars!(name));
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(StorageVol::new(ptr))
        }
    }

    pub fn lookup_by_key(conn: &Connect, key: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolLookupByKey(conn.as_ptr(), string_to_c_chars!(key));
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(StorageVol::new(ptr))
        }
    }

    pub fn lookup_by_path(conn: &Connect, path: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolLookupByPath(conn.as_ptr(), string_to_c_chars!(path));
            if ptr.is_null() {
                return Err(Error::new());
            }
            Ok(StorageVol::new(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_key(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetKey(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetPath(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = sys::virStorageVolGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolDelete(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            Ok(())
        }
    }

    pub fn wipe(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolWipe(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            Ok(())
        }
    }

    pub fn wipe_pattern(
        &self,
        algo: sys::virStorageVolWipeAlgorithm,
        flags: u32,
    ) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolWipePattern(
                self.as_ptr(),
                algo as libc::c_uint,
                flags as libc::c_uint,
            ) == -1
            {
                return Err(Error::new());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn resize(&self, capacity: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStorageVolResize(
                self.as_ptr(),
                capacity as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret as u32)
        }
    }

    pub fn get_info(&self) -> Result<StorageVolInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let res = sys::virStorageVolGetInfo(self.as_ptr(), pinfo.as_mut_ptr());
            if res == -1 {
                return Err(Error::new());
            }
            Ok(StorageVolInfo::from_ptr(&mut pinfo.assume_init()))
        }
    }

    pub fn get_info_flags(&self, flags: u32) -> Result<StorageVolInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let res = sys::virStorageVolGetInfoFlags(
                self.as_ptr(),
                pinfo.as_mut_ptr(),
                flags as libc::c_uint,
            );
            if res == -1 {
                return Err(Error::new());
            }
            Ok(StorageVolInfo::from_ptr(&mut pinfo.assume_init()))
        }
    }

    pub fn download(
        &self,
        stream: &Stream,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStorageVolDownload(
                self.as_ptr(),
                stream.as_ptr(),
                offset as libc::c_ulonglong,
                length as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(())
        }
    }

    pub fn upload(
        &self,
        stream: &Stream,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStorageVolUpload(
                self.as_ptr(),
                stream.as_ptr(),
                offset as libc::c_ulonglong,
                length as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(())
        }
    }
}
