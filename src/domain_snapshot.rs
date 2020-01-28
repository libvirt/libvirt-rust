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

use std::{mem, ptr, str};

use connect::sys::virConnectPtr;
use domain::sys::virDomainPtr;

use connect::Connect;
use domain::Domain;
use error::Error;

pub mod sys {
    #[repr(C)]
    pub struct virDomainSnapshot {}

    pub type virDomainSnapshotPtr = *mut virDomainSnapshot;
}

#[link(name = "virt")]
extern "C" {
    fn virDomainSnapshotGetName(ptr: sys::virDomainSnapshotPtr) -> *const libc::c_char;
    fn virDomainSnapshotGetDomain(ptr: sys::virDomainSnapshotPtr) -> virDomainPtr;
    fn virDomainSnapshotGetConnect(ptr: sys::virDomainSnapshotPtr) -> virConnectPtr;
    fn virDomainSnapshotGetXMLDesc(
        ptr: sys::virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virDomainSnapshotDelete(ptr: sys::virDomainSnapshotPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainSnapshotIsCurrent(
        ptr: sys::virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSnapshotHasMetadata(
        ptr: sys::virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSnapshotCreateXML(
        d: virDomainPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virDomainSnapshotPtr;
    fn virDomainSnapshotFree(ptr: sys::virDomainSnapshotPtr) -> libc::c_int;
    fn virDomainSnapshotCurrent(d: virDomainPtr, flags: libc::c_uint) -> sys::virDomainSnapshotPtr;
    fn virDomainSnapshotGetParent(
        d: virDomainPtr,
        flags: libc::c_uint,
    ) -> sys::virDomainSnapshotPtr;
    fn virDomainSnapshotLookupByName(
        d: virDomainPtr,
        name: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virDomainSnapshotPtr;
    fn virDomainSnapshotNum(d: virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainSnapshotNumChildren(
        ptr: sys::virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSnapshotListAllChildren(
        ptr: sys::virDomainSnapshotPtr,
        snaps: *mut *mut sys::virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

/// Provides APIs for the management of domain snapshots.
///
/// See https://libvirt.org/formatsnapshot.html
#[derive(Debug)]
pub struct DomainSnapshot {
    ptr: Option<sys::virDomainSnapshotPtr>,
}

impl Drop for DomainSnapshot {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for DomainSnapshot, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl DomainSnapshot {
    pub fn new(ptr: sys::virDomainSnapshotPtr) -> DomainSnapshot {
        return DomainSnapshot { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virDomainSnapshotPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virDomainSnapshotGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn get_domain(&self) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainSnapshotGetDomain(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainSnapshotGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    /// Get a handle to a named snapshot.
    pub fn lookup_by_name(dom: &Domain, name: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = virDomainSnapshotLookupByName(
                dom.as_ptr(),
                string_to_c_chars!(name),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(DomainSnapshot::new(ptr));
        }
    }

    /// Dump the XML of a snapshot.
    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = virDomainSnapshotGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create_xml(dom: &Domain, xml: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = virDomainSnapshotCreateXML(
                dom.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(DomainSnapshot::new(ptr));
        }
    }

    /// Get a handle to the current snapshot
    pub fn current(dom: &Domain, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = virDomainSnapshotCurrent(dom.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(DomainSnapshot::new(ptr));
        }
    }

    /// Get a handle to the parent snapshot, if one exists.
    pub fn get_parent(dom: &Domain, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = virDomainSnapshotGetParent(dom.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(DomainSnapshot::new(ptr));
        }
    }

    /// Delete a snapshot.
    pub fn delete(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSnapshotDelete(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Return the number of snapshots for this domain.
    pub fn num(dom: &Domain, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSnapshotNum(dom.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Return the number of child snapshots for this snapshot.
    pub fn num_children(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSnapshotNumChildren(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Determine if a snapshot is the current snapshot of its domain.
    pub fn is_current(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSnapshotIsCurrent(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    /// Determine if a snapshot has associated libvirt metadata that
    /// would prevent the deletion of its domain.
    pub fn has_metadata(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSnapshotHasMetadata(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    /// Get all snapshot object children for this snapshot.
    pub fn list_all_children(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        unsafe {
            let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
            let size =
                virDomainSnapshotListAllChildren(self.as_ptr(), &mut snaps, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            mem::forget(snaps);

            let mut array: Vec<DomainSnapshot> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainSnapshot::new(*snaps.offset(x)));
            }
            libc::free(snaps as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virDomainSnapshotFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }
}
