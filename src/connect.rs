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

use domain::{Domain, virDomainPtr};
use error::Error;
use network::{Network, virNetworkPtr};
use nodedev::{NodeDevice, virNodeDevicePtr};
use nwfilter::{NWFilter, virNWFilterPtr};
use interface::{Interface, virInterfacePtr};
use storage_pool::{StoragePool, virStoragePoolPtr};
use secret::{Secret, virSecretPtr};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virConnect {
}

#[allow(non_camel_case_types)]
pub type virConnectPtr = *const virConnect;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virConnectCredential {
}

#[allow(non_camel_case_types)]
pub type virConnectCredentialPtr = *const virConnectCredential;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virConnectAuthCallback {
}

#[allow(non_camel_case_types)]
pub type virConnectAuthCallbackPtr = *const virConnectAuthCallback;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virConnectAuth {
    credtype: *const libc::c_uint,
    ncredtype: libc::c_uint,
    cb: unsafe extern fn(*const virConnectCredential, u32, *const libc::c_void) -> i32,
    cbdata: *const libc::c_void,
}

#[allow(non_camel_case_types)]
pub type virConnectAuthPtr = *const virConnect;


//#[deny(dead_code)]
#[link(name="virt")]
extern {
    fn virGetVersion(hyver: *const libc::c_ulong,
                     ctype: *const libc::c_char,
                     typever: *const libc::c_ulong) -> libc::c_int;
    fn virConnectOpen(uri: *const libc::c_char) -> virConnectPtr;
    fn virConnectOpenReadOnly(uri: *const libc::c_char) -> virConnectPtr;
    fn virConnectOpenAuth(uri: *const libc::c_char, auth: *const virConnectAuth, flags: libc::c_uint) -> virConnectPtr;
    fn virConnectClose(c: virConnectPtr) -> libc::c_int;
    fn virConnectGetVersion(c: virConnectPtr,
                            hyver: *const libc::c_ulong) -> libc::c_int;
    fn virConnectGetHostname(c: virConnectPtr) -> *const libc::c_char;
    fn virConnectGetCapabilities(c: virConnectPtr) -> *const libc::c_char;
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
    fn virConnectListAllDomains(c: virConnectPtr,
                                domains: *mut *mut virDomainPtr,
                                flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllNetworks(c: virConnectPtr,
                                 networks: *mut *mut virNetworkPtr,
                                 flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllInterfaces(c: virConnectPtr,
                                   interfaces: *mut *mut virInterfacePtr,
                                   flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllNodeDevices(c: virConnectPtr,
                                    devices: *mut *mut virNodeDevicePtr,
                                    flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllSecrets(c: virConnectPtr,
                                secrets: *mut *mut virSecretPtr,
                                flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllNWFilters(c: virConnectPtr,
                                  nwfilters: *mut *mut virNWFilterPtr,
                                  flags: libc::c_uint) -> libc::c_int;
    fn virConnectListAllStoragePools(c: virConnectPtr,
                                     storages: *mut *mut virStoragePoolPtr,
                                     flags: libc::c_uint) -> libc::c_int;
    fn virConnectNumOfDomains(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfInterfaces(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfNetworks(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfStoragePools(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfNWFilters(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfSecrets(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedDomains(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedInterfaces(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedNetworks(c: virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedStoragePools(c: virConnectPtr) -> libc::c_int;
}

pub type ConnectFlags = self::libc::c_uint;
pub const VIR_CONNECT_RO: ConnectFlags = 1 << 0;
pub const VIR_CONNECT_NO_ALIASES: ConnectFlags = 1 << 1;

pub type ConnectListAllNodeDeviceFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SYSTEM: ConnectListAllNodeDeviceFlags = 1 << 0;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_PCI_DEV: ConnectListAllNodeDeviceFlags = 1 << 1;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_USB_DEV: ConnectListAllNodeDeviceFlags = 1 << 2;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_USB_INTERFACE: ConnectListAllNodeDeviceFlags = 1 << 3;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_NET: ConnectListAllNodeDeviceFlags = 1 << 4;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_HOST: ConnectListAllNodeDeviceFlags = 1 << 5;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_TARGET: ConnectListAllNodeDeviceFlags = 1 << 6;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI: ConnectListAllNodeDeviceFlags = 1 << 7;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_STORAGE: ConnectListAllNodeDeviceFlags = 1 << 8;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_FC_HOST: ConnectListAllNodeDeviceFlags = 1 << 9;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_VPORTS: ConnectListAllNodeDeviceFlags = 1 << 10;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_GENERIC: ConnectListAllNodeDeviceFlags = 1 << 11;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_DRM: ConnectListAllNodeDeviceFlags = 1 << 12;

pub type ConnectListAllSecretsFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_SECRETS_EPHEMERAL: ConnectListAllSecretsFlags = 1 << 0;
pub const VIR_CONNECT_LIST_SECRETS_NO_EPHEMERAL: ConnectListAllSecretsFlags = 1 << 1;
pub const VIR_CONNECT_LIST_SECRETS_PRIVATE: ConnectListAllSecretsFlags = 1 << 2;
pub const VIR_CONNECT_LIST_SECRETS_NO_PRIVATE: ConnectListAllSecretsFlags  = 1 << 3;

pub type ConnectListAllDomainsFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_DOMAINS_ACTIVE: ConnectListAllDomainsFlags = 1 << 0;
pub const VIR_CONNECT_LIST_DOMAINS_INACTIVE: ConnectListAllDomainsFlags = 1 << 1;
pub const VIR_CONNECT_LIST_DOMAINS_PERSISTENT: ConnectListAllDomainsFlags = 1 << 2;
pub const VIR_CONNECT_LIST_DOMAINS_TRANSIENT: ConnectListAllDomainsFlags = 1 << 3;
pub const VIR_CONNECT_LIST_DOMAINS_RUNNING: ConnectListAllDomainsFlags = 1 << 4;
pub const VIR_CONNECT_LIST_DOMAINS_PAUSED: ConnectListAllDomainsFlags = 1 << 5;
pub const VIR_CONNECT_LIST_DOMAINS_SHUTOFF: ConnectListAllDomainsFlags = 1 << 6;
pub const VIR_CONNECT_LIST_DOMAINS_OTHER: ConnectListAllDomainsFlags = 1 << 7;
pub const VIR_CONNECT_LIST_DOMAINS_MANAGEDSAVE: ConnectListAllDomainsFlags = 1 << 8;
pub const VIR_CONNECT_LIST_DOMAINS_NO_MANAGEDSAVE: ConnectListAllDomainsFlags = 1 << 9;
pub const VIR_CONNECT_LIST_DOMAINS_AUTOSTART: ConnectListAllDomainsFlags = 1 << 10;
pub const VIR_CONNECT_LIST_DOMAINS_NO_AUTOSTART: ConnectListAllDomainsFlags = 1 << 11;
pub const VIR_CONNECT_LIST_DOMAINS_HAS_SNAPSHOT: ConnectListAllDomainsFlags = 1 << 12;
pub const VIR_CONNECT_LIST_DOMAINS_NO_SNAPSHOT: ConnectListAllDomainsFlags = 1 << 13;

pub type ConnectListAllNetworksFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_NETWORKS_INACTIVE: ConnectListAllNetworksFlags = 1 << 0;
pub const VIR_CONNECT_LIST_NETWORKS_ACTIVE: ConnectListAllNetworksFlags = 1 << 1;
pub const VIR_CONNECT_LIST_NETWORKS_PERSISTENT: ConnectListAllNetworksFlags = 1 << 2;
pub const VIR_CONNECT_LIST_NETWORKS_TRANSIENT: ConnectListAllNetworksFlags = 1 << 3;
pub const VIR_CONNECT_LIST_NETWORKS_AUTOSTART: ConnectListAllNetworksFlags = 1 << 4;
pub const VIR_CONNECT_LIST_NETWORKS_NO_AUTOSTART: ConnectListAllNetworksFlags = 1 << 5;

pub type ConnectListAllInterfacesFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_INTERFACES_INACTIVE: ConnectListAllInterfacesFlags = 1 << 0;
pub const VIR_CONNECT_LIST_INTERFACES_ACTIVE: ConnectListAllInterfacesFlags = 1 << 1;

pub type ConnectListAllStoragePoolsFlags = self::libc::c_uint;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_INACTIVE: ConnectListAllStoragePoolsFlags = 1 << 0;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_ACTIVE: ConnectListAllStoragePoolsFlags = 1 << 1;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_PERSISTENT: ConnectListAllStoragePoolsFlags = 1 << 2;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_TRANSIENT: ConnectListAllStoragePoolsFlags = 1 << 3;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_AUTOSTART: ConnectListAllStoragePoolsFlags = 1 << 4;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_NO_AUTOSTART: ConnectListAllStoragePoolsFlags = 1 << 5;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_DIR: ConnectListAllStoragePoolsFlags = 1 << 6;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_FS: ConnectListAllStoragePoolsFlags = 1 << 7;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_NETFS: ConnectListAllStoragePoolsFlags = 1 << 8;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_LOGICAL: ConnectListAllStoragePoolsFlags = 1 << 9;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_DISK: ConnectListAllStoragePoolsFlags = 1 << 10;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_ISCSI: ConnectListAllStoragePoolsFlags = 1 << 11;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_SCSI: ConnectListAllStoragePoolsFlags = 1 << 12;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_MPATH: ConnectListAllStoragePoolsFlags = 1 << 13;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_RBD: ConnectListAllStoragePoolsFlags = 1 << 14;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_SHEEPDOG: ConnectListAllStoragePoolsFlags = 1 << 15;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_GLUSTER: ConnectListAllStoragePoolsFlags = 1 << 16;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_ZFS: ConnectListAllStoragePoolsFlags = 1 << 17;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_VSTORAGE: ConnectListAllStoragePoolsFlags = 1 << 18;

pub type ConnectCredentialType = self::libc::c_uint;
pub const VIR_CRED_USERNAME: ConnectCredentialType = 1;
pub const VIR_CRED_AUTHNAME: ConnectCredentialType = 2;
pub const VIR_CRED_LANGUAGE: ConnectCredentialType = 3;
pub const VIR_CRED_CNONCE: ConnectCredentialType = 4;
pub const VIR_CRED_PASSPHRASE: ConnectCredentialType = 5;
pub const VIR_CRED_ECHOPROMPT: ConnectCredentialType = 6;
pub const VIR_CRED_NOECHOPROMPT: ConnectCredentialType = 7;
pub const VIR_CRED_REALM: ConnectCredentialType = 8;
pub const VIR_CRED_EXTERNAL: ConnectCredentialType = 9;

pub struct ConnectAuth {
    ptr: *const virConnectAuth
}

#[allow(unused_variables)]
extern "C" fn connect_auth_callback_default(cred: virConnectCredentialPtr,
                                            ncred: libc::c_uint,
                                            cbdata: *const libc::c_void) -> libc::c_int {
    // TODO(sahid): needs to provide what we have in libvirt.
    return 0;
}

impl ConnectAuth {
    pub fn as_ptr(&self) -> *const virConnectAuth {
        self.ptr
    }

    pub fn new_default() -> ConnectAuth {
        let auth = virConnectAuth{
            credtype: [VIR_CRED_AUTHNAME,
                       VIR_CRED_ECHOPROMPT,
                       VIR_CRED_REALM,
                       VIR_CRED_PASSPHRASE,
                       VIR_CRED_NOECHOPROMPT,
                       VIR_CRED_EXTERNAL].as_ptr(),
            ncredtype: 6,
            cb: connect_auth_callback_default,
            cbdata: ptr::null(),
        };
        ConnectAuth{ptr: &auth}
    }
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
    /// match Connect::open("test:///default") {
    ///   Ok(conn) => {
    ///       conn.close();
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open(uri: &str) -> Result<Connect, Error> {
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
    /// match Connect::open_read_only("test:///default") {
    ///   Ok(conn) => {
    ///       conn.close();
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open_read_only(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpenReadOnly(CString::new(uri).unwrap().as_ptr());
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect{c: c});
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    /// use virt::connect::ConnectAuth;
    /// 
    /// let auth = ConnectAuth::new_default();
    /// match Connect::open_auth("test:///default", &auth, 0) {
    ///   Ok(conn) => {
    ///       conn.close();
    ///       return
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open_auth(uri: &str, auth: &ConnectAuth, flags: ConnectFlags) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpenAuth(
                CString::new(uri).unwrap().as_ptr(),
                auth.as_ptr(), flags as libc::c_uint);
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
    pub fn get_hostname(&self) -> Result<String, Error> {
        unsafe {
            let n = virConnectGetHostname(self.c);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_capabilities(&self) -> Result<String, Error> {
        unsafe {
            let n = virConnectGetCapabilities(self.c);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
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

    pub fn get_type(&self) -> Result<String, Error> {
        unsafe {
            let t = virConnectGetType(self.c);
            if t.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(t).to_string_lossy().into_owned())
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

    pub fn is_encrypted(&self) -> Result<bool, Error> {
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
    /// match Connect::open("test:///default") {
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
    pub fn list_domains(&self) -> Result<Vec<u32>, Error> {
        unsafe {
            let ids: [libc::c_int; 512] = [0; 512];
            let size = virConnectListDomains(self.c, ids.as_ptr(), 512);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<u32> = Vec::new();
            for x in 0..size as usize {
                array.push(ids[x] as u32);
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_interfaces(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListInterfaces(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_networks(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListNetworks(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
            return Ok(array)
        }
    }

    pub fn list_nw_filters(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListNWFilters(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
            return Ok(array)
        }
    }

    pub fn list_secrets(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListSecrets(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_storage_pools(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListStoragePools(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
            return Ok(array)
        }
    }

    pub fn list_all_domains(&self, flags: ConnectListAllDomainsFlags) -> Result<Vec<Domain>, Error> {
        unsafe {
            let mut domains: *mut virDomainPtr = ptr::null_mut();
            let size = virConnectListAllDomains(
                self.c, &mut domains, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<Domain> = Vec::new();
            for x in 0..size as isize {
                array.push(Domain{d: *domains.offset(x)});
            }
            libc::free(domains as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_networks(&self, flags: ConnectListAllNetworksFlags) -> Result<Vec<Network>, Error> {
        unsafe {
            let mut networks: *mut virNetworkPtr = ptr::null_mut();
            let size = virConnectListAllNetworks(
                self.c, &mut networks, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<Network> = Vec::new();
            for x in 0..size as isize {
                array.push(Network{d: *networks.offset(x)});
            }
            libc::free(networks as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_interfaces(&self, flags: ConnectListAllInterfacesFlags) -> Result<Vec<Interface>, Error> {
        unsafe {
            let mut interfaces: *mut virInterfacePtr = ptr::null_mut();
            let size = virConnectListAllInterfaces(
                self.c, &mut interfaces, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<Interface> = Vec::new();
            for x in 0..size as isize {
                array.push(Interface{d: *interfaces.offset(x)});
            }
            libc::free(interfaces as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_node_devices(&self, flags: ConnectListAllNodeDeviceFlags) -> Result<Vec<NodeDevice>, Error> {
        unsafe {
            let mut nodedevs: *mut virNodeDevicePtr = ptr::null_mut();
            let size = virConnectListAllNodeDevices(
                self.c, &mut nodedevs, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }
            
            let mut array: Vec<NodeDevice> = Vec::new();
            for x in 0..size as isize {
                array.push(NodeDevice{d: *nodedevs.offset(x)});
            }
            libc::free(nodedevs as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_secrets(&self, flags: ConnectListAllSecretsFlags) -> Result<Vec<Secret>, Error> {
        unsafe {
            let mut secrets: *mut virSecretPtr = ptr::null_mut();
            let size = virConnectListAllSecrets(
                self.c, &mut secrets, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<Secret> = Vec::new();
            for x in 0..size as isize {
                array.push(Secret{d: *secrets.offset(x)});
            }
            libc::free(secrets as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_storage_pools(&self, flags: ConnectListAllStoragePoolsFlags) -> Result<Vec<StoragePool>, Error> {
        unsafe {
            let mut storages: *mut virStoragePoolPtr = ptr::null_mut();
            let size = virConnectListAllStoragePools(
                self.c, &mut storages, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<StoragePool> = Vec::new();
            for x in 0..size as isize {
                array.push(StoragePool{d: *storages.offset(x)});
            }
            libc::free(storages as *mut libc::c_void);

            return Ok(array)
        }
    }

    pub fn list_all_nw_filters(&self, flags: u32) -> Result<Vec<NWFilter>, Error> {
        unsafe {
            let mut filters: *mut virNWFilterPtr = ptr::null_mut();
            let size = virConnectListAllNWFilters(
                self.c, &mut filters, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<NWFilter> = Vec::new();
            for x in 0..size as isize {
                array.push(NWFilter{d: *filters.offset(x)});
            }
            libc::free(filters as *mut libc::c_void);

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
    /// match Connect::open("test:///default") {
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
    pub fn list_defined_domains(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedDomains(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_defined_interfaces(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedInterfaces(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_defined_storage_pools(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedStoragePools(
                self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
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
    /// match Connect::open("test:///default") {
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
    pub fn list_defined_networks(&self) -> Result<Vec<&str>, Error> {
        unsafe {
            let names: [*const libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedNetworks(self.c, names.as_ptr(), 1024);
            if size == -1 {
                return Err(Error::new())
            }

            let mut array: Vec<&str> = Vec::new();
            for x in 0..size as usize {
                array.push(str::from_utf8(
                    CStr::from_ptr(names[x]).to_bytes()).unwrap());
            }
            return Ok(array)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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

    pub fn num_of_nw_filters(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfNWFilters(self.c);
            if num == -1 {
                return Err(Error::new())
            }
            return Ok(num as u32)
        }
    }

    pub fn num_of_secrets(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfSecrets(self.c);
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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
    /// match Connect::open("test:///default") {
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

    pub fn domain_lookup_by_uuid_string(&self, id: &str) -> Result<Domain, Error> {
        Domain::lookup_by_uuid_string(self, id)
    }

    pub fn network_lookup_by_id(&self, id: u32) -> Result<Network, Error> {
        Network::lookup_by_id(self, id)
    }

    pub fn network_lookup_by_name(&self, id: &str) -> Result<Network, Error> {
        Network::lookup_by_name(self, id)
    }

    pub fn network_lookup_by_uuid_string(&self, id: &str) -> Result<Network, Error> {
        Network::lookup_by_uuid_string(self, id)
    }

    pub fn interface_lookup_by_id(&self, id: u32) -> Result<Interface, Error> {
        Interface::lookup_by_id(self, id)
    }

    pub fn interface_lookup_by_name(&self, id: &str) -> Result<Interface, Error> {
        Interface::lookup_by_name(self, id)
    }

    pub fn interface_lookup_by_uuid_string(&self, id: &str) -> Result<Interface, Error> {
        Interface::lookup_by_uuid_string(self, id)
    }

    pub fn interface_lookup_by_mac_string(&self, id: &str) -> Result<Interface, Error> {
        Interface::lookup_by_mac_string(self, id)
    }

    pub fn storage_pool_lookup_by_id(&self, id: u32) -> Result<StoragePool, Error> {
        StoragePool::lookup_by_id(self, id)
    }

    pub fn storage_pool_lookup_by_name(&self, id: &str) -> Result<StoragePool, Error> {
        StoragePool::lookup_by_name(self, id)
    }

    pub fn storage_pool_lookup_by_uuid_string(&self, id: &str) -> Result<StoragePool, Error> {
        StoragePool::lookup_by_uuid_string(self, id)
    }

    pub fn nodedev_lookup_by_name(&self, id: &str) -> Result<NodeDevice, Error> {
        NodeDevice::lookup_by_name(self, id)
    }
}
