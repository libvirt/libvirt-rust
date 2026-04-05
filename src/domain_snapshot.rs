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

use std::os::raw::{c_uint, c_void};
use std::ptr;

use crate::connect::Connect;
use crate::domain::Domain;
use crate::error::Error;
use crate::util::{check_neg, check_null};

/// Provides APIs for the management of domain snapshots.
///
/// See <https://libvirt.org/formatsnapshot.html>
#[derive(Debug)]
pub struct DomainSnapshot {
    ptr: sys::virDomainSnapshotPtr,
}

unsafe impl Send for DomainSnapshot {}
unsafe impl Sync for DomainSnapshot {}

impl Drop for DomainSnapshot {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virDomainSnapshotFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on domain snapshot: {e}")
        }
    }
}

impl Clone for DomainSnapshot {
    /// Creates a copy of a domain snapshot.
    ///
    /// Increments the internal reference counter on the given
    /// snapshot.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virDomainSnapshotRef(self.as_ptr()) }) {
            panic!("Unable to add reference on domain snapshot: {e}")
        }
        unsafe { DomainSnapshot::from_ptr(self.as_ptr()) }
    }
}

impl DomainSnapshot {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virDomainSnapshotPtr) -> DomainSnapshot {
        DomainSnapshot { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virDomainSnapshotPtr {
        self.ptr
    }

    pub fn connect(&self) -> Result<Connect, Error> {
        let ptr = check_null!(unsafe { sys::virDomainSnapshotGetConnect(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virConnectRef(ptr) }) {
            panic!("Unable to add reference on connection: {e}")
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn domain(&self) -> Result<Domain, Error> {
        let ptr = check_null!(unsafe { sys::virDomainSnapshotGetDomain(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virDomainRef(ptr) }) {
            panic!("Unable to add reference on domain: {e}")
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Returns the snapshot name
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virDomainSnapshotGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Returns the snapshot XML configuration
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotGetXMLDesc>
    pub fn xml_desc(&self, flags: u32) -> Result<String, Error> {
        let xml = check_null!(unsafe {
            sys::virDomainSnapshotGetXMLDesc(self.as_ptr(), flags as c_uint)
        })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Get a handle to the parent snapshot, if one exists.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotGetParent>
    pub fn parent(&self, flags: u32) -> Result<DomainSnapshot, Error> {
        let ptr = check_null!(unsafe {
            sys::virDomainSnapshotGetParent(self.as_ptr(), flags as c_uint)
        })?;
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Revert a snapshot.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainRevertToSnapshot>
    pub fn revert(&self, flags: u32) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainRevertToSnapshot(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Delete a snapshot.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotDelete>
    pub fn delete(&self, flags: u32) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainSnapshotDelete(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Return the number of child snapshots for this snapshot.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotNumChildren>
    pub fn num_children(&self, flags: u32) -> Result<u32, Error> {
        let ret = check_neg!(unsafe {
            sys::virDomainSnapshotNumChildren(self.as_ptr(), flags as c_uint)
        })?;
        Ok(ret as u32)
    }

    /// Determine if a snapshot is the current snapshot of its domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotIsCurrent>
    pub fn is_current(&self, flags: u32) -> Result<bool, Error> {
        let ret =
            check_neg!(unsafe { sys::virDomainSnapshotIsCurrent(self.as_ptr(), flags as c_uint) })?;
        Ok(ret == 1)
    }

    /// Determine if a snapshot has associated libvirt metadata that
    /// would prevent the deletion of its domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotHasMetadata>
    pub fn has_metadata(&self, flags: u32) -> Result<bool, Error> {
        let ret = check_neg!(unsafe {
            sys::virDomainSnapshotHasMetadata(self.as_ptr(), flags as c_uint)
        })?;
        Ok(ret == 1)
    }

    /// Get all snapshot object children for this snapshot.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotListAllChildren>
    pub fn list_all_children(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
        let size = check_neg!(unsafe {
            sys::virDomainSnapshotListAllChildren(self.as_ptr(), &mut snaps, flags as c_uint)
        })?;

        let mut array: Vec<DomainSnapshot> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { DomainSnapshot::from_ptr(*snaps.offset(x)) });
        }
        unsafe { libc::free(snaps as *mut c_void) };

        Ok(array)
    }
}
