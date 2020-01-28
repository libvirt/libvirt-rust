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

use std::{mem, ptr, str};

use domain::sys::{virDomainPtr, virDomainStatsRecordPtr};
use interface::sys::virInterfacePtr;
use network::sys::virNetworkPtr;
use nodedev::sys::virNodeDevicePtr;
use nwfilter::sys::virNWFilterPtr;
use secret::sys::virSecretPtr;
use storage_pool::sys::virStoragePoolPtr;

use domain::{Domain, DomainStatsRecord};
use error::Error;
use interface::Interface;
use network::Network;
use nodedev::NodeDevice;
use nwfilter::NWFilter;
use secret::Secret;
use storage_pool::StoragePool;

pub mod sys {
    extern crate libc;

    #[repr(C)]
    pub struct virConnect {}

    pub type virConnectPtr = *mut virConnect;

    #[repr(C)]
    pub struct virConnectCredential {
        pub typed: libc::c_int,
        pub prompt: *const libc::c_char,
        pub challenge: *const libc::c_char,
        pub defresult: *const libc::c_char,
        pub result: *mut libc::c_char,
        pub resultlen: libc::c_uint,
    }

    pub type virConnectCredentialPtr = *mut virConnectCredential;

    pub type virConnectAuthCallbackPtr =
        unsafe extern "C" fn(virConnectCredentialPtr, libc::c_uint, *mut libc::c_void) -> i32;

    #[repr(C)]
    pub struct virConnectAuth {
        pub credtype: *mut libc::c_int,
        pub ncredtype: libc::c_uint,
        pub cb: virConnectAuthCallbackPtr,
        pub cbdata: *mut libc::c_void,
    }

    pub type virConnectAuthPtr = *mut virConnectAuth;

    #[repr(C)]
    #[derive(Default)]
    pub struct virNodeInfo {
        pub model: [libc::c_char; 32],
        pub memory: libc::c_ulong,
        pub cpus: libc::c_uint,
        pub mhz: libc::c_uint,
        pub nodes: libc::c_uint,
        pub sockets: libc::c_uint,
        pub cores: libc::c_uint,
        pub threads: libc::c_uint,
    }

    pub type virNodeInfoPtr = *mut virNodeInfo;
}

#[link(name = "virt")]
extern "C" {
    fn virGetVersion(
        hyver: *const libc::c_ulong,
        ctype: *const libc::c_char,
        typever: *const libc::c_ulong,
    ) -> libc::c_int;
    fn virConnectOpen(uri: *const libc::c_char) -> sys::virConnectPtr;
    fn virConnectOpenReadOnly(uri: *const libc::c_char) -> sys::virConnectPtr;
    fn virConnectOpenAuth(
        uri: *const libc::c_char,
        auth: sys::virConnectAuthPtr,
        flags: libc::c_uint,
    ) -> sys::virConnectPtr;
    fn virConnectClose(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectGetVersion(ptr: sys::virConnectPtr, hyver: *mut libc::c_ulong) -> libc::c_int;
    fn virConnectGetHostname(ptr: sys::virConnectPtr) -> *mut libc::c_char;
    fn virConnectGetCapabilities(ptr: sys::virConnectPtr) -> *mut libc::c_char;
    fn virConnectGetLibVersion(ptr: sys::virConnectPtr, ver: *mut libc::c_ulong) -> libc::c_int;
    fn virConnectGetType(ptr: sys::virConnectPtr) -> *const libc::c_char;
    fn virConnectGetURI(ptr: sys::virConnectPtr) -> *mut libc::c_char;
    fn virConnectGetSysinfo(ptr: sys::virConnectPtr, flags: libc::c_uint) -> *mut libc::c_char;
    fn virConnectIsAlive(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectIsEncrypted(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectIsSecure(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectListDomains(
        ptr: sys::virConnectPtr,
        ids: *mut libc::c_int,
        maxids: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListDefinedDomains(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListInterfaces(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListNetworks(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListNWFilters(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListStoragePools(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListSecrets(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnames: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListDefinedInterfaces(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxifaces: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListDefinedNetworks(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxnets: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListDefinedStoragePools(
        ptr: sys::virConnectPtr,
        names: *mut *mut libc::c_char,
        maxpools: libc::c_int,
    ) -> libc::c_int;
    fn virConnectListAllDomains(
        ptr: sys::virConnectPtr,
        domains: *mut *mut virDomainPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllNetworks(
        ptr: sys::virConnectPtr,
        networks: *mut *mut virNetworkPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllInterfaces(
        ptr: sys::virConnectPtr,
        interfaces: *mut *mut virInterfacePtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllNodeDevices(
        ptr: sys::virConnectPtr,
        devices: *mut *mut virNodeDevicePtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllSecrets(
        ptr: sys::virConnectPtr,
        secrets: *mut *mut virSecretPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllNWFilters(
        ptr: sys::virConnectPtr,
        nwfilters: *mut *mut virNWFilterPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectListAllStoragePools(
        ptr: sys::virConnectPtr,
        storages: *mut *mut virStoragePoolPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectNumOfDomains(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfInterfaces(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfNetworks(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfStoragePools(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfNWFilters(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfSecrets(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedDomains(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedInterfaces(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedNetworks(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectNumOfDefinedStoragePools(ptr: sys::virConnectPtr) -> libc::c_int;
    fn virConnectGetCPUModelNames(
        ptr: sys::virConnectPtr,
        arch: *const libc::c_char,
        mcpus: *mut *mut *mut libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectGetMaxVcpus(ptr: sys::virConnectPtr, attr: *const libc::c_char) -> libc::c_int;
    fn virConnectCompareCPU(
        ptr: sys::virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virNodeGetInfo(ptr: sys::virConnectPtr, ninfo: sys::virNodeInfoPtr) -> libc::c_int;
    fn virNodeGetFreeMemory(ptr: sys::virConnectPtr) -> libc::c_long;
    fn virConnectSetKeepAlive(
        ptr: sys::virConnectPtr,
        interval: libc::c_int,
        count: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectDomainXMLFromNative(
        ptr: sys::virConnectPtr,
        nformat: *const libc::c_char,
        nconfig: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virConnectDomainXMLToNative(
        ptr: sys::virConnectPtr,
        nformat: *const libc::c_char,
        dxml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virConnectGetDomainCapabilities(
        ptr: sys::virConnectPtr,
        emulatorbin: *const libc::c_char,
        arch: *const libc::c_char,
        machine: *const libc::c_char,
        virttype: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virConnectGetAllDomainStats(
        ptr: sys::virConnectPtr,
        stats: libc::c_uint,
        ret: *mut *mut virDomainStatsRecordPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virConnectBaselineCPU(
        ptr: sys::virConnectPtr,
        xmlcpus: *const *const libc::c_char,
        ncpus: libc::c_uint,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virConnectFindStoragePoolSources(
        ptr: sys::virConnectPtr,
        kind: *const libc::c_char,
        spec: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
}

extern "C" fn connectCallback(
    ccreds: sys::virConnectCredentialPtr,
    ncred: libc::c_uint,
    cbdata: *mut libc::c_void,
) -> libc::c_int {
    let callback: ConnectAuthCallback = unsafe {
        // Safe because connectCallback is private and only used by
        // Connect::open_auth(). In open_auth() we transmute the
        // callback allocate in *void.
        mem::transmute(cbdata)
    };
    let mut rcreds: Vec<ConnectCredential> = Vec::new();
    for i in 0..ncred as isize {
        unsafe {
            // Safe because ccreds is allocated.
            let c = ConnectCredential::from_ptr(ccreds.offset(i));
            rcreds.push(c);
        }
    }
    callback(&mut rcreds);
    for i in 0..ncred as isize {
        if rcreds[i as usize].result.is_some() {
            if let Some(ref result) = rcreds[i as usize].result {
                unsafe {
                    // Safe because ccreds is allocated and the result
                    // is comming from Rust calls.
                    (*ccreds.offset(i)).resultlen = result.len() as libc::c_uint;
                    (*ccreds.offset(i)).result = string_to_mut_c_chars!(result.clone());
                }
            }
        }
    }
    0
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
pub const VIR_CONNECT_LIST_SECRETS_NO_PRIVATE: ConnectListAllSecretsFlags = 1 << 3;

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

pub type ConnectCompareCPUFlags = self::libc::c_uint;
pub const VIR_CONNECT_COMPARE_CPU_FAIL_INCOMPATIBLE: ConnectCompareCPUFlags = 1 << 0;

pub type CPUCompareResult = self::libc::c_int;
pub const VIR_CPU_COMPARE_ERROR: CPUCompareResult = -1;
pub const VIR_CPU_COMPARE_INCOMPATIBLE: CPUCompareResult = 0;
pub const VIR_CPU_COMPARE_IDENTICAL: CPUCompareResult = 1;
pub const VIR_CPU_COMPARE_SUPERSET: CPUCompareResult = 2;

pub type BaselineCPUFlags = self::libc::c_int;
pub const VIR_CONNECT_BASELINE_CPU_EXPAND_FEATURES: BaselineCPUFlags = (1 << 0);
pub const VIR_CONNECT_BASELINE_CPU_MIGRATABLE: BaselineCPUFlags = (1 << 1);

pub type ConnectCredentialType = self::libc::c_int;
pub const VIR_CRED_USERNAME: ConnectCredentialType = 1;
pub const VIR_CRED_AUTHNAME: ConnectCredentialType = 2;
pub const VIR_CRED_LANGUAGE: ConnectCredentialType = 3;
pub const VIR_CRED_CNONCE: ConnectCredentialType = 4;
pub const VIR_CRED_PASSPHRASE: ConnectCredentialType = 5;
pub const VIR_CRED_ECHOPROMPT: ConnectCredentialType = 6;
pub const VIR_CRED_NOECHOPROMPT: ConnectCredentialType = 7;
pub const VIR_CRED_REALM: ConnectCredentialType = 8;
pub const VIR_CRED_EXTERNAL: ConnectCredentialType = 9;

#[derive(Clone, Debug)]
pub struct NodeInfo {
    /// Indicating the CPU model.
    pub model: String,
    /// Memory size in kilobytes.
    pub memory: u64,
    /// The number of active CPUs.
    pub cpus: u32,
    /// expected CPU frequency, 0 if not known or on unusual
    /// architectures.
    pub mhz: u32,
    /// The number of NUMA cell, 1 for unusual NUMA topologies or
    /// uniform memory access; check capabilities XML for the actual
    /// NUMA topology
    pub nodes: u32,
    /// Number of CPU sockets per node if nodes > 1, 1 in case of
    /// unusual NUMA topology.
    pub sockets: u32,
    /// Number of cores per socket, total number of processors in case
    /// of unusual NUMA topology
    pub cores: u32,
    /// Number of threads per core, 1 in case of unusual numa topology
    pub threads: u32,
}

// TODO(sahid): should support closure
pub type ConnectAuthCallback = fn(creds: &mut Vec<ConnectCredential>);

#[derive(Clone, Debug)]
pub struct ConnectCredential {
    /// One of `ConnectCredentialType` constants
    pub typed: i32,
    /// Prompt to show to user.
    pub prompt: String,
    /// Additional challenge to show.
    pub challenge: String,
    /// Optional default result.
    pub def_result: String,
    /// Result to be filled with user response (or def_result).
    pub result: Option<String>,
}

impl ConnectCredential {
    pub fn from_ptr(cred: sys::virConnectCredentialPtr) -> ConnectCredential {
        unsafe {
            let mut default: String = String::from("");
            if !(*cred).defresult.is_null() {
                default = c_chars_to_string!((*cred).defresult, nofree);
            }
            ConnectCredential {
                typed: (*cred).typed,
                prompt: c_chars_to_string!((*cred).prompt, nofree),
                challenge: c_chars_to_string!((*cred).challenge, nofree),
                def_result: default,
                result: None,
            }
        }
    }
}

pub struct ConnectAuth {
    /// List of supported `ConnectCredentialType` values.
    creds: Vec<ConnectCredentialType>,
    /// Callback used to collect credentials.
    callback: ConnectAuthCallback,
}

impl ConnectAuth {
    pub fn new(creds: Vec<ConnectCredentialType>, callback: ConnectAuthCallback) -> ConnectAuth {
        ConnectAuth {
            creds: creds,
            callback: callback,
        }
    }
}

/// Provides APIs for the management of hosts.
///
/// See http://libvirt.org/html/libvirt-libvirt-host.html
#[derive(Debug)]
pub struct Connect {
    ptr: Option<sys::virConnectPtr>,
}

impl Connect {
    pub fn as_ptr(&self) -> sys::virConnectPtr {
        self.ptr.unwrap()
    }

    pub fn new(ptr: sys::virConnectPtr) -> Connect {
        return Connect { ptr: Some(ptr) };
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
    ///   Ok(mut conn) => {
    ///       assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpen(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(c));
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
    ///   Ok(mut conn) => {
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open_read_only(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = virConnectOpenReadOnly(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(c));
        }
    }

    pub fn open_auth(
        uri: &str,
        auth: &mut ConnectAuth,
        flags: ConnectFlags,
    ) -> Result<Connect, Error> {
        let mut cauth = unsafe {
            // Safe because Rust forces to allocate all attributes of
            // the struct ConnectAuth.
            sys::virConnectAuth {
                credtype: &mut auth.creds[0],
                ncredtype: auth.creds.len() as libc::c_uint,
                cb: connectCallback,
                cbdata: mem::transmute(auth.callback),
            }
        };
        let c = unsafe {
            virConnectOpenAuth(string_to_c_chars!(uri), &mut cauth, flags as libc::c_uint)
        };
        if c.is_null() {
            return Err(Error::new());
        }
        return Ok(Connect::new(c));
    }

    /// This function closes the connection to the hypervisor. This
    /// should not be called if further interaction with the
    /// hypervisor are needed especially if there is running domain
    /// which need further monitoring by the application.
    pub fn close(&mut self) -> Result<i32, Error> {
        unsafe {
            let ret = virConnectClose(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            if ret == 0 {
                self.ptr = None;
            }
            Ok(ret)
        }
    }

    /// This returns a system hostname on which the hypervisor is
    /// running (based on the result of the gethostname system call,
    /// but possibly expanded to a fully-qualified domain name via
    /// getaddrinfo).  If we are connected to a remote system, then
    /// this returns the hostname of the remote system.
    pub fn get_hostname(&self) -> Result<String, Error> {
        unsafe {
            let n = virConnectGetHostname(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_capabilities(&self) -> Result<String, Error> {
        unsafe {
            let n = virConnectGetCapabilities(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_lib_version(&self) -> Result<u32, Error> {
        unsafe {
            let mut ver: libc::c_ulong = 0;
            if virConnectGetLibVersion(self.as_ptr(), &mut ver) == -1 {
                return Err(Error::new());
            }
            return Ok(ver as u32);
        }
    }

    pub fn get_type(&self) -> Result<String, Error> {
        unsafe {
            let t = virConnectGetType(self.as_ptr());
            if t.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(t, nofree));
        }
    }

    pub fn get_uri(&self) -> Result<String, Error> {
        unsafe {
            let t = virConnectGetURI(self.as_ptr());
            if t.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(t));
        }
    }

    pub fn get_sys_info(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let sys = virConnectGetSysinfo(self.as_ptr(), flags as libc::c_uint);
            if sys.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(sys));
        }
    }

    pub fn get_max_vcpus(&self, attr: &str) -> Result<u32, Error> {
        unsafe {
            let max = virConnectGetMaxVcpus(self.as_ptr(), string_to_c_chars!(attr));
            if max == -1 {
                return Err(Error::new());
            }
            return Ok(max as u32);
        }
    }

    pub fn get_cpu_models_names(&self, arch: &str, flags: u32) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: *mut *mut libc::c_char = ptr::null_mut();
            let size = virConnectGetCPUModelNames(
                self.as_ptr(),
                string_to_c_chars!(arch),
                &mut names,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as isize {
                array.push(c_chars_to_string!(*names.offset(x)));
            }
            libc::free(names as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn is_alive(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsAlive(self.as_ptr());
            if t == -1 {
                return Err(Error::new());
            }
            return Ok(t == 1);
        }
    }

    pub fn is_encrypted(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsEncrypted(self.as_ptr());
            if t == -1 {
                return Err(Error::new());
            }
            return Ok(t == 1);
        }
    }

    pub fn is_secure(&self) -> Result<bool, Error> {
        unsafe {
            let t = virConnectIsSecure(self.as_ptr());
            if t == -1 {
                return Err(Error::new());
            }
            return Ok(t == 1);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_domains() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_domains(&self) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut ids: [libc::c_int; 512] = [0; 512];
            let size = virConnectListDomains(self.as_ptr(), ids.as_mut_ptr(), 512);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<u32> = Vec::new();
            for x in 0..size as usize {
                array.push(ids[x] as u32);
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_interfaces() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_interfaces(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListInterfaces(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_networks() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_networks(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListNetworks(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
        }
    }

    pub fn list_nw_filters(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListNWFilters(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
        }
    }

    pub fn list_secrets(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListSecrets(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_storage_pools() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_storage_pools(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListStoragePools(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
        }
    }

    pub fn list_all_domains(
        &self,
        flags: ConnectListAllDomainsFlags,
    ) -> Result<Vec<Domain>, Error> {
        unsafe {
            let mut domains: *mut virDomainPtr = ptr::null_mut();
            let size = virConnectListAllDomains(self.as_ptr(), &mut domains, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            mem::forget(domains);

            let mut array: Vec<Domain> = Vec::new();
            for x in 0..size as isize {
                array.push(Domain::new(*domains.offset(x)));
            }
            libc::free(domains as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_networks(
        &self,
        flags: ConnectListAllNetworksFlags,
    ) -> Result<Vec<Network>, Error> {
        unsafe {
            let mut networks: *mut virNetworkPtr = ptr::null_mut();
            let size =
                virConnectListAllNetworks(self.as_ptr(), &mut networks, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<Network> = Vec::new();
            for x in 0..size as isize {
                array.push(Network::new(*networks.offset(x)));
            }
            libc::free(networks as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_interfaces(
        &self,
        flags: ConnectListAllInterfacesFlags,
    ) -> Result<Vec<Interface>, Error> {
        unsafe {
            let mut interfaces: *mut virInterfacePtr = ptr::null_mut();
            let size =
                virConnectListAllInterfaces(self.as_ptr(), &mut interfaces, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<Interface> = Vec::new();
            for x in 0..size as isize {
                array.push(Interface::new(*interfaces.offset(x)));
            }
            libc::free(interfaces as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_node_devices(
        &self,
        flags: ConnectListAllNodeDeviceFlags,
    ) -> Result<Vec<NodeDevice>, Error> {
        unsafe {
            let mut nodedevs: *mut virNodeDevicePtr = ptr::null_mut();
            let size =
                virConnectListAllNodeDevices(self.as_ptr(), &mut nodedevs, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<NodeDevice> = Vec::new();
            for x in 0..size as isize {
                array.push(NodeDevice::new(*nodedevs.offset(x)));
            }
            libc::free(nodedevs as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_secrets(
        &self,
        flags: ConnectListAllSecretsFlags,
    ) -> Result<Vec<Secret>, Error> {
        unsafe {
            let mut secrets: *mut virSecretPtr = ptr::null_mut();
            let size = virConnectListAllSecrets(self.as_ptr(), &mut secrets, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<Secret> = Vec::new();
            for x in 0..size as isize {
                array.push(Secret::new(*secrets.offset(x)));
            }
            libc::free(secrets as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_storage_pools(
        &self,
        flags: ConnectListAllStoragePoolsFlags,
    ) -> Result<Vec<StoragePool>, Error> {
        unsafe {
            let mut storages: *mut virStoragePoolPtr = ptr::null_mut();
            let size =
                virConnectListAllStoragePools(self.as_ptr(), &mut storages, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<StoragePool> = Vec::new();
            for x in 0..size as isize {
                array.push(StoragePool::new(*storages.offset(x)));
            }
            libc::free(storages as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_all_nw_filters(&self, flags: u32) -> Result<Vec<NWFilter>, Error> {
        unsafe {
            let mut filters: *mut virNWFilterPtr = ptr::null_mut();
            let size =
                virConnectListAllNWFilters(self.as_ptr(), &mut filters, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<NWFilter> = Vec::new();
            for x in 0..size as isize {
                array.push(NWFilter::new(*filters.offset(x)));
            }
            libc::free(filters as *mut libc::c_void);

            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_defined_domains() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_domains(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedDomains(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_defined_interfaces() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_interfaces(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedInterfaces(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_defined_storage_pools() {
    ///       Ok(arr) => assert_eq!(0, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_storage_pools(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedStoragePools(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
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
    ///   Ok(mut conn) => {
    ///     match conn.list_networks() {
    ///       Ok(arr) => assert_eq!(1, arr.len()),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn list_defined_networks(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = virConnectListDefinedNetworks(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_domains() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDomains(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_interfaces() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfInterfaces(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_networks() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfNetworks(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_storage_pools() {
    ///       Ok(n) => assert_eq!(1, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfStoragePools(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    pub fn num_of_nw_filters(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfNWFilters(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    pub fn num_of_secrets(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfSecrets(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_defined_domains() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedDomains(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_defined_interfaces() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedInterfaces(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_defined_networks() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedNetworks(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// match Connect::open("test:///default") {
    ///   Ok(mut conn) => {
    ///     match conn.num_of_defined_storage_pools() {
    ///       Ok(n) => assert_eq!(0, n),
    ///       Err(e) => panic!(
    ///         "failed with code {}, message: {}", e.code, e.message)
    ///     }
    ///     assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    ///   }
    /// ```
    pub fn num_of_defined_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = virConnectNumOfDefinedStoragePools(self.as_ptr());
            if num == -1 {
                return Err(Error::new());
            }
            return Ok(num as u32);
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
    ///   Ok(mut conn) => {
    ///       match conn.get_hyp_version() {
    ///         Ok(hyver) => assert_eq!(2, hyver),
    ///         Err(e) => panic!(
    ///           "failed with code {}, message: {}", e.code, e.message)
    ///       }
    ///       assert_eq!(Ok(0), conn.close());
    ///   },
    ///   Err(e) => panic!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn get_hyp_version(&self) -> Result<u32, Error> {
        unsafe {
            let mut hyver: libc::c_ulong = 0;
            if virConnectGetVersion(self.as_ptr(), &mut hyver) == -1 {
                return Err(Error::new());
            }
            return Ok(hyver as u32);
        }
    }

    pub fn compare_cpu(
        &self,
        xml: &str,
        flags: ConnectCompareCPUFlags,
    ) -> Result<CPUCompareResult, Error> {
        unsafe {
            let res = virConnectCompareCPU(
                self.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if res == VIR_CPU_COMPARE_ERROR {
                return Err(Error::new());
            }
            return Ok(res as CPUCompareResult);
        }
    }

    pub fn get_free_memory(&self) -> Result<u64, Error> {
        unsafe {
            let res = virNodeGetFreeMemory(self.as_ptr());
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(res as u64);
        }
    }

    pub fn get_node_info(&self) -> Result<NodeInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virNodeInfo::default();
            let res = virNodeGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(NodeInfo {
                model: c_chars_to_string!((*pinfo).model.as_ptr(), nofree),
                memory: (*pinfo).memory as u64,
                cpus: (*pinfo).cpus as u32,
                mhz: (*pinfo).mhz as u32,
                nodes: (*pinfo).nodes as u32,
                sockets: (*pinfo).sockets as u32,
                cores: (*pinfo).cores as u32,
                threads: (*pinfo).threads as u32,
            });
        }
    }

    pub fn set_keep_alive(&self, interval: i32, count: u32) -> Result<i32, Error> {
        unsafe {
            let ret = virConnectSetKeepAlive(
                self.as_ptr(),
                interval as libc::c_int,
                count as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret as i32)
        }
    }

    pub fn domain_xml_from_native(
        &self,
        nformat: &str,
        nconfig: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let ret = virConnectDomainXMLFromNative(
                self.as_ptr(),
                string_to_c_chars!(nformat),
                string_to_c_chars!(nconfig),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(ret))
        }
    }

    pub fn domain_xml_to_native(
        &self,
        nformat: &str,
        dxml: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let ret = virConnectDomainXMLToNative(
                self.as_ptr(),
                string_to_c_chars!(nformat),
                string_to_c_chars!(dxml),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(ret))
        }
    }

    pub fn get_domain_capabilities(
        &self,
        emulatorbin: &str,
        arch: &str,
        machine: &str,
        virttype: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let ret = virConnectGetDomainCapabilities(
                self.as_ptr(),
                string_to_c_chars!(emulatorbin),
                string_to_c_chars!(arch),
                string_to_c_chars!(machine),
                string_to_c_chars!(virttype),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(ret))
        }
    }

    pub fn get_all_domain_stats(
        &self,
        stats: u32,
        flags: u32,
    ) -> Result<Vec<DomainStatsRecord>, Error> {
        unsafe {
            let mut record: *mut virDomainStatsRecordPtr = ptr::null_mut();
            let size = virConnectGetAllDomainStats(
                self.as_ptr(),
                stats as libc::c_uint,
                &mut record,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::new());
            }

            let mut array: Vec<DomainStatsRecord> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainStatsRecord {
                    ptr: *record.offset(x),
                });
            }
            libc::free(record as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn baseline_cpu(&self, xmlcpus: &[&str], flags: BaselineCPUFlags) -> Result<String, Error> {
        unsafe {
            let mut xcpus: [*const libc::c_char; 512] = [ptr::null_mut(); 512];
            for x in 0..xmlcpus.len() {
                xcpus[x] = string_to_c_chars!(xmlcpus[x]);
            }
            let ret = virConnectBaselineCPU(
                self.as_ptr(),
                xcpus.as_ptr(),
                xmlcpus.len() as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::new());
            }
            Ok(c_chars_to_string!(ret))
        }
    }

    pub fn find_storage_pool_sources(
        &self,
        kind: &str,
        spec: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let n = virConnectFindStoragePoolSources(
                self.as_ptr(),
                string_to_c_chars!(kind),
                string_to_c_chars!(spec),
                flags as libc::c_uint,
            );
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }
}
