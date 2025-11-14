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

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management of secrets.
///
/// See <https://libvirt.org/html/libvirt-libvirt-secret.html>
#[derive(Debug)]
pub struct Secret {
    ptr: sys::virSecretPtr,
}

unsafe impl Send for Secret {}
unsafe impl Sync for Secret {}

impl Drop for Secret {
    fn drop(&mut self) {
        let ret = unsafe { sys::virSecretFree(self.as_ptr()) };
        if ret == -1 {
            let e = Error::last_error();
            panic!("Unable to drop reference on secret: {e}")
        }
    }
}

impl Clone for Secret {
    /// Creates a copy of a secret.
    ///
    /// Increments the internal reference counter on the given
    /// secret.
    fn clone(&self) -> Self {
        let ret = unsafe { sys::virSecretRef(self.as_ptr()) };
        if ret == -1 {
            let e = Error::last_error();
            panic!("Unable to add reference on secret: {e}")
        }

        unsafe { Secret::from_ptr(self.as_ptr()) }
    }
}

impl Secret {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virSecretPtr) -> Secret {
        Secret { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virSecretPtr {
        self.ptr
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virSecretGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
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
}
