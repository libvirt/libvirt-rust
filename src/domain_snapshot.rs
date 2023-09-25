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
use std::{ptr, str};

use crate::connect::Connect;
use crate::domain::Domain;
use crate::error::Error;

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
                panic!("Unable to drop memory for DomainSnapshot: {}", e)
            }
        }
    }
}

impl DomainSnapshot {
    pub fn new(ptr: sys::virDomainSnapshotPtr) -> DomainSnapshot {
        DomainSnapshot { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virDomainSnapshotPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virDomainSnapshotGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn get_domain(&self) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainSnapshotGetDomain(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainSnapshotGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    /// Get a handle to a named snapshot.
    pub fn lookup_by_name(dom: &Domain, name: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let name_buf = CString::new(name).unwrap();
            let ptr = sys::virDomainSnapshotLookupByName(
                dom.as_ptr(),
                name_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(DomainSnapshot::new(ptr))
        }
    }

    /// Dump the XML of a snapshot.
    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = sys::virDomainSnapshotGetXMLDesc(self.as_ptr(), flags as libc::c_uint);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn create_xml(dom: &Domain, xml: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virDomainSnapshotCreateXML(
                dom.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(DomainSnapshot::new(ptr))
        }
    }

    /// Get a handle to the current snapshot
    pub fn current(dom: &Domain, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = sys::virDomainSnapshotCurrent(dom.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(DomainSnapshot::new(ptr))
        }
    }

    /// Get a handle to the parent snapshot, if one exists.
    pub fn get_parent(&self, flags: u32) -> Result<DomainSnapshot, Error> {
        unsafe {
            let ptr = sys::virDomainSnapshotGetParent(self.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(DomainSnapshot::new(ptr))
        }
    }

    /// Revert a snapshot.
    pub fn revert(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virDomainRevertToSnapshot(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    /// Delete a snapshot.
    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virDomainSnapshotDelete(self.as_ptr(), flags as libc::c_uint) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    /// Return the number of snapshots for this domain.
    pub fn num(dom: &Domain, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainSnapshotNum(dom.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Return the number of child snapshots for this snapshot.
    pub fn num_children(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainSnapshotNumChildren(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Determine if a snapshot is the current snapshot of its domain.
    pub fn is_current(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSnapshotIsCurrent(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    /// Determine if a snapshot has associated libvirt metadata that
    /// would prevent the deletion of its domain.
    pub fn has_metadata(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSnapshotHasMetadata(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    /// Get all snapshot object children for this snapshot.
    pub fn list_all_children(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        unsafe {
            let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
            let size = sys::virDomainSnapshotListAllChildren(
                self.as_ptr(),
                &mut snaps,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<DomainSnapshot> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainSnapshot::new(*snaps.offset(x)));
            }
            libc::free(snaps as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainSnapshotFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }
}
