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

use std::str;

use connect::sys::virConnectPtr;

use connect::Connect;
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
    fn virNodeDeviceGetParent(ptr: sys::virNodeDevicePtr) -> *const libc::c_char;
    fn virNodeDeviceGetXMLDesc(ptr: sys::virNodeDevicePtr,
                               flags: libc::c_uint)
                               -> *mut libc::c_char;
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

#[derive(Debug)]
pub struct NodeDevice {
    ptr: Option<sys::virNodeDevicePtr>,
}

impl Drop for NodeDevice {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for NodeDevice, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl NodeDevice {
    pub fn new(ptr: sys::virNodeDevicePtr) -> NodeDevice {
        return NodeDevice { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virNodeDevicePtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice::new(ptr));
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceCreateXML(conn.as_ptr(),
                                             string_to_c_chars!(xml),
                                             flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNodeDeviceGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_parent(&self) -> Result<String, Error> {
        unsafe {
            let n = virNodeDeviceGetParent(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNodeDeviceGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virNodeDeviceGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceDestroy(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn num_of_devices(&self, cap: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let num = virNodeNumOfDevices(self.as_ptr(),
                                          string_to_c_chars!(cap),
                                          flags as libc::c_uint);
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }
}
