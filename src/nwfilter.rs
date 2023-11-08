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
use std::str;

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management for network filters.
///
/// See <https://libvirt.org/formatnwfilter.html>
#[derive(Debug)]
pub struct NWFilter {
    ptr: Option<sys::virNWFilterPtr>,
}

unsafe impl Send for NWFilter {}
unsafe impl Sync for NWFilter {}

impl Drop for NWFilter {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for NWFilter: {}", e)
            }
        }
    }
}

impl NWFilter {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virNWFilterPtr) -> NWFilter {
        NWFilter { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virNWFilterPtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<NWFilter, Error> {
        unsafe {
            let id_buf = CString::new(id).unwrap();
            let ptr = sys::virNWFilterLookupByName(conn.as_ptr(), id_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NWFilter::from_ptr(ptr))
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<NWFilter, Error> {
        unsafe {
            let uuid_buf = CString::new(uuid).unwrap();
            let ptr = sys::virNWFilterLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NWFilter::from_ptr(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virNWFilterGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virNWFilterGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(uuid.as_ptr(), nofree))
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = sys::virNWFilterGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<NWFilter, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virNWFilterDefineXML(conn.as_ptr(), xml_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(NWFilter::from_ptr(ptr))
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virNWFilterUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virNWFilterFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }
}
