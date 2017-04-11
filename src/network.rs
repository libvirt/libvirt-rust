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
pub struct virNetwork {
}

#[allow(non_camel_case_types)]
pub type virNetworkPtr = *const virNetwork;

#[link(name = "virt")]
extern {
    fn virNetworkLookupByID(c: virConnectPtr, id: libc::c_int) -> virNetworkPtr;
    fn virNetworkLookupByName(c: virConnectPtr, id: *const libc::c_char) -> virNetworkPtr;
    fn virNetworkLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virNetworkPtr;
    fn virNetworkCreate(d: virNetworkPtr, flags: libc::c_uint) -> libc::c_int;
    fn virNetworkDestroy(d: virNetworkPtr) -> libc::c_int;
    fn virNetworkUndefine(d: virNetworkPtr) -> libc::c_int;
    fn virNetworkFree(d: virNetworkPtr) -> libc::c_int;
    fn virNetworkIsActive(d: virNetworkPtr) -> libc::c_int;
    fn virNetworkGetName(d: virNetworkPtr) -> *const libc::c_char;
    fn virNetworkGetXMLDesc(d: virNetworkPtr, flags: libc::c_uint) -> *const libc::c_char;
}

pub type NetworkXMLFlags = self::libc::c_uint;
pub const VIR_NETWORK_XML_INACTIVE:NetworkXMLFlags = 1;

pub struct Network {
    pub d: virNetworkPtr
}

impl Network {

    pub fn as_ptr(&self) -> virNetworkPtr {
        self.d
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<&str, Error> {
        unsafe {
            let n = virNetworkGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }

    pub fn get_xml_desc(&self, flags:NetworkXMLFlags) -> Result<&str, Error> {
        unsafe {
            let xml = virNetworkGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(xml).to_bytes()).unwrap())
        }
    }

    pub fn create(&self, flags: NetworkXMLFlags) -> Result<(), Error> {
        unsafe {
            if virNetworkCreate(self.d, flags) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkIsActive(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
