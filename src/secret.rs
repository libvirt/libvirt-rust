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
use std::ptr;

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virSecret {
}

#[allow(non_camel_case_types)]
pub type virSecretPtr = *mut virSecret;

#[link(name = "virt")]
extern {
    fn virSecretLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virSecretPtr;
    fn virSecretLookupByUsage(c: virConnectPtr, usaget: libc::c_int, usageid: *const libc::c_char) -> virSecretPtr;
    fn virSecretUndefine(d: virSecretPtr) -> libc::c_int;
    fn virSecretFree(d: virSecretPtr) -> libc::c_int;
    fn virSecretGetName(d: virSecretPtr) -> *const libc::c_char;
    fn virSecretGetUUIDString(d: virSecretPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virSecretGetUsageID(d: virSecretPtr) -> *const libc::c_char;
    fn virSecretGetXMLDesc(d: virSecretPtr, flags: libc::c_uint) -> *const libc::c_char;

    fn virSecretSetValue(d: virSecretPtr, value: *const libc::c_uchar,
                         vsize: libc::c_uint, flags: libc::c_uint) -> libc::c_int;
    fn virSecretGetValue(d: virSecretPtr, vsize: libc::c_uint, flags: libc::c_uint) -> *const libc::c_uchar;
    fn virSecretGetConnect(d: virSecretPtr) -> virConnectPtr;
    fn virSecretGetUsageType(d: virSecretPtr) -> libc::c_int;
    fn virSecretDefineXML(c: virConnectPtr, xml: *const libc::c_char, flags: libc::c_uint) -> virSecretPtr;
}

pub type SecretXMLFlags = self::libc::c_uint;
pub const VIR_SECRET_XML_INACTIVE:SecretXMLFlags = 1 << 0;

pub type SecretSecretUsageType = self::libc::c_uint;
pub const VIR_SECRET_USAGE_TYPE_NONE: SecretSecretUsageType = 0;
pub const VIR_SECRET_USAGE_TYPE_VOLUME: SecretSecretUsageType = 1;
pub const VIR_SECRET_USAGE_TYPE_CEPH: SecretSecretUsageType = 2;
pub const VIR_SECRET_USAGE_TYPE_ISCSI: SecretSecretUsageType = 3;
pub const VIR_SECRET_USAGE_TYPE_TLS: SecretSecretUsageType = 4;

pub type SecretsFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_SECRETS_EPHEMERAL: SecretsFlags = 1 << 0;
pub const VIR_CONNECT_LIST_SECRETS_NO_EPHEMERAL: SecretsFlags = 1 << 1;
pub const VIR_CONNECT_LIST_SECRETS_PRIVATE: SecretsFlags = 1 << 2;
pub const VIR_CONNECT_LIST_SECRETS_NO_PRIVATE: SecretsFlags  = 1 << 3;


pub struct Secret {
    pub d: virSecretPtr
}

impl Drop for Secret {
    fn drop(&mut self) {
        if !self.d.is_null() {
            self.free();
            return;
        }
    }
}

impl Secret {

    pub fn as_ptr(&self) -> virSecretPtr {
        self.d
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virSecretGetConnect(self.d);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect{c: ptr});
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretDefineXML(
                conn.as_ptr(),  CString::new(xml).unwrap().as_ptr(),
                flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret{d: ptr});
        }
    }

    pub fn lookup_by_usage(conn: &Connect, usagetype: i32, usageid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretLookupByUsage(
                conn.as_ptr(), usagetype as libc::c_int,
                CString::new(usageid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virSecretGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_usage_id(&self) -> Result<String, Error> {
        unsafe {
            let n = virSecretGetUsageID(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_usage_type(&self) -> Result<u32, Error> {
        unsafe {
            let t = virSecretGetUsageType(self.d);
            if t == -1 {
                return Err(Error::new())
            }
            return Ok(t as u32)
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let uuid: *mut libc::c_char = ptr::null_mut();
            if virSecretGetUUIDString(self.d, uuid) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid).to_string_lossy().into_owned())
        }
    }

    pub fn get_xml_desc(&self, flags:SecretXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virSecretGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn set_value(&self, value: &[u8], flags: u32) -> Result<(), Error> {
        unsafe {
            if virSecretSetValue(self.d, value.as_ptr(),
                                 value.len() as libc::c_uint, flags) == -1 {
                return Err(Error::new())
            }
            return Ok(())
        }
    }

    pub fn get_value(&self, size: isize, flags: u32) -> Result<Vec<u8>, Error> {
        unsafe {
            let n = virSecretGetValue(self.d, size as libc::c_uint, flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new())
            }

            let mut array: Vec<u8> = Vec::new();
            for x in 0..size {
                array.push(*n.offset(x))
            }
            return Ok(array)
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virSecretUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virSecretFree(self.d) == -1 {
                return Err(Error::new());
            }
            self.d = ptr::null_mut();
            return Ok(());
        }
    }
}
