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

use std::ffi::CString;
use std::{mem, ptr, str};

use crate::domain::{Domain, DomainStatsRecord};
use crate::error::Error;
use crate::interface::Interface;
use crate::network::Network;
use crate::nodedev::NodeDevice;
use crate::nwfilter::NWFilter;
use crate::secret::Secret;
use crate::storage_pool::StoragePool;
use crate::util::c_ulong_to_u64;

extern "C" fn connect_callback(
    ccreds: sys::virConnectCredentialPtr,
    ncred: libc::c_uint,
    cbdata: *mut libc::c_void,
) -> libc::c_int {
    let callback: ConnectAuthCallback = unsafe {
        // Safe because connect_callback is private and only used by
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
                    // libvirt will call free() on 'result', so we must provide
                    // memory allocated by the C malloc impl
                    let bytes = result.as_bytes();
                    let buffer = ::libc::malloc(bytes.len() + 1);
                    ::std::ptr::copy(bytes.as_ptr().cast(), buffer, bytes.len());
                    ::std::ptr::write(buffer.add(bytes.len()) as *mut u8, 0u8);

                    // Safe because ccreds is allocated and the result
                    // is comming from Rust calls.
                    (*ccreds.offset(i)).resultlen = result.len() as libc::c_uint;
                    (*ccreds.offset(i)).result = buffer as *mut i8;
                }
            }
        }
    }
    0
}

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
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(cred: sys::virConnectCredentialPtr) -> ConnectCredential {
        let mut default: String = String::from("");
        if !(*cred).defresult.is_null() {
            default = c_chars_to_string!((*cred).defresult, nofree);
        }
        ConnectCredential {
            typed: (*cred).type_,
            prompt: c_chars_to_string!((*cred).prompt, nofree),
            challenge: c_chars_to_string!((*cred).challenge, nofree),
            def_result: default,
            result: None,
        }
    }
}

pub struct ConnectAuth {
    /// List of supported `ConnectCredentialType` values.
    creds: Vec<sys::virConnectCredentialType>,
    /// Callback used to collect credentials.
    callback: ConnectAuthCallback,
}

impl ConnectAuth {
    pub fn new(
        creds: Vec<sys::virConnectCredentialType>,
        callback: ConnectAuthCallback,
    ) -> ConnectAuth {
        ConnectAuth { creds, callback }
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
        Connect { ptr: Some(ptr) }
    }

    pub fn get_version() -> Result<u32, Error> {
        unsafe {
            let mut ver: libc::c_ulong = 0;
            if sys::virGetVersion(&mut ver, ptr::null(), ptr::null_mut()) == -1 {
                return Err(Error::last_error());
            }
            Ok(ver as u32)
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
    /// ```
    pub fn open(uri: &str) -> Result<Connect, Error> {
        let uri_buf = CString::new(uri).unwrap();
        unsafe {
            let c = sys::virConnectOpen(uri_buf.as_ptr());
            if c.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(c))
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
    pub fn open_read_only(uri: &str) -> Result<Connect, Error> {
        let uri_buf = CString::new(uri).unwrap();
        unsafe {
            let c = sys::virConnectOpenReadOnly(uri_buf.as_ptr());
            if c.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(c))
        }
    }

    pub fn open_auth(
        uri: &str,
        auth: &mut ConnectAuth,
        flags: sys::virConnectFlags,
    ) -> Result<Connect, Error> {
        let mut cauth =
            // Safe because Rust forces to allocate all attributes of
            // the struct ConnectAuth.
            sys::virConnectAuth {
                credtype: auth.creds.as_mut_ptr() as *mut libc::c_int,
                ncredtype: auth.creds.len() as libc::c_uint,
                cb: Some(connect_callback),
                cbdata: auth.callback as *mut _,
        };
        let uri_buf = CString::new(uri).unwrap();
        unsafe {
            let c = sys::virConnectOpenAuth(uri_buf.as_ptr(), &mut cauth, flags as libc::c_uint);
            if c.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(c))
        }
    }

    /// This function closes the connection to the hypervisor. This
    /// should not be called if further interaction with the
    /// hypervisor are needed especially if there is running domain
    /// which need further monitoring by the application.
    pub fn close(&mut self) -> Result<i32, Error> {
        unsafe {
            let ret = sys::virConnectClose(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
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
            let n = sys::virConnectGetHostname(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn get_capabilities(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virConnectGetCapabilities(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn get_lib_version(&self) -> Result<u32, Error> {
        unsafe {
            let mut ver: libc::c_ulong = 0;
            if sys::virConnectGetLibVersion(self.as_ptr(), &mut ver) == -1 {
                return Err(Error::last_error());
            }
            Ok(ver as u32)
        }
    }

    pub fn get_type(&self) -> Result<String, Error> {
        unsafe {
            let t = sys::virConnectGetType(self.as_ptr());
            if t.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(t, nofree))
        }
    }

    pub fn get_uri(&self) -> Result<String, Error> {
        unsafe {
            let t = sys::virConnectGetURI(self.as_ptr());
            if t.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(t))
        }
    }

    pub fn get_sys_info(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let sys = sys::virConnectGetSysinfo(self.as_ptr(), flags as libc::c_uint);
            if sys.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(sys))
        }
    }

    pub fn get_max_vcpus(&self, attr: &str) -> Result<u32, Error> {
        let attr_buf = CString::new(attr).unwrap();
        unsafe {
            let max = sys::virConnectGetMaxVcpus(self.as_ptr(), attr_buf.as_ptr());
            if max == -1 {
                return Err(Error::last_error());
            }
            Ok(max as u32)
        }
    }

    pub fn get_cpu_models_names(&self, arch: &str, flags: u32) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: *mut *mut libc::c_char = ptr::null_mut();
            let arch_buf = CString::new(arch).unwrap();
            let size = sys::virConnectGetCPUModelNames(
                self.as_ptr(),
                arch_buf.as_ptr(),
                &mut names,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as isize {
                array.push(c_chars_to_string!(*names.offset(x)));
            }
            libc::free(names as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn is_alive(&self) -> Result<bool, Error> {
        unsafe {
            let t = sys::virConnectIsAlive(self.as_ptr());
            if t == -1 {
                return Err(Error::last_error());
            }
            Ok(t == 1)
        }
    }

    pub fn is_encrypted(&self) -> Result<bool, Error> {
        unsafe {
            let t = sys::virConnectIsEncrypted(self.as_ptr());
            if t == -1 {
                return Err(Error::last_error());
            }
            Ok(t == 1)
        }
    }

    pub fn is_secure(&self) -> Result<bool, Error> {
        unsafe {
            let t = sys::virConnectIsSecure(self.as_ptr());
            if t == -1 {
                return Err(Error::last_error());
            }
            Ok(t == 1)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let domains = conn.list_domains().unwrap();
    /// assert_eq!(domains.len(), 1);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_domains(&self) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut ids: [libc::c_int; 512] = [0; 512];
            let size = sys::virConnectListDomains(self.as_ptr(), ids.as_mut_ptr(), 512);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<u32> = Vec::new();
            for x in 0..size as usize {
                array.push(ids[x] as u32);
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let ifaces = conn.list_interfaces().unwrap();
    /// assert_eq!(ifaces.len(), 1);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_interfaces(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListInterfaces(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let networks = conn.list_networks().unwrap();
    /// assert_eq!(networks.len(), 1);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_networks(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListNetworks(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn list_nw_filters(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListNWFilters(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn list_secrets(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListSecrets(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let pools = conn.list_storage_pools().unwrap();
    /// assert_eq!(pools.len(), 1);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_storage_pools(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListStoragePools(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    pub fn list_all_domains(
        &self,
        flags: sys::virConnectListAllDomainsFlags,
    ) -> Result<Vec<Domain>, Error> {
        unsafe {
            let mut domains: *mut sys::virDomainPtr = ptr::null_mut();
            let size =
                sys::virConnectListAllDomains(self.as_ptr(), &mut domains, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<Domain> = Vec::new();
            for x in 0..size as isize {
                array.push(Domain::new(*domains.offset(x)));
            }
            libc::free(domains as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_networks(
        &self,
        flags: sys::virConnectListAllNetworksFlags,
    ) -> Result<Vec<Network>, Error> {
        unsafe {
            let mut networks: *mut sys::virNetworkPtr = ptr::null_mut();
            let size =
                sys::virConnectListAllNetworks(self.as_ptr(), &mut networks, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<Network> = Vec::new();
            for x in 0..size as isize {
                array.push(Network::new(*networks.offset(x)));
            }
            libc::free(networks as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_interfaces(
        &self,
        flags: sys::virConnectListAllInterfacesFlags,
    ) -> Result<Vec<Interface>, Error> {
        unsafe {
            let mut interfaces: *mut sys::virInterfacePtr = ptr::null_mut();
            let size = sys::virConnectListAllInterfaces(
                self.as_ptr(),
                &mut interfaces,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<Interface> = Vec::new();
            for x in 0..size as isize {
                array.push(Interface::new(*interfaces.offset(x)));
            }
            libc::free(interfaces as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_node_devices(
        &self,
        flags: sys::virConnectListAllNodeDeviceFlags,
    ) -> Result<Vec<NodeDevice>, Error> {
        unsafe {
            let mut nodedevs: *mut sys::virNodeDevicePtr = ptr::null_mut();
            let size = sys::virConnectListAllNodeDevices(
                self.as_ptr(),
                &mut nodedevs,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<NodeDevice> = Vec::new();
            for x in 0..size as isize {
                array.push(NodeDevice::new(*nodedevs.offset(x)));
            }
            libc::free(nodedevs as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_secrets(
        &self,
        flags: sys::virConnectListAllSecretsFlags,
    ) -> Result<Vec<Secret>, Error> {
        unsafe {
            let mut secrets: *mut sys::virSecretPtr = ptr::null_mut();
            let size =
                sys::virConnectListAllSecrets(self.as_ptr(), &mut secrets, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<Secret> = Vec::new();
            for x in 0..size as isize {
                array.push(Secret::new(*secrets.offset(x)));
            }
            libc::free(secrets as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_storage_pools(
        &self,
        flags: sys::virConnectListAllStoragePoolsFlags,
    ) -> Result<Vec<StoragePool>, Error> {
        unsafe {
            let mut storages: *mut sys::virStoragePoolPtr = ptr::null_mut();
            let size = sys::virConnectListAllStoragePools(
                self.as_ptr(),
                &mut storages,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<StoragePool> = Vec::new();
            for x in 0..size as isize {
                array.push(StoragePool::new(*storages.offset(x)));
            }
            libc::free(storages as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn list_all_nw_filters(&self, flags: u32) -> Result<Vec<NWFilter>, Error> {
        unsafe {
            let mut filters: *mut sys::virNWFilterPtr = ptr::null_mut();
            let size =
                sys::virConnectListAllNWFilters(self.as_ptr(), &mut filters, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<NWFilter> = Vec::new();
            for x in 0..size as isize {
                array.push(NWFilter::new(*filters.offset(x)));
            }
            libc::free(filters as *mut libc::c_void);

            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let domains = conn.list_defined_domains().unwrap();
    /// assert_eq!(domains.len(), 0);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_defined_domains(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListDefinedDomains(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let ifaces = conn.list_defined_interfaces().unwrap();
    /// assert_eq!(ifaces.len(), 0);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_defined_interfaces(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size =
                sys::virConnectListDefinedInterfaces(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let pools = conn.list_defined_storage_pools().unwrap();
    /// assert_eq!(pools.len(), 0);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_defined_storage_pools(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size =
                sys::virConnectListDefinedStoragePools(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let networks = conn.list_defined_networks().unwrap();
    /// assert_eq!(networks.len(), 0);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn list_defined_networks(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListDefinedNetworks(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            Ok(array)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_domains = conn.num_of_domains().unwrap();
    /// assert_eq!(num_domains, 1);
    /// ```
    pub fn num_of_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfDomains(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_ifaces = conn.num_of_interfaces().unwrap();
    /// assert_eq!(num_ifaces, 1);
    /// ```
    pub fn num_of_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfInterfaces(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_networks = conn.num_of_networks().unwrap();
    /// assert_eq!(num_networks, 1);
    /// ```
    pub fn num_of_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfNetworks(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_pools = conn.num_of_storage_pools().unwrap();
    /// assert_eq!(num_pools, 1);
    /// ```
    pub fn num_of_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfStoragePools(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    pub fn num_of_nw_filters(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfNWFilters(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    pub fn num_of_secrets(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfSecrets(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_domains = conn.num_of_defined_domains().unwrap();
    /// assert_eq!(num_domains, 0);
    /// ```
    pub fn num_of_defined_domains(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfDefinedDomains(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_ifaces = conn.num_of_defined_interfaces().unwrap();
    /// assert_eq!(num_ifaces, 0);
    /// ```
    pub fn num_of_defined_interfaces(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfDefinedInterfaces(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_networks = conn.num_of_defined_networks().unwrap();
    /// assert_eq!(num_networks, 0);
    /// ```
    pub fn num_of_defined_networks(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfDefinedNetworks(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
        }
    }

    /// # Examples
    ///
    /// ```
    /// use virt::connect::Connect;
    ///
    /// let conn = Connect::open("test:///default").unwrap();
    /// let num_pools = conn.num_of_defined_storage_pools().unwrap();
    /// assert_eq!(num_pools, 0);
    /// ```
    pub fn num_of_defined_storage_pools(&self) -> Result<u32, Error> {
        unsafe {
            let num = sys::virConnectNumOfDefinedStoragePools(self.as_ptr());
            if num == -1 {
                return Err(Error::last_error());
            }
            Ok(num as u32)
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
    /// let conn = Connect::open("test:///default").unwrap();
    /// let hyp_version = conn.get_hyp_version().unwrap();
    /// assert_eq!(hyp_version, 2);
    /// ```
    pub fn get_hyp_version(&self) -> Result<u32, Error> {
        unsafe {
            let mut hyver: libc::c_ulong = 0;
            if sys::virConnectGetVersion(self.as_ptr(), &mut hyver) == -1 {
                return Err(Error::last_error());
            }
            Ok(hyver as u32)
        }
    }

    pub fn compare_cpu(
        &self,
        xml: &str,
        flags: sys::virConnectCompareCPUFlags,
    ) -> Result<sys::virCPUCompareResult, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let res =
                sys::virConnectCompareCPU(self.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint);
            if res == sys::VIR_CPU_COMPARE_ERROR {
                return Err(Error::last_error());
            }
            Ok(res as sys::virCPUCompareResult)
        }
    }

    pub fn get_free_memory(&self) -> Result<u64, Error> {
        unsafe {
            let res = sys::virNodeGetFreeMemory(self.as_ptr());
            if res == 0 {
                return Err(Error::last_error());
            }
            Ok(res)
        }
    }

    pub fn get_node_info(&self) -> Result<NodeInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let res = sys::virNodeGetInfo(self.as_ptr(), pinfo.as_mut_ptr());
            if res == -1 {
                return Err(Error::last_error());
            }
            let pinfo = pinfo.assume_init();
            Ok(NodeInfo {
                model: c_chars_to_string!(pinfo.model.as_ptr(), nofree),
                memory: c_ulong_to_u64(pinfo.memory),
                cpus: pinfo.cpus,
                mhz: pinfo.mhz,
                nodes: pinfo.nodes,
                sockets: pinfo.sockets,
                cores: pinfo.cores,
                threads: pinfo.threads,
            })
        }
    }

    pub fn set_keep_alive(&self, interval: i32, count: u32) -> Result<i32, Error> {
        unsafe {
            let ret = sys::virConnectSetKeepAlive(
                self.as_ptr(),
                interval as libc::c_int,
                count as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret)
        }
    }

    pub fn domain_xml_from_native(
        &self,
        nformat: &str,
        nconfig: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let nformat_buf = CString::new(nformat).unwrap();
            let nconfig_buf = CString::new(nconfig).unwrap();
            let ret = sys::virConnectDomainXMLFromNative(
                self.as_ptr(),
                nformat_buf.as_ptr(),
                nconfig_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::last_error());
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
            let nformat_buf = CString::new(nformat).unwrap();
            let dxml_buf = CString::new(dxml).unwrap();
            let ret = sys::virConnectDomainXMLToNative(
                self.as_ptr(),
                nformat_buf.as_ptr(),
                dxml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::last_error());
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
            let emulatorbin_buf = CString::new(emulatorbin).unwrap();
            let arch_buf = CString::new(arch).unwrap();
            let machine_buf = CString::new(machine).unwrap();
            let virttype_buf = CString::new(virttype).unwrap();
            let ret = sys::virConnectGetDomainCapabilities(
                self.as_ptr(),
                emulatorbin_buf.as_ptr(),
                arch_buf.as_ptr(),
                machine_buf.as_ptr(),
                virttype_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::last_error());
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
            let mut record: *mut sys::virDomainStatsRecordPtr = ptr::null_mut();
            let size = sys::virConnectGetAllDomainStats(
                self.as_ptr(),
                stats as libc::c_uint,
                &mut record,
                flags as libc::c_uint,
            );
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<DomainStatsRecord> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainStatsRecord {
                    ptr: *record.offset(x),
                });
            }
            libc::free(record as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn baseline_cpu(
        &self,
        xmlcpus: &[&str],
        flags: sys::virConnectBaselineCPUFlags,
    ) -> Result<String, Error> {
        unsafe {
            let mut xcpus: [*mut CString; 512] = [ptr::null_mut(); 512];
            let mut xcpus_buf: [*const libc::c_char; 512] = [ptr::null(); 512];
            for x in 0..xmlcpus.len() {
                let mut buf = CString::new(xmlcpus[x]).unwrap();
                xcpus[x] = &mut buf;
                xcpus_buf[x] = buf.as_ptr()
            }
            let ret = sys::virConnectBaselineCPU(
                self.as_ptr(),
                xcpus_buf.as_mut_ptr(),
                xmlcpus.len() as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret.is_null() {
                return Err(Error::last_error());
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
            let kind_buf = CString::new(kind).unwrap();
            let spec_buf = CString::new(spec).unwrap();
            let n = sys::virConnectFindStoragePoolSources(
                self.as_ptr(),
                kind_buf.as_ptr(),
                spec_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }
}
