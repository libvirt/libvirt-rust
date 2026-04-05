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

use libc::{c_char, c_uint};
use std::ffi::CString;
use std::{ptr, str};

use crate::error::Error;
use crate::util::{check_neg, check_null};

/// Provides APIs for the management of nodedevs.
///
/// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html>
#[derive(Debug)]
pub struct NodeDevice {
    ptr: sys::virNodeDevicePtr,
}

unsafe impl Send for NodeDevice {}
unsafe impl Sync for NodeDevice {}

impl Drop for NodeDevice {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virNodeDeviceFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on node device: {e}")
        }
    }
}

impl Clone for NodeDevice {
    /// Creates a copy of a node device.
    ///
    /// Increments the internal reference counter on the given
    /// device.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virNodeDeviceRef(self.as_ptr()) }) {
            panic!("Unable to add reference on node device: {e}")
        }
        unsafe { NodeDevice::from_ptr(self.as_ptr()) }
    }
}

impl NodeDevice {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virNodeDevicePtr) -> NodeDevice {
        NodeDevice { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virNodeDevicePtr {
        self.ptr
    }

    /// Returns the name of the node device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virNodeDeviceGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the name of the parent node device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceGetParent>
    pub fn parent(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virNodeDeviceGetParent(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the node device XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceGetXMLDesc>
    pub fn xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml =
            check_null!(unsafe { sys::virNodeDeviceGetXMLDesc(self.as_ptr(), flags as c_uint) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Remove the node device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceDestroy>
    pub fn destroy(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNodeDeviceDestroy(self.as_ptr()) })?;
        Ok(())
    }

    /// Detach the node device from the host kernel driver
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceDettach>
    pub fn detach(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNodeDeviceDettach(self.as_ptr()) })?;
        Ok(())
    }

    /// Perform a hardware reset on the node device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceReset>
    pub fn reset(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNodeDeviceReset(self.as_ptr()) })?;
        Ok(())
    }

    /// Re-attach the node device to the host kernel driver
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceReAttach>
    pub fn reattach(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virNodeDeviceReAttach(self.as_ptr()) })?;
        Ok(())
    }

    /// Detach the node device from the host kernel driver
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceDetachFlags>
    pub fn detach_flags(&self, driver: Option<&str>, flags: u32) -> Result<(), Error> {
        let driver_buf = some_string_to_cstring!(driver);
        let _ = check_neg!(unsafe {
            sys::virNodeDeviceDetachFlags(
                self.as_ptr(),
                some_cstring_to_c_chars!(driver_buf),
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returns the number of node device capability names
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceNumOfCaps>
    pub fn num_of_caps(&self) -> Result<u32, Error> {
        let num = check_neg!(unsafe { sys::virNodeDeviceNumOfCaps(self.as_ptr()) })?;
        Ok(num as u32)
    }

    /// List the node device capability names
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html#virNodeDeviceListCaps>
    #[allow(clippy::needless_range_loop)]
    pub fn list_caps(&self) -> Result<Vec<String>, Error> {
        let mut names: [*mut c_char; 1024] = [ptr::null_mut(); 1024];
        let size = check_neg!(unsafe {
            sys::virNodeDeviceListCaps(self.as_ptr(), names.as_mut_ptr(), 1024)
        })?;

        let mut array: Vec<String> = Vec::new();
        for x in 0..size as usize {
            array.push(unsafe { c_chars_to_string!(names[x]) });
        }
        Ok(array)
    }
}
