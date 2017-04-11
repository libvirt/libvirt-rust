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
use std::str;

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virInterface {
}

#[allow(non_camel_case_types)]
pub type virInterfacePtr = *const virInterface;

#[link(name = "virt")]
extern {
    fn virInterfaceLookupByID(c: virConnectPtr,
                              id: libc::c_int) -> virInterfacePtr;
    fn virInterfaceLookupByName(c: virConnectPtr,id: *const libc::c_char) -> virInterfacePtr;
    fn virInterfaceLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virInterfacePtr;
    fn virInterfaceCreate(d: virInterfacePtr, flags: libc::c_uint) -> libc::c_int;
    fn virInterfaceDestroy(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceUndefine(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceFree(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceIsActive(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceGetName(d: virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetMACString(d: virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetXMLDesc(d: virInterfacePtr, flags: libc::c_uint) -> *const libc::c_char;
}

pub type InterfaceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE:InterfaceXMLFlags = 1;

pub struct Interface {
    pub d: virInterfacePtr
}

impl Interface {

    pub fn as_ptr(&self) -> virInterfacePtr {
        self.d
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<&str, Error> {
        unsafe {
            let n = virInterfaceGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }

    pub fn get_mac_string(&self) -> Result<&str, Error> {
        unsafe {
            let mac = virInterfaceGetMACString(self.d);
            if mac.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(mac).to_bytes()).unwrap())
        }
    }

    pub fn get_xml_desc(&self, flags:InterfaceXMLFlags) -> Result<&str, Error> {
        unsafe {
            let xml = virInterfaceGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(xml).to_bytes()).unwrap())
        }
    }

    pub fn create(&self, flags: InterfaceXMLFlags) -> Result<(), Error> {
        unsafe {
            if virInterfaceCreate(self.d, flags) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceIsActive(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
