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

use libc::{c_char, c_int, c_uchar};
use std::ffi::CString;
use std::str;

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;
use crate::util::{check_neg, check_null};

/// Provides APIs for the management of networks.
///
/// See <https://libvirt.org/html/libvirt-libvirt-network.html>
#[derive(Debug)]
pub struct Network {
    ptr: sys::virNetworkPtr,
}

unsafe impl Send for Network {}
unsafe impl Sync for Network {}

impl Drop for Network {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virNetworkFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on network: {e}")
        }
    }
}

impl Clone for Network {
    /// Creates a copy of a network.
    ///
    /// Increments the internal reference counter on the given
    /// network.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virNetworkRef(self.as_ptr()) }) {
            panic!("Unable to add reference on network: {e}")
        }
        unsafe { Network::from_ptr(self.as_ptr()) }
    }
}

impl Network {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virNetworkPtr) -> Network {
        Network { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virNetworkPtr {
        self.ptr
    }

    pub fn connect(&self) -> Result<Connect, Error> {
        let ptr = check_null!(unsafe { sys::virNetworkGetConnect(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virConnectRef(ptr) }) {
            panic!("Unable to add reference on connection: {e}")
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    /// Returns the network name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virNetworkGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the network UUID
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetUUID>
    pub fn uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [c_uchar; sys::VIR_UUID_BUFLEN as usize] = [0; sys::VIR_UUID_BUFLEN as usize];
        let _ = check_neg!(unsafe { sys::virNetworkGetUUID(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(Uuid::from_bytes(uuid))
    }

    /// Returns the network UUID string
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetUUIDString>
    pub fn uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let _ =
            check_neg!(unsafe { sys::virNetworkGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Returns the network bridge name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetBridgeName>
    pub fn bridge_name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virNetworkGetBridgeName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Returns the network XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetXMLDesc>
    pub fn xml_desc(&self, flags: sys::virNetworkXMLFlags) -> Result<String, Error> {
        let xml = check_null!(unsafe { sys::virNetworkGetXMLDesc(self.as_ptr(), flags) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Starts an inactive network
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkCreate>
    pub fn create(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNetworkCreate(self.as_ptr()) })?;
        Ok(())
    }

    /// Stops an active network
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkDestroy>
    pub fn destroy(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNetworkDestroy(self.as_ptr()) })?;
        Ok(())
    }

    /// Removes the network configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkUndefine>
    pub fn undefine(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNetworkUndefine(self.as_ptr()) })?;
        Ok(())
    }

    /// Determines if the network is active
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkIsActive>
    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virNetworkIsActive(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Determines if the network has a persistent configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkIsPersistent>
    pub fn is_persistent(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virNetworkIsPersistent(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Returns the network autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkGetAutostart>
    pub fn autostart(&self) -> Result<bool, Error> {
        let mut auto = 0;
        let _ = check_neg!(unsafe { sys::virNetworkGetAutostart(self.as_ptr(), &mut auto) })?;
        Ok(auto == 1)
    }

    /// Updates the network autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkSetAutostart>
    pub fn set_autostart(&self, autostart: bool) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virNetworkSetAutostart(self.as_ptr(), autostart as c_int) })?;
        Ok(())
    }

    /// Updates the network configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-network.html#virNetworkUpdate>
    pub fn update(
        &self,
        cmd: sys::virNetworkUpdateCommand,
        section: sys::virNetworkUpdateSection,
        index: i32,
        xml: &str,
        flags: sys::virNetworkUpdateFlags,
    ) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe {
            sys::virNetworkUpdate(
                self.as_ptr(),
                cmd,
                section,
                index as c_int,
                xml_buf.as_ptr(),
                flags,
            )
        })?;
        Ok(())
    }
}
