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
use std::str;

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virDomain {
}

#[allow(non_camel_case_types)]
pub type virDomainPtr = *const virDomain;

#[link(name = "virt")]
extern {
    fn virDomainLookupByID(c: virConnectPtr, id: libc::c_int) -> virDomainPtr;
    fn virDomainLookupByName(c: virConnectPtr, id: *const libc::c_char) -> virDomainPtr;
    fn virDomainLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virDomainPtr;
    fn virDomainCreate(c: virConnectPtr) -> virDomainPtr;
    fn virDomainCreateWithFlags(c: virConnectPtr, flags: libc::c_uint) -> virDomainPtr;
    fn virDomainDestroy(d: virDomainPtr) -> libc::c_int;
    fn virDomainUndefine(d: virDomainPtr) -> libc::c_int;
    fn virDomainFree(d: virDomainPtr) -> libc::c_int;
    fn virDomainShutdown(d: virDomainPtr) -> libc::c_int;
    fn virDomainReboot(d: virDomainPtr) -> libc::c_int;
    fn virDomainIsActive(d: virDomainPtr) -> libc::c_int;
    fn virDomainIsUpdated(d: virDomainPtr) -> libc::c_int;
    fn virDomainGetName(d: virDomainPtr) -> *const libc::c_char;
    fn virDomainGetUUIDString(d: virDomainPtr) -> *const libc::c_char;
    fn virDomainGetXMLDesc(d: virDomainPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virDomainGetAutostart(d: virDomainPtr) -> libc::c_int;
    fn virDomainSetAutostart(d: virDomainPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virDomainGetID(d: virDomainPtr) -> libc::c_int;
}

pub type DomainXMLFlags = self::libc::c_uint;
pub const VIR_DOMAIN_XML_SECURE: DomainXMLFlags = 1 << 0;
pub const VIR_DOMAIN_XML_INACTIVE: DomainXMLFlags = 1 << 1;
pub const VIR_DOMAIN_XML_UPDATE_CPU: DomainXMLFlags = 1 << 2;
pub const VIR_DOMAIN_XML_MIGRATABLE: DomainXMLFlags = 1 << 3;

pub type DomainCreateFlags = self::libc::c_uint;
pub const VIR_DOMAIN_NONE: DomainCreateFlags = 0;
pub const VIR_DOMAIN_START_PAUSED: DomainCreateFlags = 1 << 0;
pub const VIR_DOMAIN_START_AUTODESTROY: DomainCreateFlags = 1 << 1;
pub const VIR_DOMAIN_START_BYPASS_CACHE: DomainCreateFlags = 1 << 2;
pub const VIR_DOMAIN_START_FORCE_BOOT: DomainCreateFlags = 1 << 3;
pub const VIR_DOMAIN_START_VALIDATE: DomainCreateFlags = 1 << 4;

pub struct Domain {
    pub d: virDomainPtr
}

impl Domain {

    pub fn as_ptr(&self) -> virDomainPtr {
        self.d
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<&str, Error> {
        unsafe {
            let n = virDomainGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }

    pub fn get_uuid_string(&self) -> Result<&str, Error> {
        unsafe {
            let n = virDomainGetUUIDString(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }

    pub fn get_id(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainGetID(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_xml_desc(&self, flags:DomainCreateFlags) -> Result<&str, Error> {
        unsafe {
            let xml = virDomainGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(xml).to_bytes()).unwrap())
        }
    }

    pub fn create(conn: &Connect) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreate(conn.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain{d: ptr});
        }
    }

    pub fn create_with_flags(conn: &Connect, flags: DomainXMLFlags) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreateWithFlags(conn.as_ptr(), flags);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain{d: ptr});
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virDomainDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn shutdown(&self) -> Result<(), Error> {
        unsafe {
            if virDomainShutdown(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn reboot(&self) -> Result<(), Error> {
        unsafe {
            if virDomainReboot(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsActive(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virDomainUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virDomainFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsUpdated(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainGetAutostart(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetAutostart(self.d, autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
