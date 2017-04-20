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
use std::{str, ptr};

use connect::sys::virConnectPtr;

use connect::Connect;
use error::Error;

pub mod sys {
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virNetwork {}

    #[allow(non_camel_case_types)]
    pub type virNetworkPtr = *mut virNetwork;
}

#[link(name = "virt")]
extern "C" {
    fn virNetworkLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virNetworkPtr;
    fn virNetworkLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virNetworkPtr;
    fn virNetworkLookupByUUIDString(c: virConnectPtr,
                                    uuid: *const libc::c_char)
                                    -> sys::virNetworkPtr;
    fn virNetworkCreate(c: virConnectPtr, flags: libc::c_uint) -> sys::virNetworkPtr;
    fn virNetworkDestroy(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkUndefine(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkFree(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkIsActive(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkGetName(ptr: sys::virNetworkPtr) -> *const libc::c_char;
    fn virNetworkGetUUIDString(ptr: sys::virNetworkPtr, uuiptr: *mut libc::c_char) -> libc::c_int;
    fn virNetworkGetXMLDesc(ptr: sys::virNetworkPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virNetworkGetBridgeName(ptr: sys::virNetworkPtr) -> *const libc::c_char;
    fn virNetworkGetAutostart(ptr: sys::virNetworkPtr) -> libc::c_int;
    fn virNetworkSetAutostart(ptr: sys::virNetworkPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virNetworkUpdate(ptr: sys::virNetworkPtr,
                        cmptr: libc::c_uint,
                        section: libc::c_uint,
                        index: libc::c_uint,
                        xml: *const libc::c_char,
                        flags: libc::c_uint)
                        -> libc::c_int;
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

pub struct Network {
    ptr: sys::virNetworkPtr,
}

impl Drop for Network {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for Domain")
            }
            return;
        }
    }
}

impl Network {
    pub fn new(ptr: sys::virNetworkPtr) -> Network {
        return Network { ptr: ptr };
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virNetworkGetConnect(self.ptr);
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
            let ptr = virNetworkLookupByName(conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkLookupByUUIDString(conn.as_ptr(),
                                                   CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNetworkGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned());
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virNetworkGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(uuid.as_ptr())
                          .to_string_lossy()
                          .into_owned());
        }
    }

    pub fn get_bridge_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virNetworkGetBridgeName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned());
        }
    }

    pub fn get_xml_desc(&self, flags: NetworkXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virNetworkGetXMLDesc(self.ptr, flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned());
        }
    }

    pub fn create(conn: &Connect, flags: NetworkXMLFlags) -> Result<Network, Error> {
        unsafe {
            let ptr = virNetworkCreate(conn.as_ptr(), flags);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Network { ptr: ptr });
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virNetworkUndefine(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virNetworkFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkIsActive(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkGetAutostart(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = virNetworkSetAutostart(self.ptr, autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn update(&self,
                  cmd: NetworkUpdateCommand,
                  section: NetworkUpdateSection,
                  index: i32,
                  xml: &str,
                  flags: NetworkUpdateFlags)
                  -> Result<(), Error> {
        unsafe {
            let ret = virNetworkUpdate(self.ptr,
                                       cmd,
                                       section,
                                       index as libc::c_uint,
                                       CString::new(xml).unwrap().as_ptr(),
                                       flags);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }
}
