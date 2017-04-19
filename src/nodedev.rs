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

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virNodeDevice {
}

#[allow(non_camel_case_types)]
pub type virNodeDevicePtr = *mut virNodeDevice;

#[link(name = "virt")]
extern {
    fn virNodeDeviceLookupByName(c: virConnectPtr, id: *const libc::c_char) -> virNodeDevicePtr;
    fn virNodeDeviceCreateXML(c: virConnectPtr, xml: *const libc::c_char, flags: libc::c_uint) -> virNodeDevicePtr;
    fn virNodeDeviceDestroy(d: virNodeDevicePtr) -> libc::c_int;
    fn virNodeDeviceFree(d: virNodeDevicePtr) -> libc::c_int;
    fn virNodeDeviceGetName(d: virNodeDevicePtr) -> *const libc::c_char;
    fn virNodeDeviceGetXMLDesc(d: virNodeDevicePtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virNodeDeviceGetUUIDString(d: virNodeDevicePtr, uuid: *mut libc::c_char) -> libc::c_int;

    fn virNodeNumOfDevices(d: virNodeDevicePtr, cap: *const libc::c_char, flags: libc::c_uint) -> libc::c_int;

    // TODO: need to be implemented
    fn virNodeDeviceLookupSCSIHostByWWN() -> ();
    fn virNodeDeviceNumOfCaps() -> ();
    fn virNodeDeviceReset() -> ();
    fn virNodeDeviceListCaps() -> ();
    fn virNodeDeviceRef() -> ();
    fn virNodeDeviceDetachFlags() -> ();
    fn virNodeDeviceReAttach() -> ();
    fn virNodeDeviceGetParent() -> ();
    fn virNodeDeviceDettach() -> ();
}

pub type NodeDeviceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE:NodeDeviceXMLFlags = 1 << 0;

pub struct NodeDevice {
    pub d: virNodeDevicePtr
}

impl Drop for NodeDevice {
    fn drop(&mut self) {
        if !self.d.is_null() {
            self.free();
            return;
        }
    }
}

impl NodeDevice {

    pub fn as_ptr(&self) -> virNodeDevicePtr {
        self.d
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice{d: ptr});
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<NodeDevice, Error> {
        unsafe {
            let ptr = virNodeDeviceCreateXML(
                conn.as_ptr(), CString::new(xml).unwrap().as_ptr(),
                flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NodeDevice{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNodeDeviceGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNodeDeviceGetUUIDString(self.d, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virNodeDeviceGetXMLDesc(self.d, flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNodeDeviceFree(self.d) == -1 {
                return Err(Error::new());
            }
            self.d = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn num_of_devices(&self, cap: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let num = virNodeNumOfDevices(
                self.d,
                CString::new(cap).unwrap().as_ptr(),
                flags as libc::c_uint);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }
}
