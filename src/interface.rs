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

use std::str;

use crate::connect::Connect;
use crate::error::Error;

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
                panic!("Unable to drop memory for Interface: {}", e)
            }
        }
    }
}

impl Interface {
    pub fn new(ptr: sys::virInterfacePtr) -> Interface {
        Interface { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virInterfacePtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virInterfaceGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Interface::new(ptr))
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceDefineXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Interface::new(ptr))
        }
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceLookupByMACString(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Interface::new(ptr))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virInterfaceGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        unsafe {
            let mac = sys::virInterfaceGetMACString(self.as_ptr());
            if mac.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(mac, nofree))
        }
    }

    pub fn get_xml_desc(&self, flags: sys::virInterfaceXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virInterfaceGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn create(&self, flags: sys::virInterfaceXMLFlags) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virInterfaceCreate(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn destroy(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceDestroy(self.as_ptr(), flags) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virInterfaceIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }
}
