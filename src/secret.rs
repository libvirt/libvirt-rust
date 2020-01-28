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

use connect::sys::virConnectPtr;

use connect::Connect;
use error::Error;

pub mod sys {
    #[repr(C)]
    pub struct virSecret {}

    pub type virSecretPtr = *mut virSecret;
}

#[link(name = "virt")]
extern "C" {
    fn virSecretLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virSecretPtr;
    fn virSecretLookupByUsage(
        c: virConnectPtr,
        usaget: libc::c_int,
        usageid: *const libc::c_char,
    ) -> sys::virSecretPtr;
    fn virSecretUndefine(ptr: sys::virSecretPtr) -> libc::c_int;
    fn virSecretFree(ptr: sys::virSecretPtr) -> libc::c_int;
    fn virSecretGetName(ptr: sys::virSecretPtr) -> *const libc::c_char;
    fn virSecretGetUUIDString(ptr: sys::virSecretPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virSecretGetUsageID(ptr: sys::virSecretPtr) -> *const libc::c_char;
    fn virSecretGetXMLDesc(ptr: sys::virSecretPtr, flags: libc::c_uint) -> *const libc::c_char;

    fn virSecretSetValue(
        ptr: sys::virSecretPtr,
        value: *const libc::c_uchar,
        vsize: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virSecretGetValue(
        ptr: sys::virSecretPtr,
        vsize: libc::c_uint,
        flags: libc::c_uint,
    ) -> *const libc::c_uchar;
    fn virSecretGetConnect(ptr: sys::virSecretPtr) -> virConnectPtr;
    fn virSecretGetUsageType(ptr: sys::virSecretPtr) -> libc::c_int;
    fn virSecretDefineXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virSecretPtr;
}

pub type SecretXMLFlags = self::libc::c_uint;
pub const VIR_SECRET_XML_INACTIVE: SecretXMLFlags = 1 << 0;

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
pub const VIR_CONNECT_LIST_SECRETS_NO_PRIVATE: SecretsFlags = 1 << 3;

/// Provides APIs for the management of secrets.
///
/// See http://libvirt.org/html/libvirt-libvirt-secret.html
#[derive(Debug)]
pub struct Secret {
    ptr: Option<sys::virSecretPtr>,
}

impl Drop for Secret {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for Secret, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

impl Secret {
    pub fn new(ptr: sys::virSecretPtr) -> Secret {
        return Secret { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virSecretPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virSecretGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretDefineXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn lookup_by_usage(conn: &Connect, usagetype: i32, usageid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = virSecretLookupByUsage(
                conn.as_ptr(),
                usagetype as libc::c_int,
                string_to_c_chars!(usageid),
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virSecretGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_usage_id(&self) -> Result<String, Error> {
        unsafe {
            let n = virSecretGetUsageID(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_usage_type(&self) -> Result<u32, Error> {
        unsafe {
            let t = virSecretGetUsageType(self.as_ptr());
            if t == -1 {
                return Err(Error::new());
            }
            return Ok(t as u32);
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virSecretGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: SecretXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virSecretGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn set_value(&self, value: &[u8], flags: u32) -> Result<(), Error> {
        unsafe {
            if virSecretSetValue(
                self.as_ptr(),
                value.as_ptr(),
                value.len() as libc::c_uint,
                flags,
            ) == -1
            {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn get_value(&self, size: isize, flags: u32) -> Result<Vec<u8>, Error> {
        unsafe {
            let n = virSecretGetValue(self.as_ptr(), size as libc::c_uint, flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new());
            }

            let mut array: Vec<u8> = Vec::new();
            for x in 0..size {
                array.push(*n.offset(x))
            }
            return Ok(array);
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virSecretUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virSecretFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }
}
