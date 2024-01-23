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

use uuid::Uuid;

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management of networks.
///
/// See <https://libvirt.org/html/libvirt-libvirt-network.html>
#[derive(Debug)]
pub struct Network {
    ptr: Option<sys::virNetworkPtr>,
}

unsafe impl Send for Network {}
unsafe impl Sync for Network {}

impl Drop for Network {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Network: {}", e)
            }
        }
    }
}

impl Clone for Network {
    /// Creates a copy of a network.
    ///
    /// Increments the internal reference counter on the given
    /// network. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: Network::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl Network {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virNetworkPtr) -> Network {
        Network { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<Network, Error> {
        unsafe {
            if sys::virNetworkRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { Network::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virNetworkPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virNetworkGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Network, Error> {
        let id_buf = CString::new(id).unwrap();
        let ptr = unsafe { sys::virNetworkLookupByName(conn.as_ptr(), id_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Network::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid(conn: &Connect, uuid: Uuid) -> Result<Network, Error> {
        let ptr = unsafe { sys::virNetworkLookupByUUID(conn.as_ptr(), uuid.as_bytes().as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Network::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Network, Error> {
        let uuid_buf = CString::new(uuid).unwrap();
        let ptr = unsafe { sys::virNetworkLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Network::from_ptr(ptr) })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virNetworkGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [libc::c_uchar; sys::VIR_UUID_BUFLEN as usize] =
            [0; sys::VIR_UUID_BUFLEN as usize];
        let ret = unsafe { sys::virNetworkGetUUID(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(Uuid::from_bytes(uuid))
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [libc::c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let ret = unsafe { sys::virNetworkGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    pub fn get_bridge_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virNetworkGetBridgeName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    pub fn get_xml_desc(&self, flags: sys::virNetworkXMLFlags) -> Result<String, Error> {
        let xml = unsafe { sys::virNetworkGetXMLDesc(self.as_ptr(), flags) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn create(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virNetworkCreate(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Network, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe { sys::virNetworkDefineXML(conn.as_ptr(), xml_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Network::from_ptr(ptr) })
    }

    pub fn create_xml(conn: &Connect, xml: &str) -> Result<Network, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe { sys::virNetworkCreateXML(conn.as_ptr(), xml_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Network::from_ptr(ptr) })
    }

    pub fn destroy(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virNetworkDestroy(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn undefine(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virNetworkUndefine(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virNetworkFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = unsafe { sys::virNetworkIsActive(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        let ret = unsafe { sys::virNetworkIsPersistent(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        let mut auto = 0;
        let ret = unsafe { sys::virNetworkGetAutostart(self.as_ptr(), &mut auto) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(auto == 1)
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<u32, Error> {
        let ret = unsafe { sys::virNetworkSetAutostart(self.as_ptr(), autostart as libc::c_int) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn update(
        &self,
        cmd: sys::virNetworkUpdateCommand,
        section: sys::virNetworkUpdateSection,
        index: i32,
        xml: &str,
        flags: sys::virNetworkUpdateFlags,
    ) -> Result<(), Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe {
            sys::virNetworkUpdate(
                self.as_ptr(),
                cmd,
                section,
                index as libc::c_int,
                xml_buf.as_ptr(),
                flags,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }
}
