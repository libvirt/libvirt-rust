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

use std::{str, ptr};

use connect::sys::virConnectPtr;

use connect::Connect;
use error::Error;

pub mod sys {
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virInterface {}

    #[allow(non_camel_case_types)]
    pub type virInterfacePtr = *mut virInterface;
}

#[link(name = "virt")]
extern "C" {
    fn virInterfaceLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virInterfacePtr;
    fn virInterfaceLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virInterfacePtr;
    fn virInterfaceLookupByMACString(c: virConnectPtr,
                                     id: *const libc::c_char)
                                     -> sys::virInterfacePtr;
    fn virInterfaceLookupByUUIDString(c: virConnectPtr,
                                      uuid: *const libc::c_char)
                                      -> sys::virInterfacePtr;
    fn virInterfaceDefineXML(c: virConnectPtr,
                             xml: *const libc::c_char,
                             flags: libc::c_uint)
                             -> sys::virInterfacePtr;
    fn virInterfaceCreate(ptr: sys::virInterfacePtr, flags: libc::c_uint) -> libc::c_int;
    fn virInterfaceDestroy(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceUndefine(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceFree(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceIsActive(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceGetName(ptr: sys::virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetMACString(ptr: sys::virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetXMLDesc(ptr: sys::virInterfacePtr,
                              flags: libc::c_uint)
                              -> *const libc::c_char;
    fn virInterfaceGetUUIDString(ptr: sys::virInterfacePtr,
                                 uuid: *mut libc::c_char)
                                 -> libc::c_int;
    fn virInterfaceGetConnect(ptr: sys::virInterfacePtr) -> virConnectPtr;
}

pub type InterfaceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE: InterfaceXMLFlags = 1 << 0;

pub struct Interface {
    ptr: sys::virInterfacePtr,
}

impl Drop for Interface {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for Interface")
            }
            return;
        }
    }
}

impl Interface {
    pub fn new(ptr: sys::virInterfacePtr) -> Interface {
        return Interface { ptr: ptr };
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virInterfaceGetConnect(self.ptr);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceDefineXML(conn.as_ptr(),
                                            string_to_c_chars!(xml),
                                            flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByMACString(conn.as_ptr(),
                                                    string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByUUIDString(conn.as_ptr(),
                                                     string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virInterfaceGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virInterfaceGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr()));
        }
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        unsafe {
            let mac = virInterfaceGetMACString(self.ptr);
            if mac.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(mac));
        }
    }

    pub fn get_xml_desc(&self, flags: InterfaceXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virInterfaceGetXMLDesc(self.ptr, flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self, flags: InterfaceXMLFlags) -> Result<(), Error> {
        unsafe {
            if virInterfaceCreate(self.ptr, flags) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceUndefine(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virInterfaceFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virInterfaceIsActive(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
