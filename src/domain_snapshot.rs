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
/// See <https://libvirt.org/formatsnapshot.html>
#[derive(Debug)]
pub struct DomainSnapshot {
    ptr: Option<sys::virDomainSnapshotPtr>,
}

unsafe impl Send for DomainSnapshot {}
unsafe impl Sync for DomainSnapshot {}

impl Drop for DomainSnapshot {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for DomainSnapshot: {}", e)
            }
        }
    }
}

impl Clone for DomainSnapshot {
    /// Creates a copy of a domain snapshot.
    ///
    /// Increments the internal reference counter on the given
    /// snapshot. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: DomainSnapshot::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl DomainSnapshot {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainSnapshotPtr) -> DomainSnapshot {
        DomainSnapshot { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<DomainSnapshot, Error> {
        unsafe {
            if sys::virDomainSnapshotRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { DomainSnapshot::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virDomainSnapshotPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virDomainSnapshotGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn get_domain(&self) -> Result<Domain, Error> {
        let ptr = unsafe { sys::virDomainSnapshotGetDomain(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virDomainSnapshotGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Get a handle to a named snapshot.
    pub fn lookup_by_name(dom: &Domain, name: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        let name_buf = CString::new(name).unwrap();
        let ptr = unsafe {
            sys::virDomainSnapshotLookupByName(
                dom.as_ptr(),
                name_buf.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Dump the XML of a snapshot.
    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = unsafe { sys::virDomainSnapshotGetXMLDesc(self.as_ptr(), flags as libc::c_uint) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn create_xml(dom: &Domain, xml: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virDomainSnapshotCreateXML(dom.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Get a handle to the current snapshot
    pub fn current(dom: &Domain, flags: u32) -> Result<DomainSnapshot, Error> {
        let ptr = unsafe { sys::virDomainSnapshotCurrent(dom.as_ptr(), flags as libc::c_uint) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Get a handle to the parent snapshot, if one exists.
    pub fn get_parent(&self, flags: u32) -> Result<DomainSnapshot, Error> {
        let ptr = unsafe { sys::virDomainSnapshotGetParent(self.as_ptr(), flags as libc::c_uint) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Revert a snapshot.
    pub fn revert(&self, flags: u32) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainRevertToSnapshot(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    /// Delete a snapshot.
    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainSnapshotDelete(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    /// Return the number of snapshots for this domain.
    pub fn num(dom: &Domain, flags: u32) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainSnapshotNum(dom.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Return the number of child snapshots for this snapshot.
    pub fn num_children(&self, flags: u32) -> Result<u32, Error> {
        let ret =
            unsafe { sys::virDomainSnapshotNumChildren(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Determine if a snapshot is the current snapshot of its domain.
    pub fn is_current(&self, flags: u32) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainSnapshotIsCurrent(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    /// Determine if a snapshot has associated libvirt metadata that
    /// would prevent the deletion of its domain.
    pub fn has_metadata(&self, flags: u32) -> Result<bool, Error> {
        let ret =
            unsafe { sys::virDomainSnapshotHasMetadata(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    /// Get all snapshot object children for this snapshot.
    pub fn list_all_children(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
        let size = unsafe {
            sys::virDomainSnapshotListAllChildren(self.as_ptr(), &mut snaps, flags as libc::c_uint)
        };
        if size == -1 {
            return Err(Error::last_error());
        }

        let mut array: Vec<DomainSnapshot> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { DomainSnapshot::from_ptr(*snaps.offset(x)) });
        }
        unsafe { libc::free(snaps as *mut libc::c_void) };

        Ok(array)
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainSnapshotFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }
}
