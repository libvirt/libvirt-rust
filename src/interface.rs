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

/// Provides APIs for the management of interfaces.
///
/// See <https://libvirt.org/html/libvirt-libvirt-interface.html>
#[derive(Debug)]
pub struct Interface {
    ptr: Option<sys::virInterfacePtr>,
}

unsafe impl Send for Interface {}
unsafe impl Sync for Interface {}

impl Drop for Interface {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Interface: {}", e)
            }
        }
    }
}

impl Clone for Interface {
    /// Creates a copy of a interface.
    ///
    /// Increments the internal reference counter on the given
    /// interface. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: Interface::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl Interface {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virInterfacePtr) -> Interface {
        Interface { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<Interface, Error> {
        unsafe {
            if sys::virInterfaceRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { Interface::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virInterfacePtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virInterfaceGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        let id_buf = CString::new(id).unwrap();
        let ptr = unsafe { sys::virInterfaceLookupByName(conn.as_ptr(), id_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Interface::from_ptr(ptr) })
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Interface, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virInterfaceDefineXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Interface::from_ptr(ptr) })
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        let id_buf = CString::new(id).unwrap();
        let ptr = unsafe { sys::virInterfaceLookupByMACString(conn.as_ptr(), id_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Interface::from_ptr(ptr) })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virInterfaceGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        let mac = unsafe { sys::virInterfaceGetMACString(self.as_ptr()) };
        if mac.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(mac, nofree) })
    }

    pub fn get_xml_desc(&self, flags: sys::virInterfaceXMLFlags) -> Result<String, Error> {
        let xml = unsafe { sys::virInterfaceGetXMLDesc(self.as_ptr(), flags) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    pub fn create(&self, flags: sys::virInterfaceXMLFlags) -> Result<u32, Error> {
        let ret = unsafe { sys::virInterfaceCreate(self.as_ptr(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn destroy(&self, flags: u32) -> Result<(), Error> {
        let ret = unsafe { sys::virInterfaceDestroy(self.as_ptr(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn undefine(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virInterfaceUndefine(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virInterfaceFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = unsafe { sys::virInterfaceIsActive(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }
}
