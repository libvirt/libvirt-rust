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
    pub struct virInterface {}

    pub type virInterfacePtr = *mut virInterface;
}

#[link(name = "virt")]
extern "C" {
    fn virInterfaceLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virInterfacePtr;
    fn virInterfaceLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virInterfacePtr;
    fn virInterfaceLookupByMACString(
        c: virConnectPtr,
        id: *const libc::c_char,
    ) -> sys::virInterfacePtr;
    fn virInterfaceLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virInterfacePtr;
    fn virInterfaceDefineXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virInterfacePtr;
    fn virInterfaceCreate(ptr: sys::virInterfacePtr, flags: libc::c_uint) -> libc::c_int;
    fn virInterfaceDestroy(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceUndefine(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceFree(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceIsActive(ptr: sys::virInterfacePtr) -> libc::c_int;
    fn virInterfaceGetName(ptr: sys::virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetMACString(ptr: sys::virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetXMLDesc(ptr: sys::virInterfacePtr, flags: libc::c_uint) -> *mut libc::c_char;
    fn virInterfaceGetConnect(ptr: sys::virInterfacePtr) -> virConnectPtr;
}

pub type InterfaceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE: InterfaceXMLFlags = 1 << 0;

/// Provides APIs for the management of interfaces.
///
/// See http://libvirt.org/html/libvirt-libvirt-interface.html
#[derive(Debug)]
pub struct Interface {
    ptr: Option<sys::virInterfacePtr>,
}

impl Drop for Interface {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for Interface, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl Interface {
    pub fn new(ptr: sys::virInterfacePtr) -> Interface {
        return Interface { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virInterfacePtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virInterfaceGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceDefineXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByMACString(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virInterfaceGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        unsafe {
            let mac = virInterfaceGetMACString(self.as_ptr());
            if mac.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(mac, nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: InterfaceXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virInterfaceGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self, flags: InterfaceXMLFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virInterfaceCreate(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceDestroy(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virInterfaceFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virInterfaceIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
