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

extern crate libc;

use std::str;

use connect::sys::virConnectPtr;

use connect::Connect;
use error::Error;

pub mod sys {
    #[repr(C)]
    pub struct virNWFilter {}

    pub type virNWFilterPtr = *mut virNWFilter;
}

#[link(name = "virt")]
extern "C" {
    fn virNWFilterLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virNWFilterPtr;
    fn virNWFilterLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virNWFilterPtr;
    fn virNWFilterLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virNWFilterPtr;
    fn virNWFilterDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> sys::virNWFilterPtr;
    fn virNWFilterUndefine(ptr: sys::virNWFilterPtr) -> libc::c_int;
    fn virNWFilterFree(ptr: sys::virNWFilterPtr) -> libc::c_int;
    fn virNWFilterGetName(ptr: sys::virNWFilterPtr) -> *const libc::c_char;
    fn virNWFilterGetUUIDString(ptr: sys::virNWFilterPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virNWFilterGetXMLDesc(ptr: sys::virNWFilterPtr, flags: libc::c_uint) -> *mut libc::c_char;
}

/// Provides APIs for the management for network filters.
///
/// See http://libvirt.org/formatnwfilter.html
#[derive(Debug)]
pub struct NWFilter {
    ptr: Option<sys::virNWFilterPtr>,
}

impl Drop for NWFilter {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for NWFilter, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl NWFilter {
    pub fn new(ptr: sys::virNWFilterPtr) -> NWFilter {
        return NWFilter { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virNWFilterPtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNWFilterGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNWFilterGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virNWFilterGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<NWFilter, Error> {
        unsafe {
            let ptr = virNWFilterDefineXML(conn.as_ptr(), string_to_c_chars!(xml));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(NWFilter::new(ptr));
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virNWFilterUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNWFilterFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }
}
