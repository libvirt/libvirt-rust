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
use std::{ptr, str};

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management of nodedevs.
///
/// See <https://libvirt.org/html/libvirt-libvirt-nodedev.html>
#[derive(Debug)]
pub struct NodeDevice {
    ptr: Option<sys::virNodeDevicePtr>,
}

unsafe impl Send for NodeDevice {}
unsafe impl Sync for NodeDevice {}

impl Drop for NodeDevice {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for NodeDevice: {}", e)
            }
        }
    }
}

impl Clone for NodeDevice {
    /// Creates a copy of a node device.
    ///
    /// Increments the internal reference counter on the given
    /// device. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: NodeDevice::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl NodeDevice {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virNodeDevicePtr) -> NodeDevice {
        NodeDevice { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<NodeDevice, Error> {
        unsafe {
            if sys::virNodeDeviceRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { NodeDevice::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virNodeDevicePtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NodeDevice, Error> {
        let id_buf = CString::new(id).unwrap();
        let ptr = unsafe { sys::virNodeDeviceLookupByName(conn.as_ptr(), id_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { NodeDevice::from_ptr(ptr) })
    }

    pub fn lookup_scsi_host_by_www(
        conn: &Connect,
        wwnn: &str,
        wwpn: &str,
        flags: u32,
    ) -> Result<NodeDevice, Error> {
        let wwnn_buf = CString::new(wwnn).unwrap();
        let wwpn_buf = CString::new(wwpn).unwrap();
        let ptr = unsafe {
            sys::virNodeDeviceLookupSCSIHostByWWN(
                conn.as_ptr(),
                wwnn_buf.as_ptr(),
                wwpn_buf.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { NodeDevice::from_ptr(ptr) })
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<NodeDevice, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virNodeDeviceCreateXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { NodeDevice::from_ptr(ptr) })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virNodeDeviceGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_parent(&self) -> Result<String, Error> {
        let n = unsafe { sys::virNodeDeviceGetParent(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = unsafe { sys::virNodeDeviceGetXMLDesc(self.as_ptr(), flags as libc::c_uint) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn destroy(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virNodeDeviceDestroy(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn detach(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virNodeDeviceDettach(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn reset(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virNodeDeviceReset(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn reattach(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virNodeDeviceReAttach(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn detach_flags(&self, driver: Option<&str>, flags: u32) -> Result<u32, Error> {
        let driver_buf = some_string_to_cstring!(driver);
        let ret = unsafe {
            sys::virNodeDeviceDetachFlags(
                self.as_ptr(),
                some_cstring_to_c_chars!(driver_buf),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virNodeDeviceFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn num_of_devices(conn: &Connect, cap: Option<&str>, flags: u32) -> Result<u32, Error> {
        let cap_buf = some_string_to_cstring!(cap);
        let num = unsafe {
            sys::virNodeNumOfDevices(
                conn.as_ptr(),
                some_cstring_to_c_chars!(cap_buf),
                flags as libc::c_uint,
            )
        };
        if num == -1 {
            return Err(Error::last_error());
        }
        Ok(num as u32)
    }

    pub fn num_of_caps(&self) -> Result<u32, Error> {
        let num = unsafe { sys::virNodeDeviceNumOfCaps(self.as_ptr()) };
        if num == -1 {
            return Err(Error::last_error());
        }
        Ok(num as u32)
    }

    #[allow(clippy::needless_range_loop)]
    pub fn list_caps(&self) -> Result<Vec<String>, Error> {
        let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
        let size = unsafe { sys::virNodeDeviceListCaps(self.as_ptr(), names.as_mut_ptr(), 1024) };
        if size == -1 {
            return Err(Error::last_error());
        }

        let mut array: Vec<String> = Vec::new();
        for x in 0..size as usize {
            array.push(unsafe { c_chars_to_string!(names[x]) });
        }
        Ok(array)
    }
}
