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

use crate::error::Error;
use crate::util::{check_neg, check_null};

/// Provides APIs for the management for network filters.
///
/// See <https://libvirt.org/formatnwfilter.html>
#[derive(Debug)]
pub struct NWFilter {
    ptr: sys::virNWFilterPtr,
}

unsafe impl Send for NWFilter {}
unsafe impl Sync for NWFilter {}

impl Drop for NWFilter {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virNWFilterFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on network filter: {e}")
        }
    }
}

impl Clone for NWFilter {
    /// Creates a copy of a network filter.
    ///
    /// Increments the internal reference counter on the given
    /// filter.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virNWFilterRef(self.as_ptr()) }) {
            panic!("Unable to add reference on network filter: {e}")
        }
        unsafe { NWFilter::from_ptr(self.as_ptr()) }
    }
}

impl NWFilter {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virNWFilterPtr) -> NWFilter {
        NWFilter { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virNWFilterPtr {
        self.ptr
    }

    /// Returns the network filter name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nwfilter.html#virNWFilterGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virNWFilterGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the network filter UUID
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nwfilter.html#virNWFilterGetUUID>
    pub fn uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [c_uchar; sys::VIR_UUID_BUFLEN as usize] = [0; sys::VIR_UUID_BUFLEN as usize];
        let _ = check_neg!(unsafe { sys::virNWFilterGetUUID(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(Uuid::from_bytes(uuid))
    }

    /// Returns the network filter UUID string
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nwfilter.html#virNWFilterGetUUIDString>
    pub fn uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let _ =
            check_neg!(unsafe { sys::virNWFilterGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Returns the network filter XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nwfilter.html#virNWFilterGetXMLDesc>
    pub fn xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml =
            check_null!(unsafe { sys::virNWFilterGetXMLDesc(self.as_ptr(), flags as c_uint) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Removes the network filter configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nwfilter.html#virNWFilterUndefine>
    pub fn undefine(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNWFilterUndefine(self.as_ptr()) })?;
        Ok(())
    }
}
