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

#![allow(improper_ctypes)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::{str, ptr};

use connect::{Connect, virConnectPtr};
use error::Error;

pub mod sys {
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virNodeDevice {}

    #[allow(non_camel_case_types)]
    pub type virNodeDevicePtr = *mut virNodeDevice;
}

#[link(name = "virt")]
extern "C" {
    fn virNodeDeviceLookupByName(c: virConnectPtr,
                                 id: *const libc::c_char)
                                 -> sys::virNodeDevicePtr;
    fn virNodeDeviceCreateXML(c: virConnectPtr,
                              xml: *const libc::c_char,
                              flags: libc::c_uint)
                              -> sys::virNodeDevicePtr;
    fn virNodeDeviceDestroy(ptr: sys::virNodeDevicePtr) -> libc::c_int;
    fn virNodeDeviceFree(ptr: sys::virNodeDevicePtr) -> libc::c_int;
    fn virNodeDeviceGetName(ptr: sys::virNodeDevicePtr) -> *const libc::c_char;
    fn virNodeDeviceGetXMLDesc(ptr: sys::virNodeDevicePtr,
                               flags: libc::c_uint)
                               -> *const libc::c_char;
    fn virNodeDeviceGetUUIDString(ptr: sys::virNodeDevicePtr,
                                  uuid: *mut libc::c_char)
                                  -> libc::c_int;

    fn virNodeNumOfDevices(ptr: sys::virNodeDevicePtr,
                           cap: *const libc::c_char,
                           flags: libc::c_uint)
                           -> libc::c_int;
}

pub type NodeDeviceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE: NodeDeviceXMLFlags = 1 << 0;

pub struct NodeDevice {
    pub ptr: sys::virNodeDevicePtr,
}

impl Drop for NodeDevice {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for NodeDevice")
            }
            return;
        }
    }
}

impl NodeDevice {
    pub fn new(ptr: sys::virNodeDevicePtr) -> NodeDevice {
        return NodeDevice { ptr: ptr };
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceLookupByName(conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice::new(ptr));
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceCreateXML(conn.as_ptr(),
                                             CString::new(xml).unwrap().as_ptr(),
                                             flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNodeDeviceGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned());
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNodeDeviceGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(uuid.as_ptr())
                          .to_string_lossy()
                          .into_owned());
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virNodeDeviceGetXMLDesc(self.ptr, flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn num_of_devices(&self, cap: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let num = virNodeNumOfDevices(self.ptr,
                                          CString::new(cap).unwrap().as_ptr(),
                                          flags as libc::c_uint);
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }
}
