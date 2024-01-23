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

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management of secrets.
///
/// See <https://libvirt.org/html/libvirt-libvirt-secret.html>
#[derive(Debug)]
pub struct Secret {
    ptr: Option<sys::virSecretPtr>,
}

unsafe impl Send for Secret {}
unsafe impl Sync for Secret {}

impl Drop for Secret {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Secret: {}", e)
            }
        }
    }
}

impl Clone for Secret {
    /// Creates a copy of a secret.
    ///
    /// Increments the internal reference counter on the given
    /// secret. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: Secret::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl Secret {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virSecretPtr) -> Secret {
        Secret { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<Secret, Error> {
        unsafe {
            if sys::virSecretRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { Secret::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virSecretPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virSecretGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Secret, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virSecretDefineXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Secret::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid(conn: &Connect, uuid: Uuid) -> Result<Secret, Error> {
        let ptr = unsafe { sys::virSecretLookupByUUID(conn.as_ptr(), uuid.as_bytes().as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Secret::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Secret, Error> {
        let uuid_buf = CString::new(uuid).unwrap();
        let ptr = unsafe { sys::virSecretLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Secret::from_ptr(ptr) })
    }

    pub fn lookup_by_usage(conn: &Connect, usagetype: i32, usageid: &str) -> Result<Secret, Error> {
        let usageid_buf = CString::new(usageid).unwrap();
        let ptr = unsafe {
            sys::virSecretLookupByUsage(
                conn.as_ptr(),
                usagetype as libc::c_int,
                usageid_buf.as_ptr(),
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Secret::from_ptr(ptr) })
    }

    pub fn get_usage_id(&self) -> Result<String, Error> {
        let n = unsafe { sys::virSecretGetUsageID(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    pub fn get_usage_type(&self) -> Result<u32, Error> {
        let t = unsafe { sys::virSecretGetUsageType(self.as_ptr()) };
        if t == -1 {
            return Err(Error::last_error());
        }
        Ok(t as u32)
    }

    pub fn get_uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [libc::c_uchar; sys::VIR_UUID_BUFLEN as usize] =
            [0; sys::VIR_UUID_BUFLEN as usize];
        let ret = unsafe { sys::virSecretGetUUID(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(Uuid::from_bytes(uuid))
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [libc::c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let ret = unsafe { sys::virSecretGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = unsafe { sys::virSecretGetXMLDesc(self.as_ptr(), flags) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn set_value(&self, value: &[u8], flags: u32) -> Result<(), Error> {
        let ret =
            unsafe { sys::virSecretSetValue(self.as_ptr(), value.as_ptr(), value.len(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn get_value(&self, flags: u32) -> Result<Vec<u8>, Error> {
        let mut size: usize = 0;
        let n = unsafe { sys::virSecretGetValue(self.as_ptr(), &mut size, flags as libc::c_uint) };
        if n.is_null() {
            return Err(Error::last_error());
        }

        let mut array: Vec<u8> = Vec::new();
        for x in 0..size {
            array.push(unsafe { *n.add(x) })
        }
        Ok(array)
    }

    pub fn undefine(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virSecretUndefine(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virSecretFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }
}
