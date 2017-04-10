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
use std::{str, ptr, slice, mem};

use domain::Domain;
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virConnect {
}

#[allow(non_camel_case_types)]
pub type virConnectPtr = *const virConnect;

#[link(name = "virt")]
extern {
    fn virGetVersion(hyver: *const libc::c_ulong,
                     ctype: *const libc::c_char,
                     typever: *const libc::c_ulong) -> libc::c_int;
    fn virConnectOpen(uri: *const libc::c_char) -> virConnectPtr;
    fn virConnectOpenReadOnly(uri: *const libc::c_char) -> virConnectPtr;    
    fn virConnectClose(c: virConnectPtr) -> libc::c_int;
    fn virConnectGetVersion(c: virConnectPtr,
                            hyver: *const libc::c_ulong) -> libc::c_int;
    fn virConnectGetHostname(c: virConnectPtr) -> *const libc::c_char;
    fn virConnectGetLibVersion(c: virConnectPtr,
                               ver: *const libc::c_ulong) -> libc::c_int;
    fn virConnectGetType(c: virConnectPtr) -> *const libc::c_char;
    fn virConnectIsAlive(c: virConnectPtr) -> libc::c_int;
    fn virConnectIsEncrypted(c: virConnectPtr) -> libc::c_int;
    fn virConnectIsSecure(c: virConnectPtr) -> libc::c_int;
    fn virConnectListDomains(c: virConnectPtr,
                             ids: *const libc::c_int,
                             maxids: libc::c_int) -> libc::c_int;
    fn virConnectListDefinedDomains(c: virConnectPtr,
                                    names: *const *const libc::c_char,
                                    maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListInterfaces(c: virConnectPtr,
                                names: *const *const libc::c_char,
                                maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListNetworks(c: virConnectPtr,
                              names: *const *const libc::c_char,
                              maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListNWFilters(c: virConnectPtr,
                               names: *const *const libc::c_char,
                               maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListStoragePools(c: virConnectPtr,
                                  names: *const *const libc::c_char,
                                  maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListSecrets(c: virConnectPtr,
                             names: *const *const libc::c_char,
                             maxnames: libc::c_int) -> libc::c_int;
    fn virConnectListDefinedInterfaces(c: virConnectPtr,
                                       names: *const *const libc::c_char,
                                       maxifaces: libc::c_int) -> libc::c_int;
    fn virConnectListDefinedNetworks(c: virConnectPtr,
                                     names: *const *const libc::c_char,
                                     maxnets: libc::c_int) -> libc::c_int;
    fn virConnectListDefinedStoragePools(c: virConnectPtr,
                                         names: *const *const libc::c_char,
                                         maxpools: libc::c_int) -> libc::c_int;
    fn virConnectNumOfDomains(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfInterfaces(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfNetworks(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfStoragePools(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedDomains(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedInterfaces(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedNetworks(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedStoragePools(c: virConnectPtr) -> libc::c_int;

}

pub struct Connect {
    pub c: virConnectPtr
}

impl Connect {

    pub fn as_ptr(&self) -> virConnectPtr {
        self.c
    }

    pub fn get_version() -> Result<u32, Error> {
        unsafe {
            let ver: libc::c_ulong = 0;
            if virGetVersion(&ver, ptr::null(), ptr::null()) == -1 {
                return Err(Error::new());
            }
            return Ok(ver as u32);
        }
    }

    /// This function should be called first to get a connection to
    /// the Hypervisor and xen store.
    ///
    /// If @uri is "", if the LIBVIRT_DEFAULT_URI environment
    /// variable is set, then it will be used. Otherwise if the client
    /// configuration file has the "uri_default" parameter set, then
    /// it will be used. Finally probing will be done to determine a
    /// suitable default driver to activate. This involves trying each
    /// hypervisor in turn until one successfully opens.
    ///
    /// If connecting to an unprivileged hypervisor driver which
    /// requires the libvirtd daemon to be active, it will
    /// automatically be launched if not already running. This can be
    /// prevented by setting the environment variable
    /// LIBVIRT_AUTOSTART=0
    ///
    /// URIs are documented at http://libvirt.org/uri.html
    ///
    /// Connect.close should be used to release the resources after the
    /// connection is no longer needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///       conn.close();
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn new(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpen(CString::new(uri).unwrap().as_ptr());
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect{c: c});
        }
    }

    /// This function should be called first to get a restricted
    /// connection to the library functionalities. The set of APIs
    /// usable are then restricted on the available methods to control
    /// the domains.
    ///
    /// See 'new' for notes about environment variables which can have
    /// an effect on opening drivers and freeing the connection
    /// resources.
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new_read_only("test:///default") {
    ///   Ok(conn) => {
    ///       conn.close();
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn new_read_only(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpenReadOnly(CString::new(uri).unwrap().as_ptr());
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect{c: c});
        }
    }

    /// This function closes the connection to the hypervisor. This
    /// should not be called if further interaction with the
    /// hypervisor are needed especially if there is running domain
    /// which need further monitoring by the application.
    pub fn close(&self) {
        unsafe {
            virConnectClose(self.c);
        }
    }

    /// This returns a system hostname on which the hypervisor is
    /// running (based on the result of the gethostname system call,
    /// but possibly expanded to a fully-qualified domain name via
    /// getaddrinfo).  If we are connected to a remote system, then
    /// this returns the hostname of the remote system.
    pub fn get_hostname(&self) -> Result<&str, Error> {
        unsafe {
            let n = virConnectGetHostname(self.c);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(n).to_bytes()).unwrap())
        }
    }

    pub fn get_lib_version(&self) -> Result<u32, Error> {
        unsafe {
            let ver: libc::c_ulong = 0;
            if virConnectGetLibVersion(self.c, &ver) == -1 {
                return Err(Error::new());
            }
            return Ok(ver as u32);
        }
    }

    pub fn get_type(&self) -> Result<&str, Error> {
        unsafe {
            let t = virConnectGetType(self.c);
            if t.is_null() {
                return Err(Error::new())
            }
            return Ok(str::from_utf8(
                CStr::from_ptr(t).to_bytes()).unwrap())
        }
    }

    pub fn is_alive(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsAlive(self.c);
            if t == -1 {
                return Err(Error::new())
            }
            return Ok(t == 1)
        }
    }

    pub fn is_enscrypted(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsEncrypted(self.c);
            if t == -1 {
                return Err(Error::new())
            }
            return Ok(t == 1)
        }
    }

    pub fn is_secure(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsSecure(self.c);
            if t == -1 {
                return Err(Error::new())
            }
            return Ok(t == 1)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_domains() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_domains(&self) -> Result<&[u32], Error> {
        unsafe {
            let ptr: *const libc::c_int = mem::uninitialized();
            let size = virConnectListDomains(self.c, ptr, 512);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr as *const u32, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_interfaces() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_interfaces(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListInterfaces(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_networks() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_networks(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListNetworks(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    pub fn list_nw_filters(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListNWFilters(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    pub fn list_secrets(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListSecrets(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_storage_pools() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_storage_pools(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListStoragePools(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_defined_domains() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_domains(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListDefinedDomains(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_defined_interfaces() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_interfaces(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListDefinedInterfaces(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_defined_storage_pools() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_storage_pools(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListDefinedStoragePools(
                self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.list_networks() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_networks(&self) -> Result<&[&str], Error> {
        unsafe {
            let ptr: [*const libc::c_char; 1024] = mem::uninitialized();
            let size = virConnectListDefinedNetworks(self.c, ptr.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }
            let array = slice::from_raw_parts(
                ptr.as_ptr() as *const &str, size as usize);
            return Ok(array)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_domains() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDomains(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }
    
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_interfaces() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfInterfaces(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_networks() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfNetworks(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_storage_pools() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfStoragePools(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_defined_domains() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedDomains(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }
    
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_defined_interfaces() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedInterfaces(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_defined_networks() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedNetworks(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///     match conn.num_of_defined_storage_pools() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     conn.close();
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedStoragePools(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    /// Connect.close should be used to release the resources after the
    /// connection is no longer needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::new("test:///default") {
    ///   Ok(conn) => {
    ///       match conn.get_hyp_version() {
    ///         Ok(hyver) => assert_eq!(2, hyver),
    ///         Err(e) => panic!(
    ///           "failed with code {}, message: {}", e.code, e.message)
    ///       }
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn get_hyp_version(&self) -> Result<u32, Error> {
        unsafe {
            let hyver: libc::c_ulong = 0;
            if virConnectGetVersion(self.c, &hyver) == -1 {
                return Err(Error::new());
            }
            return Ok(hyver as u32);
        }
    }

    pub fn domain_lookup_by_id(&self, id: u32) -> Result<Domain, Error> {
        Domain::lookup_by_id(self, id)
    }

    pub fn domain_lookup_by_name(&self, id: &str) -> Result<Domain, Error> {
        Domain::lookup_by_name(self, id)
    }
}

#[test]
fn exercices() {
    match Connect::new("test:///default") {
        Ok(conn) => {
            println!("hostname={}", conn.get_hostname().unwrap_or("unknow"));
            println!("is alive={}", conn.is_alive().unwrap_or(false)); // default false 
            println!("is secure={}", conn.is_secure().unwrap_or(false)); // default false
            println!("is encrypted={}", conn.is_enscrypted().unwrap_or(true)); // default true
            println!("version={}", Connect::get_version().unwrap_or(0));
            println!("hyp version={}", conn.get_hyp_version().unwrap_or(0));
            println!("lib version={}", conn.get_lib_version().unwrap_or(0));
            println!("type={}", conn.get_type().unwrap_or("unknow"));
            conn.close();
        },
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
}

#[test]
fn list_domains() {
    match Connect::new("test:///default") {
        Ok(conn) => {
            let doms = conn.list_domains().unwrap_or(&[]);
            assert_eq!(1, doms.len());
            let domid = doms[0];
            match conn.domain_lookup_by_id(domid) {
                Ok(domain) => println!("A domain name: {}",
                                       domain.get_name().unwrap_or("noname")),
                Err(e) => panic!(
                    "failed with code {}, message: {}", e.code, e.message)
            }
            conn.close();
        },
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
}
