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

use crate::connect::Connect;
use crate::error::Error;

/// Provides APIs for the management of secrets.
///
/// See <https://libvirt.org/html/libvirt-libvirt-secret.html>
#[derive(Debug)]
pub struct Secret {
    ptr: Option<sys::virSecretPtr>,
}

impl Drop for Secret {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Secret: {}", e)
            }
        }
    }
}

impl Secret {
    pub fn new(ptr: sys::virSecretPtr) -> Secret {
        Secret { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virSecretPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virSecretGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Secret, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr =
                sys::virSecretDefineXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Secret::new(ptr))
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Secret, Error> {
        unsafe {
            let uuid_buf = CString::new(uuid).unwrap();
            let ptr = sys::virSecretLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Secret::new(ptr))
        }
    }

    pub fn lookup_by_usage(conn: &Connect, usagetype: i32, usageid: &str) -> Result<Secret, Error> {
        unsafe {
            let usageid_buf = CString::new(usageid).unwrap();
            let ptr = sys::virSecretLookupByUsage(
                conn.as_ptr(),
                usagetype as libc::c_int,
                usageid_buf.as_ptr(),
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Secret::new(ptr))
        }
    }

    pub fn get_usage_id(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virSecretGetUsageID(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn get_usage_type(&self) -> Result<u32, Error> {
        unsafe {
            let t = sys::virSecretGetUsageType(self.as_ptr());
            if t == -1 {
                return Err(Error::last_error());
            }
            Ok(t as u32)
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virSecretGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(uuid.as_ptr(), nofree))
        }
    }

    pub fn get_xml_desc(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let xml = sys::virSecretGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    pub fn set_value(&self, value: &[u8], flags: u32) -> Result<(), Error> {
        unsafe {
            if sys::virSecretSetValue(self.as_ptr(), value.as_ptr(), value.len(), flags) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn get_value(&self, flags: u32) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut size: usize = 0;
            let n = sys::virSecretGetValue(self.as_ptr(), &mut size, flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::last_error());
            }

            let mut array: Vec<u8> = Vec::new();
            for x in 0..size {
                array.push(*n.add(x))
            }
            Ok(array)
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virSecretUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virSecretFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }
}
