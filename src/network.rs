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
    pub struct virNetwork {}

    pub type virNetworkPtr = *mut virNetwork;
}

#[link(name = "virt")]
extern "C" {
    fn virNetworkLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virNetworkPtr;
    fn virNetworkLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virNetworkPtr;
    fn virNetworkLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virNetworkPtr;
    fn virNetworkCreate(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> sys::virNetworkPtr;
    fn virNetworkCreateXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virNetworkPtr;
    fn virNetworkDestroy(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkUndefine(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkFree(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkIsActive(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkIsPersistent(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkGetName(ptr: sys::virNetworkPtr) -> *const libc::c_char;
    fn virNetworkGetUUIDString(ptr: sys::virNetworkPtr, uuiptr: *mut libc::c_char) -> libc::c_int;
    fn virNetworkGetXMLDesc(ptr: sys::virNetworkPtr, flags: libc::c_uint) -> *mut libc::c_char;
    fn virNetworkGetBridgeName(ptr: sys::virNetworkPtr) -> *mut libc::c_char;
    fn virNetworkGetAutostart(ptr: sys::virNetworkPtr, autostart: *mut libc::c_int) -> libc::c_int;
    fn virNetworkSetAutostart(ptr: sys::virNetworkPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virNetworkUpdate(
        ptr: sys::virNetworkPtr,
        cmptr: libc::c_uint,
        section: libc::c_uint,
        index: libc::c_uint,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virNetworkGetConnect(ptr: sys::virNetworkPtr) -> virConnectPtr;
}

pub type NetworkXMLFlags = self::libc::c_uint;
pub const VIR_NETWORK_XML_INACTIVE: NetworkXMLFlags = 1 << 0;

pub type NetworkUpdateCommand = self::libc::c_uint;
pub const VIR_NETWORK_UPDATE_COMMAND_NONE: NetworkUpdateCommand = 0;
pub const VIR_NETWORK_UPDATE_COMMAND_MODIFY: NetworkUpdateCommand = 1;
pub const VIR_NETWORK_UPDATE_COMMAND_DELETE: NetworkUpdateCommand = 2;
pub const VIR_NETWORK_UPDATE_COMMAND_ADD_LAST: NetworkUpdateCommand = 3;
pub const VIR_NETWORK_UPDATE_COMMAND_ADD_FIRST: NetworkUpdateCommand = 4;

pub type NetworkUpdateSection = self::libc::c_uint;
pub const VIR_NETWORK_SECTION_NONE: NetworkUpdateSection = 0;
pub const VIR_NETWORK_SECTION_BRIDGE: NetworkUpdateSection = 1;
pub const VIR_NETWORK_SECTION_DOMAIN: NetworkUpdateSection = 2;
pub const VIR_NETWORK_SECTION_IP: NetworkUpdateSection = 3;
pub const VIR_NETWORK_SECTION_IP_DHCP_HOST: NetworkUpdateSection = 4;
pub const VIR_NETWORK_SECTION_IP_DHCP_RANGE: NetworkUpdateSection = 5;
pub const VIR_NETWORK_SECTION_FORWARD: NetworkUpdateSection = 6;
pub const VIR_NETWORK_SECTION_FORWARD_INTERFACE: NetworkUpdateSection = 7;
pub const VIR_NETWORK_SECTION_FORWARD_PF: NetworkUpdateSection = 8;
pub const VIR_NETWORK_SECTION_PORTGROUP: NetworkUpdateSection = 9;
pub const VIR_NETWORK_SECTION_DNS_HOST: NetworkUpdateSection = 10;
pub const VIR_NETWORK_SECTION_DNS_TXT: NetworkUpdateSection = 11;
pub const VIR_NETWORK_SECTION_DNS_SRV: NetworkUpdateSection = 12;

pub type NetworkUpdateFlags = self::libc::c_uint;
pub const VIR_NETWORK_UPDATE_AFFECT_CURRENT: NetworkUpdateFlags = 0;
pub const VIR_NETWORK_UPDATE_AFFECT_LIVE: NetworkUpdateFlags = 1 << 0;
pub const VIR_NETWORK_UPDATE_AFFECT_CONFIG: NetworkUpdateFlags = 1 << 1;

/// Provides APIs for the management of networks.
///
/// See http://libvirt.org/html/libvirt-libvirt-network.html
#[derive(Debug)]
pub struct Network {
    ptr: Option<sys::virNetworkPtr>,
}

impl Drop for Network {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for Network, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl Network {
    pub fn new(ptr: sys::virNetworkPtr) -> Network {
        return Network { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virNetworkPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virNetworkGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNetworkGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNetworkGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_bridge_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNetworkGetBridgeName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_xml_desc(&self, flags: NetworkXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virNetworkGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virNetworkCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkDefineXML(conn.as_ptr(), string_to_c_chars!(xml));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkCreateXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkDestroy(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNetworkFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkIsPersistent(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut auto = 0;
            let ret = virNetworkGetAutostart(self.as_ptr(), &mut auto);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(auto == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<u32, Error> {
        unsafe {
            let ret = virNetworkSetAutostart(self.as_ptr(), autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn update(
        &self,
        cmd: NetworkUpdateCommand,
        section: NetworkUpdateSection,
        index: i32,
        xml: &str,
        flags: NetworkUpdateFlags,
    ) -> Result<(), Error> {
        unsafe {
            let ret = virNetworkUpdate(
                self.as_ptr(),
                cmd,
                section,
                index as libc::c_uint,
                string_to_c_chars!(xml),
                flags,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
