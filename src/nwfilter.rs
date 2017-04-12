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
use std::{str, mem};

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virNWFilter {
}

#[allow(non_camel_case_types)]
pub type virNWFilterPtr = *const virNWFilter;

#[link(name = "virt")]
extern {
    fn virNWFilterLookupByID(c: virConnectPtr, id: libc::c_int) -> virNWFilterPtr;
    fn virNWFilterLookupByName(c: virConnectPtr, id: *const libc::c_char) -> virNWFilterPtr;
    fn virNWFilterLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virNWFilterPtr;
    fn virNWFilterDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> virNWFilterPtr;
    fn virNWFilterUndefine(d: virNWFilterPtr) -> libc::c_int;
    fn virNWFilterFree(d: virNWFilterPtr) -> libc::c_int;
    fn virNWFilterGetName(d: virNWFilterPtr) -> *const libc::c_char;
    fn virNWFilterGetUUIDString(d: virNWFilterPtr, uuid: *const libc::c_char) -> libc::c_int;
    fn virNWFilterGetXMLDesc(d: virNWFilterPtr, flags: libc::c_uint) -> *const libc::c_char;
}

pub struct NWFilter {
    pub d: virNWFilterPtr
}

impl NWFilter {

    pub fn as_ptr(&self) -> virNWFilterPtr {
        self.d
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNWFilterGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let uuid: [libc::c_char; 37] = mem::uninitialized();
            if virNWFilterGetUUIDString(self.d, uuid.as_ptr()) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn get_xml_desc(&self, flags:u32) -> Result<String, Error> {
        unsafe {
            let xml = virNWFilterGetXMLDesc(self.d, flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterDefineXML(
                conn.as_ptr(), CString::new(xml).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter{d: ptr});
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virNWFilterUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virNWFilterFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
