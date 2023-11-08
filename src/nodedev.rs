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

impl NodeDevice {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virNodeDevicePtr) -> NodeDevice {
        NodeDevice { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virNodeDevicePtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NodeDevice, Error> {
        unsafe {
            let id_buf = CString::new(id).unwrap();
            let ptr = sys::virNodeDeviceLookupByName(conn.as_ptr(), id_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NodeDevice::from_ptr(ptr))
        }
    }

    pub fn lookup_scsi_host_by_www(
        conn: &Connect,
        wwnn: &str,
        wwpn: &str,
        flags: u32,
    ) -> Result<NodeDevice, Error> {
        unsafe {
            let wwnn_buf = CString::new(wwnn).unwrap();
            let wwpn_buf = CString::new(wwpn).unwrap();
            let ptr = sys::virNodeDeviceLookupSCSIHostByWWN(
                conn.as_ptr(),
                wwnn_buf.as_ptr(),
                wwpn_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NodeDevice::from_ptr(ptr))
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<NodeDevice, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr =
                sys::virNodeDeviceCreateXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NodeDevice::from_ptr(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virNodeDeviceGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_parent(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virNodeDeviceGetParent(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = sys::virNodeDeviceGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn destroy(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNodeDeviceDestroy(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn detach(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNodeDeviceDettach(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn reset(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNodeDeviceReset(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn reattach(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNodeDeviceReAttach(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn detach_flags(&self, driver: Option<&str>, flags: u32) -> Result<u32, Error> {
        unsafe {
            let driver_buf = some_string_to_cstring!(driver);
            let ret = sys::virNodeDeviceDetachFlags(
                self.as_ptr(),
                some_cstring_to_c_chars!(driver_buf),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virNodeDeviceFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn num_of_devices(conn: &Connect, cap: Option<&str>, flags: u32) -> Result<u32, Error> {
        unsafe {
            let cap_buf = some_string_to_cstring!(cap);
            let num = sys::virNodeNumOfDevices(
                conn.as_ptr(),
                some_cstring_to_c_chars!(cap_buf),
                flags as libc::c_uint,
            );
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    pub fn num_of_caps(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virNodeDeviceNumOfCaps(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn list_caps(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virNodeDeviceListCaps(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }
}
