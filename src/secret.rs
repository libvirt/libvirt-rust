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

use libc::{c_char, c_uchar, c_uint};

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;
use crate::util::{check_neg, check_null};

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
        if let Err(e) = check_neg!(unsafe { sys::virSecretFree(self.as_ptr()) }) {
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
        if let Err(e) = check_neg!(unsafe { sys::virSecretRef(self.as_ptr()) }) {
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

    pub fn connect(&self) -> Result<Connect, Error> {
        let ptr = check_null!(unsafe { sys::virSecretGetConnect(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virConnectRef(ptr) }) {
            panic!("Unable to add reference on connection: {e}")
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    /// Returns the secret usage ID string
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetUsageID>
    pub fn usage_id(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virSecretGetUsageID(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Returns the secret usage type
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetUsageType>
    pub fn usage_type(&self) -> Result<u32, Error> {
        let t = check_neg!(unsafe { sys::virSecretGetUsageType(self.as_ptr()) })?;
        Ok(t as u32)
    }

    /// Returns the secret UUID
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetUUID>
    pub fn uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [c_uchar; sys::VIR_UUID_BUFLEN as usize] = [0; sys::VIR_UUID_BUFLEN as usize];
        let _ = check_neg!(unsafe { sys::virSecretGetUUID(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(Uuid::from_bytes(uuid))
    }

    /// Returns the secret UUID string
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetUUIDString>
    pub fn uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let _ =
            check_neg!(unsafe { sys::virSecretGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Returns the secret XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetXMLDesc>
    pub fn xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = check_null!(unsafe { sys::virSecretGetXMLDesc(self.as_ptr(), flags) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Sets the secret data value
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretSetValue>
    pub fn set_value(&self, value: &[u8], flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virSecretSetValue(self.as_ptr(), value.as_ptr(), value.len(), flags)
        })?;
        Ok(())
    }

    /// Returns the secret data value
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretGetValue>
    pub fn value(&self, flags: u32) -> Result<Vec<u8>, Error> {
        let mut size: usize = 0;
        let n = check_null!(unsafe {
            sys::virSecretGetValue(self.as_ptr(), &mut size, flags as c_uint)
        })?;

        let mut array: Vec<u8> = Vec::new();
        for x in 0..size {
            array.push(unsafe { *n.add(x) })
        }
        Ok(array)
    }

    /// Removes the secret object configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-secret.html#virSecretUndefine>
    pub fn undefine(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virSecretUndefine(self.as_ptr()) })?;
        Ok(())
    }
}
