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

use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::enumutil::impl_enum;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The level of an error.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorLevel>
pub enum ErrorLevel {
    None,
    /// A simple warning.
    Warning,
    /// An error.
    Error,
}

impl_enum! {
    enum: ErrorLevel,
    raw: sys::virErrorLevel,
    match: {
        sys::VIR_ERR_NONE => None,
        sys::VIR_ERR_WARNING => Warning,
        sys::VIR_ERR_ERROR => Error,
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// An enumeration of all possible origins of an error.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorDomain>
pub enum ErrorDomain {
    None,
    /// Error at Xen hypervisor layer
    Xen,
    /// Error at connection with xend daemon
    Xend,
    /// Error at connection with xen store
    XenStore,
    /// Error in the S-Expression code
    SExpr,
    /// Error in the XML code
    Xml,
    /// Error when operating on a domain
    Dom,
    /// Error in the XML-RPC code
    Rpc,
    /// Error in the proxy code; unused since 0.8.6
    Proxy,
    /// Error in the configuration file handling
    Conf,
    /// Error at the QEMU daemon
    Qemu,
    /// Error when operating on a network
    Net,
    /// Error from test driver
    Test,
    /// Error from remote driver
    Remote,
    /// Error from OpenVZ driver
    OpenVz,
    /// Error at Xen XM layer
    XenXm,
    /// Error in the Linux Stats code
    StatsLinux,
    /// Error from Linux Container driver
    Lxc,
    /// Error from storage driver
    Storage,
    /// Error from network config
    Network,
    /// Error from domain config
    Domain,
    /// Error at the UML driver; unused since 5.0.0
    Uml,
    /// Error from node device monitor
    Nodedev,
    /// Error from xen inotify layer
    XenINotify,
    /// Error from security framework
    Security,
    /// Error from VirtualBox driver
    VBox,
    /// Error when operating on an interface
    Interface,
    /// The OpenNebula driver no longer exists. Retained for ABI/API compat only
    ONe,
    /// Error from ESX driver
    Esx,
    /// Error from the phyp driver, unused since 6.0.0
    Phyp,
    /// Error from secret storage
    Secret,
    /// Error from CPU driver
    Cpu,
    /// Error from XenAPI
    XenApi,
    /// Error from network filter driver
    Nwfilter,
    /// Error from Synchronous hooks
    Hook,
    /// Error from domain snapshot
    DomainSnapshot,
    /// Error from auditing subsystem
    Audit,
    /// Error from sysinfo/SMBIOS
    SysInfo,
    /// Error from I/O streams
    Streams,
    /// Error from VMware driver
    Vmware,
    /// Error from event loop impl
    Event,
    /// Error from libxenlight driver
    Libxl,
    /// Error from lock manager
    Locking,
    /// Error from Hyper-V driver
    HyperV,
    /// Error from capabilities
    Capabilities,
    /// Error from URI handling
    Uri,
    /// Error from auth handling
    Auth,
    /// Error from DBus
    Dbus,
    /// Error from Parallels
    Parallels,
    /// Error from Device
    Device,
    /// Error from libssh2 connection transport
    Ssh,
    /// Error from lockspace
    Lockspace,
    /// Error from initctl device communication
    Initctl,
    /// Error from identity code
    Identity,
    /// Error from cgroups
    Cgroup,
    /// Error from access control manager
    Access,
    /// Error from systemd code
    Systemd,
    /// Error from bhyve driver
    Bhyve,
    /// Error from crypto code
    Crypto,
    /// Error from firewall
    Firewall,
    /// Error from polkit code
    Polkit,
    /// Error from thread utils
    Thread,
    /// Error from admin backend
    Admin,
    /// Error from log manager
    Logging,
    /// Error from Xen xl config code
    XenXl,
    /// Error from perf
    Perf,
    /// Error from libssh connection transport
    Libssh,
    /// Error from resource control
    ResCtrl,
    /// Error from firewalld
    Firewalld,
    /// Error from domain checkpoint
    DomainCheckpoint,
    /// Error from TPM
    Tpm,
    /// Error from BPF code
    Bpf,
    /// Error from Cloud Hypervisor driver
    Ch,
    /// Indicates an error domain not yet supported by the Rust bindings
    Last,
}

impl_enum! {
    enum: ErrorDomain,
    raw: sys::virErrorDomain,
    match: {
        sys::VIR_FROM_NONE => None,
        sys::VIR_FROM_XEN => Xen,
        sys::VIR_FROM_XEND => Xend,
        sys::VIR_FROM_XENSTORE => XenStore,
        sys::VIR_FROM_SEXPR => SExpr,
        sys::VIR_FROM_XML => Xml,
        sys::VIR_FROM_DOM => Dom,
        sys::VIR_FROM_RPC => Rpc,
        sys::VIR_FROM_PROXY => Proxy,
        sys::VIR_FROM_CONF => Conf,
        sys::VIR_FROM_QEMU => Qemu,
        sys::VIR_FROM_NET => Net,
        sys::VIR_FROM_TEST => Test,
        sys::VIR_FROM_REMOTE => Remote,
        sys::VIR_FROM_OPENVZ => OpenVz,
        sys::VIR_FROM_XENXM => XenXm,
        sys::VIR_FROM_STATS_LINUX => StatsLinux,
        sys::VIR_FROM_LXC => Lxc,
        sys::VIR_FROM_STORAGE => Storage,
        sys::VIR_FROM_NETWORK => Network,
        sys::VIR_FROM_DOMAIN => Domain,
        sys::VIR_FROM_UML => Uml,
        sys::VIR_FROM_NODEDEV => Nodedev,
        sys::VIR_FROM_XEN_INOTIFY => XenINotify,
        sys::VIR_FROM_SECURITY => Security,
        sys::VIR_FROM_VBOX => VBox,
        sys::VIR_FROM_INTERFACE => Interface,
        sys::VIR_FROM_ONE => ONe,
        sys::VIR_FROM_ESX => Esx,
        sys::VIR_FROM_PHYP => Phyp,
        sys::VIR_FROM_SECRET => Secret,
        sys::VIR_FROM_CPU => Cpu,
        sys::VIR_FROM_XENAPI => XenApi,
        sys::VIR_FROM_NWFILTER => Nwfilter,
        sys::VIR_FROM_HOOK => Hook,
        sys::VIR_FROM_DOMAIN_SNAPSHOT => DomainSnapshot,
        sys::VIR_FROM_AUDIT => Audit,
        sys::VIR_FROM_SYSINFO => SysInfo,
        sys::VIR_FROM_STREAMS => Streams,
        sys::VIR_FROM_VMWARE => Vmware,
        sys::VIR_FROM_EVENT => Event,
        sys::VIR_FROM_LIBXL => Libxl,
        sys::VIR_FROM_LOCKING => Locking,
        sys::VIR_FROM_HYPERV => HyperV,
        sys::VIR_FROM_CAPABILITIES => Capabilities,
        sys::VIR_FROM_URI => Uri,
        sys::VIR_FROM_AUTH => Auth,
        sys::VIR_FROM_DBUS => Dbus,
        sys::VIR_FROM_PARALLELS => Parallels,
        sys::VIR_FROM_DEVICE => Device,
        sys::VIR_FROM_SSH => Ssh,
        sys::VIR_FROM_LOCKSPACE => Lockspace,
        sys::VIR_FROM_INITCTL => Initctl,
        sys::VIR_FROM_IDENTITY => Identity,
        sys::VIR_FROM_CGROUP => Cgroup,
        sys::VIR_FROM_ACCESS => Access,
        sys::VIR_FROM_SYSTEMD => Systemd,
        sys::VIR_FROM_BHYVE => Bhyve,
        sys::VIR_FROM_CRYPTO => Crypto,
        sys::VIR_FROM_FIREWALL => Firewall,
        sys::VIR_FROM_POLKIT => Polkit,
        sys::VIR_FROM_THREAD => Thread,
        sys::VIR_FROM_ADMIN => Admin,
        sys::VIR_FROM_LOGGING => Logging,
        sys::VIR_FROM_XENXL => XenXl,
        sys::VIR_FROM_PERF => Perf,
        sys::VIR_FROM_LIBSSH => Libssh,
        sys::VIR_FROM_RESCTRL => ResCtrl,
        sys::VIR_FROM_FIREWALLD => Firewalld,
        sys::VIR_FROM_DOMAIN_CHECKPOINT => DomainCheckpoint,
        sys::VIR_FROM_TPM => Tpm,
        sys::VIR_FROM_BPF => Bpf,
        sys::VIR_FROM_CH => Ch,
        _ => Last => sys::VIR_FROM_NONE,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// An enumeration of all possible errors.
///
/// See <https://libvirt.org/html/libvirt-virterror.html#virErrorNumber>
pub enum ErrorNumber {
    Ok,
    /// Internal error
    InternalError,
    /// Memory allocation failure
    NoMemory,
    /// No support for this function
    NoSupport,
    /// Could not resolve hostname
    UnknownHost,
    /// Can't connect to hypervisor
    NoConnect,
    /// Invalid connection object
    InvalidConn,
    /// Invalid domain object
    InvalidDomain,
    /// Invalid function argument
    InvalidArg,
    /// A command to hypervisor failed
    OperationFailed,
    /// A HTTP GET command to failed
    GetFailed,
    /// A HTTP POST command to failed
    PostFailed,
    /// Unexpected HTTP error code
    HttpError,
    /// Failure to serialize an S-Expr
    SExprSerial,
    /// Could not open Xen hypervisor control
    NoXen,
    /// Failure doing an hypervisor call
    XenCall,
    /// Unknown OS type
    OsType,
    /// Missing kernel information
    NoKernel,
    /// Missing root device information
    NoRoot,
    /// Missing source device information
    NoSource,
    /// Missing target device information
    NoTarget,
    /// Missing domain name information
    NoName,
    /// Missing domain OS information
    NoOs,
    /// Missing domain devices information
    NoDevice,
    /// Could not open Xen Store control
    NoXenStore,
    /// Too many drivers registered
    DriverFull,
    /// Not supported by the drivers (DEPRECATED)
    CallFailed,
    /// An XML description is not well formed or broken
    XmlError,
    /// The domain already exist
    DomExist,
    /// Operation forbidden on read-only connections
    OperationDenied,
    /// Failed to open a conf file
    OpenFailed,
    /// Failed to read a conf file
    ReadFailed,
    /// Failed to parse a conf file
    ParseFailed,
    /// Failed to parse the syntax of a conf file
    ConfSyntax,
    /// Failed to write a conf file
    WriteFailed,
    /// Detail of an XML error
    XmlDetail,
    /// Invalid network object
    InvalidNetwork,
    /// The network already exist
    NetworkExist,
    /// General system call failure
    SystemError,
    /// Some sort of RPC error
    Rpc,
    /// Error from a GNUTLS call
    GnutlsError,
    /// Failed to start network
    NoNetworkStart,
    /// Domain not found or unexpectedly disappeared
    NoDomain,
    /// Network not found
    NoNetwork,
    /// Invalid MAC address
    InvalidMac,
    /// Authentication failed
    AuthFailed,
    /// Invalid storage pool object
    InvalidStoragePool,
    /// Invalid storage vol object
    InvalidStorageVol,
    /// Failed to start storage
    NoStorage,
    /// Storage pool not found
    NoStoragePool,
    /// Storage volume not found
    NoStorageVolume,
    /// Failed to start node driver
    NoNode,
    /// Invalid node device object
    InvalidNodeDevice,
    /// Node device not found
    NoNodeDevice,
    /// Security model not found
    NoSecurityModel,
    /// Operation is not applicable at this time
    OperationInvalid,
    /// Failed to start interface driver
    NoInterfaceStart,
    /// Interface driver not running
    NoInterface,
    /// Invalid interface object
    InvalidInterface,
    /// More than one matching interface found
    MultipleInterfaces,
    /// Failed to start nwfilter driver
    NoNwfilterStart,
    /// Invalid nwfilter object
    InvalidNwfilter,
    /// Nw filter pool not found
    NoNwfilter,
    /// Failed to build firewall
    BuildFirewall,
    /// Failed to start secret storage
    NoSecretStart,
    /// Invalid secret
    InvalidSecret,
    /// Secret not found
    NoSecret,
    /// Unsupported configuration construct
    ConfigUnsupported,
    /// Timeout occurred during operation
    OperationTimeout,
    /// A migration worked, but making the VM persist on the dest host failed
    MigratePersistFailed,
    /// A synchronous hook script failed
    HookScriptFailed,
    /// Invalid domain snapshot
    InvalidDomainSnapshot,
    /// Domain snapshot not found
    NoDomainSnapshot,
    /// Stream pointer not valid
    InvalidStream,
    /// Valid API use but unsupported by the given driver
    ArgumentUnsupported,
    /// Storage pool probe failed
    StorageProbeFailed,
    /// Storage pool already built
    StoragePoolBuilt,
    /// Force was not requested for a risky domain snapshot revert
    SnapshotRevertRisky,
    /// Operation on a domain was canceled/aborted by user
    OperationAborted,
    /// Authentication cancelled
    AuthCancelled,
    /// The metadata is not present
    NoDomainMetadata,
    /// Migration is not safe
    MigrateUnsafe,
    /// Integer overflow
    Overflow,
    /// Action prevented by block copy job
    BlockCopyActive,
    /// The requested operation is not supported
    OperationUnsupported,
    /// Error in ssh transport driver
    Ssh,
    /// Guest agent is unresponsive, not running or not usable
    AgentUnresponsive,
    /// Resource is already in use
    ResourceBusy,
    /// Operation on the object/resource was denied
    AccessDenied,
    /// Error from a dbus service
    DbusService,
    /// The storage vol already exists
    StorageVolExist,
    /// Given CPU is incompatible with host CPU
    CpuIncompatible,
    /// XML document doesn't validate against schema
    XmlInvalidSchema,
    /// Finish API succeeded but it is expected to return NULL
    MigrateFinishOk,
    /// Authentication unavailable
    AuthUnavailable,
    /// Server was not found
    NoServer,
    /// Client was not found
    NoClient,
    /// Guest agent replies with wrong id to guest-sync command (DEPRECATED)
    AgentUnsynced,
    /// Error in libssh transport driver
    Libssh,
    /// Fail to find the desired device
    DeviceMissing,
    /// Invalid nwfilter binding
    InvalidNwfilterBinding,
    /// No nwfilter binding
    NoNwfilterBinding,
    /// Invalid domain checkpoint
    InvalidDomainCheckpoint,
    /// Domain checkpoint not found
    NoDomainCheckpoint,
    /// Domain backup job id not found
    NoDomainBackup,
    /// Invalid network port object
    InvalidNetworkPort,
    /// The network port already exist
    NetworkPortExists,
    /// Network port not found
    NoNetworkPort,
    /// No domain's hostname found
    NoHostname,
    /// Checkpoint can't be used
    CheckpointInconsistent,
    /// More than one matching domain found
    MultipleDomains,
    /// Network metadata is not present
    NoNetworkMetadata,
    /// Guest agent didn't respond to a non-sync command within timeout
    AgentCommandTimeout,
    /// Guest agent responded with failure to a command
    AgentCommandFailed,
    /// Indicates an error number not yet supported by the Rust bindings
    Last,
}

impl_enum! {
    enum: ErrorNumber,
    raw: sys::virErrorNumber,
    match: {
        sys::VIR_ERR_OK => Ok,
        sys::VIR_ERR_INTERNAL_ERROR => InternalError,
        sys::VIR_ERR_NO_MEMORY => NoMemory,
        sys::VIR_ERR_NO_SUPPORT => NoSupport,
        sys::VIR_ERR_UNKNOWN_HOST => UnknownHost,
        sys::VIR_ERR_NO_CONNECT => NoConnect,
        sys::VIR_ERR_INVALID_CONN => InvalidConn,
        sys::VIR_ERR_INVALID_DOMAIN => InvalidDomain,
        sys::VIR_ERR_INVALID_ARG => InvalidArg,
        sys::VIR_ERR_OPERATION_FAILED => OperationFailed,
        sys::VIR_ERR_GET_FAILED => GetFailed,
        sys::VIR_ERR_POST_FAILED => PostFailed,
        sys::VIR_ERR_HTTP_ERROR => HttpError,
        sys::VIR_ERR_SEXPR_SERIAL => SExprSerial,
        sys::VIR_ERR_NO_XEN => NoXen,
        sys::VIR_ERR_XEN_CALL => XenCall,
        sys::VIR_ERR_OS_TYPE => OsType,
        sys::VIR_ERR_NO_KERNEL => NoKernel,
        sys::VIR_ERR_NO_ROOT => NoRoot,
        sys::VIR_ERR_NO_SOURCE => NoSource,
        sys::VIR_ERR_NO_TARGET => NoTarget,
        sys::VIR_ERR_NO_NAME => NoName,
        sys::VIR_ERR_NO_OS => NoOs,
        sys::VIR_ERR_NO_DEVICE => NoDevice,
        sys::VIR_ERR_NO_XENSTORE => NoXenStore,
        sys::VIR_ERR_DRIVER_FULL => DriverFull,
        sys::VIR_ERR_CALL_FAILED => CallFailed,
        sys::VIR_ERR_XML_ERROR => XmlError,
        sys::VIR_ERR_DOM_EXIST => DomExist,
        sys::VIR_ERR_OPERATION_DENIED => OperationDenied,
        sys::VIR_ERR_OPEN_FAILED => OpenFailed,
        sys::VIR_ERR_READ_FAILED => ReadFailed,
        sys::VIR_ERR_PARSE_FAILED => ParseFailed,
        sys::VIR_ERR_CONF_SYNTAX => ConfSyntax,
        sys::VIR_ERR_WRITE_FAILED => WriteFailed,
        sys::VIR_ERR_XML_DETAIL => XmlDetail,
        sys::VIR_ERR_INVALID_NETWORK => InvalidNetwork,
        sys::VIR_ERR_NETWORK_EXIST => NetworkExist,
        sys::VIR_ERR_SYSTEM_ERROR => SystemError,
        sys::VIR_ERR_RPC => Rpc,
        sys::VIR_ERR_GNUTLS_ERROR => GnutlsError,
        sys::VIR_WAR_NO_NETWORK => NoNetworkStart,
        sys::VIR_ERR_NO_DOMAIN => NoDomain,
        sys::VIR_ERR_NO_NETWORK => NoNetwork,
        sys::VIR_ERR_INVALID_MAC => InvalidMac,
        sys::VIR_ERR_AUTH_FAILED => AuthFailed,
        sys::VIR_ERR_INVALID_STORAGE_POOL => InvalidStoragePool,
        sys::VIR_ERR_INVALID_STORAGE_VOL => InvalidStorageVol,
        sys::VIR_WAR_NO_STORAGE => NoStorage,
        sys::VIR_ERR_NO_STORAGE_POOL => NoStoragePool,
        sys::VIR_ERR_NO_STORAGE_VOL => NoStorageVolume,
        sys::VIR_WAR_NO_NODE => NoNode,
        sys::VIR_ERR_INVALID_NODE_DEVICE => InvalidNodeDevice,
        sys::VIR_ERR_NO_NODE_DEVICE => NoNodeDevice,
        sys::VIR_ERR_NO_SECURITY_MODEL => NoSecurityModel,
        sys::VIR_ERR_OPERATION_INVALID => OperationInvalid,
        sys::VIR_WAR_NO_INTERFACE => NoInterfaceStart,
        sys::VIR_ERR_NO_INTERFACE => NoInterface,
        sys::VIR_ERR_INVALID_INTERFACE => InvalidInterface,
        sys::VIR_ERR_MULTIPLE_INTERFACES => MultipleInterfaces,
        sys::VIR_WAR_NO_NWFILTER => NoNwfilterStart,
        sys::VIR_ERR_INVALID_NWFILTER => InvalidNwfilter,
        sys::VIR_ERR_NO_NWFILTER => NoNwfilter,
        sys::VIR_ERR_BUILD_FIREWALL => BuildFirewall,
        sys::VIR_WAR_NO_SECRET => NoSecretStart,
        sys::VIR_ERR_INVALID_SECRET => InvalidSecret,
        sys::VIR_ERR_NO_SECRET => NoSecret,
        sys::VIR_ERR_CONFIG_UNSUPPORTED => ConfigUnsupported,
        sys::VIR_ERR_OPERATION_TIMEOUT => OperationTimeout,
        sys::VIR_ERR_MIGRATE_PERSIST_FAILED => MigratePersistFailed,
        sys::VIR_ERR_HOOK_SCRIPT_FAILED => HookScriptFailed,
        sys::VIR_ERR_INVALID_DOMAIN_SNAPSHOT => InvalidDomainSnapshot,
        sys::VIR_ERR_NO_DOMAIN_SNAPSHOT => NoDomainSnapshot,
        sys::VIR_ERR_INVALID_STREAM => InvalidStream,
        sys::VIR_ERR_ARGUMENT_UNSUPPORTED => ArgumentUnsupported,
        sys::VIR_ERR_STORAGE_PROBE_FAILED => StorageProbeFailed,
        sys::VIR_ERR_STORAGE_POOL_BUILT => StoragePoolBuilt,
        sys::VIR_ERR_SNAPSHOT_REVERT_RISKY => SnapshotRevertRisky,
        sys::VIR_ERR_OPERATION_ABORTED => OperationAborted,
        sys::VIR_ERR_AUTH_CANCELLED => AuthCancelled,
        sys::VIR_ERR_NO_DOMAIN_METADATA => NoDomainMetadata,
        sys::VIR_ERR_MIGRATE_UNSAFE => MigrateUnsafe,
        sys::VIR_ERR_OVERFLOW => Overflow,
        sys::VIR_ERR_BLOCK_COPY_ACTIVE => BlockCopyActive,
        sys::VIR_ERR_OPERATION_UNSUPPORTED => OperationUnsupported,
        sys::VIR_ERR_SSH => Ssh,
        sys::VIR_ERR_AGENT_UNRESPONSIVE => AgentUnresponsive,
        sys::VIR_ERR_RESOURCE_BUSY => ResourceBusy,
        sys::VIR_ERR_ACCESS_DENIED => AccessDenied,
        sys::VIR_ERR_DBUS_SERVICE => DbusService,
        sys::VIR_ERR_STORAGE_VOL_EXIST => StorageVolExist,
        sys::VIR_ERR_CPU_INCOMPATIBLE => CpuIncompatible,
        sys::VIR_ERR_XML_INVALID_SCHEMA => XmlInvalidSchema,
        sys::VIR_ERR_MIGRATE_FINISH_OK => MigrateFinishOk,
        sys::VIR_ERR_AUTH_UNAVAILABLE => AuthUnavailable,
        sys::VIR_ERR_NO_SERVER => NoServer,
        sys::VIR_ERR_NO_CLIENT => NoClient,
        sys::VIR_ERR_AGENT_UNSYNCED => AgentUnsynced,
        sys::VIR_ERR_LIBSSH => Libssh,
        sys::VIR_ERR_DEVICE_MISSING => DeviceMissing,
        sys::VIR_ERR_INVALID_NWFILTER_BINDING => InvalidNwfilterBinding,
        sys::VIR_ERR_NO_NWFILTER_BINDING => NoNwfilterBinding,
        sys::VIR_ERR_INVALID_DOMAIN_CHECKPOINT => InvalidDomainCheckpoint,
        sys::VIR_ERR_NO_DOMAIN_CHECKPOINT => NoDomainCheckpoint,
        sys::VIR_ERR_NO_DOMAIN_BACKUP => NoDomainBackup,
        sys::VIR_ERR_INVALID_NETWORK_PORT => InvalidNetworkPort,
        sys::VIR_ERR_NETWORK_PORT_EXIST => NetworkPortExists,
        sys::VIR_ERR_NO_NETWORK_PORT => NoNetworkPort,
        sys::VIR_ERR_NO_HOSTNAME => NoHostname,
        sys::VIR_ERR_CHECKPOINT_INCONSISTENT => CheckpointInconsistent,
        sys::VIR_ERR_MULTIPLE_DOMAINS => MultipleDomains,
        sys::VIR_ERR_NO_NETWORK_METADATA => NoNetworkMetadata,
        sys::VIR_ERR_AGENT_COMMAND_TIMEOUT => AgentCommandTimeout,
        sys::VIR_ERR_AGENT_COMMAND_FAILED => AgentCommandFailed,
        _ => Last => sys::VIR_ERR_INTERNAL_ERROR,
    }
}

/// A structure that represents errors coming from libvirt.
///
/// See <https://libvirt.org/html/libvirt-virterror.html>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    code: sys::virErrorNumber,
    domain: sys::virErrorDomain,
    message: String,
    level: sys::virErrorLevel,
}

extern "C" fn noop(_data: *mut libc::c_void, _error: sys::virErrorPtr) {}

impl Error {
    /// Returns the last error that occurred.
    ///
    /// This function is typically called by the safe wrappers in this library. It should only be
    /// used if you call [virt_sys] functions directly.
    pub fn last_error() -> Error {
        let ptr: sys::virErrorPtr = unsafe { sys::virGetLastError() };
        if ptr.is_null() {
            Error {
                code: sys::VIR_ERR_INTERNAL_ERROR,
                domain: sys::VIR_FROM_NONE,
                message: "an unknown libvirt error occurred".into(),
                level: sys::VIR_ERR_ERROR,
            }
        } else {
            unsafe { Error::from_raw(ptr) }
        }
    }

    unsafe fn from_raw(ptr: sys::virErrorPtr) -> Error {
        let code = (*ptr).code as sys::virErrorNumber;
        let domain = (*ptr).domain as sys::virErrorDomain;
        let message = CStr::from_ptr((*ptr).message)
            .to_string_lossy()
            .into_owned();
        let level = (*ptr).level;
        Error {
            code,
            domain,
            message,
            level,
        }
    }

    /// Returns the exact error code.
    pub fn code(&self) -> ErrorNumber {
        ErrorNumber::from_raw(self.code)
    }

    /// Returns the source of the error.
    pub fn domain(&self) -> ErrorDomain {
        ErrorDomain::from_raw(self.domain)
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error level.
    pub fn level(&self) -> ErrorLevel {
        ErrorLevel::from_raw(self.level)
    }
}

impl StdError for Error {}

// Clippy is confused:
//  warning: current MSRV (Minimum Supported Rust Version) is `1.63.0` but this item is stable since `1.64.0`
//
// but rust docs say
//
//  https://blog.rust-lang.org/2022/09/22/Rust-1.64.0/
//
// "These types were previously stable in std::ffi, but are now also available in core and alloc:
//      alloc::ffi::NulError"
//
// IOW, std::ffi::NulError was already stable before 1.63.0
#[allow(clippy::incompatible_msrv)]
impl From<std::ffi::NulError> for Error {
    fn from(nulerr: std::ffi::NulError) -> Self {
        Error {
            code: ErrorNumber::InvalidArg as u32,
            domain: ErrorDomain::None as u32,
            message: format!("Null byte passed to CString: {nulerr}"),
            level: ErrorLevel::Error as u32,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.level() {
            ErrorLevel::None => {}
            _ => write!(f, "{}: ", self.level())?,
        }
        write!(
            f,
            "{} [code={} ({}), domain={} ({})]",
            self.message,
            self.code(),
            self.code,
            self.domain(),
            self.domain,
        )
    }
}

/// Clears the libvirt error callback.
///
/// Use this to disable libvirt's default handler, which prints all errors to stdout
pub fn clear_error_callback() {
    unsafe {
        sys::virSetErrorFunc(std::ptr::null_mut(), Some(noop));
    }
}
