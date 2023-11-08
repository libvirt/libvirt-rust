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

impl Network {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn new(ptr: sys::virNetworkPtr) -> Network {
        Network { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virNetworkPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virNetworkGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Network, Error> {
        unsafe {
            let id_buf = CString::new(id).unwrap();
            let ptr = sys::virNetworkLookupByName(conn.as_ptr(), id_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Network::new(ptr))
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Network, Error> {
        unsafe {
            let uuid_buf = CString::new(uuid).unwrap();
            let ptr = sys::virNetworkLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Network::new(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virNetworkGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virNetworkGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(uuid.as_ptr(), nofree))
        }
    }

    pub fn get_bridge_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virNetworkGetBridgeName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn get_xml_desc(&self, flags: sys::virNetworkXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virNetworkGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn create(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNetworkCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Network, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virNetworkDefineXML(conn.as_ptr(), xml_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Network::new(ptr))
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str) -> Result<Network, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virNetworkCreateXML(conn.as_ptr(), xml_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Network::new(ptr))
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virNetworkDestroy(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virNetworkUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virNetworkFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virNetworkIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virNetworkIsPersistent(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut auto = 0;
            let ret = sys::virNetworkGetAutostart(self.as_ptr(), &mut auto);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(auto == 1)
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virNetworkSetAutostart(self.as_ptr(), autostart as libc::c_int);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn update(
        &self,
        cmd: sys::virNetworkUpdateCommand,
        section: sys::virNetworkUpdateSection,
        index: i32,
        xml: &str,
        flags: sys::virNetworkUpdateFlags,
    ) -> Result<(), Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virNetworkUpdate(
                self.as_ptr(),
                cmd,
                section,
                index as libc::c_int,
                xml_buf.as_ptr(),
                flags,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }
}
