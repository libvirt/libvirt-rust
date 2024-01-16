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

use crate::util::impl_enum;

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
        sys::VIR_ERR_NONE => ErrorLevel::None,
        sys::VIR_ERR_WARNING => ErrorLevel::Warning,
        sys::VIR_ERR_ERROR => ErrorLevel::Error,
        _ => ErrorLevel::None,
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
        sys::VIR_FROM_NONE => ErrorDomain::None,
        sys::VIR_FROM_XEN => ErrorDomain::Xen,
        sys::VIR_FROM_XEND => ErrorDomain::Xend,
        sys::VIR_FROM_XENSTORE => ErrorDomain::XenStore,
        sys::VIR_FROM_SEXPR => ErrorDomain::SExpr,
        sys::VIR_FROM_XML => ErrorDomain::Xml,
        sys::VIR_FROM_DOM => ErrorDomain::Dom,
        sys::VIR_FROM_RPC => ErrorDomain::Rpc,
        sys::VIR_FROM_PROXY => ErrorDomain::Proxy,
        sys::VIR_FROM_CONF => ErrorDomain::Conf,
        sys::VIR_FROM_QEMU => ErrorDomain::Qemu,
        sys::VIR_FROM_NET => ErrorDomain::Net,
        sys::VIR_FROM_TEST => ErrorDomain::Test,
        sys::VIR_FROM_REMOTE => ErrorDomain::Remote,
        sys::VIR_FROM_OPENVZ => ErrorDomain::OpenVz,
        sys::VIR_FROM_XENXM => ErrorDomain::XenXm,
        sys::VIR_FROM_STATS_LINUX => ErrorDomain::StatsLinux,
        sys::VIR_FROM_LXC => ErrorDomain::Lxc,
        sys::VIR_FROM_STORAGE => ErrorDomain::Storage,
        sys::VIR_FROM_NETWORK => ErrorDomain::Network,
        sys::VIR_FROM_DOMAIN => ErrorDomain::Domain,
        sys::VIR_FROM_UML => ErrorDomain::Uml,
        sys::VIR_FROM_NODEDEV => ErrorDomain::Nodedev,
        sys::VIR_FROM_XEN_INOTIFY => ErrorDomain::XenINotify,
        sys::VIR_FROM_SECURITY => ErrorDomain::Security,
        sys::VIR_FROM_VBOX => ErrorDomain::VBox,
        sys::VIR_FROM_INTERFACE => ErrorDomain::Interface,
        sys::VIR_FROM_ONE => ErrorDomain::ONe,
        sys::VIR_FROM_ESX => ErrorDomain::Esx,
        sys::VIR_FROM_PHYP => ErrorDomain::Phyp,
        sys::VIR_FROM_SECRET => ErrorDomain::Secret,
        sys::VIR_FROM_CPU => ErrorDomain::Cpu,
        sys::VIR_FROM_XENAPI => ErrorDomain::XenApi,
        sys::VIR_FROM_NWFILTER => ErrorDomain::Nwfilter,
        sys::VIR_FROM_HOOK => ErrorDomain::Hook,
        sys::VIR_FROM_DOMAIN_SNAPSHOT => ErrorDomain::DomainSnapshot,
        sys::VIR_FROM_AUDIT => ErrorDomain::Audit,
        sys::VIR_FROM_SYSINFO => ErrorDomain::SysInfo,
        sys::VIR_FROM_STREAMS => ErrorDomain::Streams,
        sys::VIR_FROM_VMWARE => ErrorDomain::Vmware,
        sys::VIR_FROM_EVENT => ErrorDomain::Event,
        sys::VIR_FROM_LIBXL => ErrorDomain::Libxl,
        sys::VIR_FROM_LOCKING => ErrorDomain::Locking,
        sys::VIR_FROM_HYPERV => ErrorDomain::HyperV,
        sys::VIR_FROM_CAPABILITIES => ErrorDomain::Capabilities,
        sys::VIR_FROM_URI => ErrorDomain::Uri,
        sys::VIR_FROM_AUTH => ErrorDomain::Auth,
        sys::VIR_FROM_DBUS => ErrorDomain::Dbus,
        sys::VIR_FROM_PARALLELS => ErrorDomain::Parallels,
        sys::VIR_FROM_DEVICE => ErrorDomain::Device,
        sys::VIR_FROM_SSH => ErrorDomain::Ssh,
        sys::VIR_FROM_LOCKSPACE => ErrorDomain::Lockspace,
        sys::VIR_FROM_INITCTL => ErrorDomain::Initctl,
        sys::VIR_FROM_IDENTITY => ErrorDomain::Identity,
        sys::VIR_FROM_CGROUP => ErrorDomain::Cgroup,
        sys::VIR_FROM_ACCESS => ErrorDomain::Access,
        sys::VIR_FROM_SYSTEMD => ErrorDomain::Systemd,
        sys::VIR_FROM_BHYVE => ErrorDomain::Bhyve,
        sys::VIR_FROM_CRYPTO => ErrorDomain::Crypto,
        sys::VIR_FROM_FIREWALL => ErrorDomain::Firewall,
        sys::VIR_FROM_POLKIT => ErrorDomain::Polkit,
        sys::VIR_FROM_THREAD => ErrorDomain::Thread,
        sys::VIR_FROM_ADMIN => ErrorDomain::Admin,
        sys::VIR_FROM_LOGGING => ErrorDomain::Logging,
        sys::VIR_FROM_XENXL => ErrorDomain::XenXl,
        sys::VIR_FROM_PERF => ErrorDomain::Perf,
        sys::VIR_FROM_LIBSSH => ErrorDomain::Libssh,
        sys::VIR_FROM_RESCTRL => ErrorDomain::ResCtrl,
        sys::VIR_FROM_FIREWALLD => ErrorDomain::Firewalld,
        sys::VIR_FROM_DOMAIN_CHECKPOINT => ErrorDomain::DomainCheckpoint,
        sys::VIR_FROM_TPM => ErrorDomain::Tpm,
        sys::VIR_FROM_BPF => ErrorDomain::Bpf,
        sys::VIR_FROM_CH => ErrorDomain::Ch,
        _ => ErrorDomain::Last => sys::VIR_FROM_NONE,
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
    /// Indicates an error number not yet supported by the Rust bindings
    Last,
}

impl_enum! {
    enum: ErrorNumber,
    raw: sys::virErrorNumber,
    match: {
        sys::VIR_ERR_OK => ErrorNumber::Ok,
        sys::VIR_ERR_INTERNAL_ERROR => ErrorNumber::InternalError,
        sys::VIR_ERR_NO_MEMORY => ErrorNumber::NoMemory,
        sys::VIR_ERR_NO_SUPPORT => ErrorNumber::NoSupport,
        sys::VIR_ERR_UNKNOWN_HOST => ErrorNumber::UnknownHost,
        sys::VIR_ERR_NO_CONNECT => ErrorNumber::NoConnect,
        sys::VIR_ERR_INVALID_CONN => ErrorNumber::InvalidConn,
        sys::VIR_ERR_INVALID_DOMAIN => ErrorNumber::InvalidDomain,
        sys::VIR_ERR_INVALID_ARG => ErrorNumber::InvalidArg,
        sys::VIR_ERR_OPERATION_FAILED => ErrorNumber::OperationFailed,
        sys::VIR_ERR_GET_FAILED => ErrorNumber::GetFailed,
        sys::VIR_ERR_POST_FAILED => ErrorNumber::PostFailed,
        sys::VIR_ERR_HTTP_ERROR => ErrorNumber::HttpError,
        sys::VIR_ERR_SEXPR_SERIAL => ErrorNumber::SExprSerial,
        sys::VIR_ERR_NO_XEN => ErrorNumber::NoXen,
        sys::VIR_ERR_XEN_CALL => ErrorNumber::XenCall,
        sys::VIR_ERR_OS_TYPE => ErrorNumber::OsType,
        sys::VIR_ERR_NO_KERNEL => ErrorNumber::NoKernel,
        sys::VIR_ERR_NO_ROOT => ErrorNumber::NoRoot,
        sys::VIR_ERR_NO_SOURCE => ErrorNumber::NoSource,
        sys::VIR_ERR_NO_TARGET => ErrorNumber::NoTarget,
        sys::VIR_ERR_NO_NAME => ErrorNumber::NoName,
        sys::VIR_ERR_NO_OS => ErrorNumber::NoOs,
        sys::VIR_ERR_NO_DEVICE => ErrorNumber::NoDevice,
        sys::VIR_ERR_NO_XENSTORE => ErrorNumber::NoXenStore,
        sys::VIR_ERR_DRIVER_FULL => ErrorNumber::DriverFull,
        sys::VIR_ERR_CALL_FAILED => ErrorNumber::CallFailed,
        sys::VIR_ERR_XML_ERROR => ErrorNumber::XmlError,
        sys::VIR_ERR_DOM_EXIST => ErrorNumber::DomExist,
        sys::VIR_ERR_OPERATION_DENIED => ErrorNumber::OperationDenied,
        sys::VIR_ERR_OPEN_FAILED => ErrorNumber::OpenFailed,
        sys::VIR_ERR_READ_FAILED => ErrorNumber::ReadFailed,
        sys::VIR_ERR_PARSE_FAILED => ErrorNumber::ParseFailed,
        sys::VIR_ERR_CONF_SYNTAX => ErrorNumber::ConfSyntax,
        sys::VIR_ERR_WRITE_FAILED => ErrorNumber::WriteFailed,
        sys::VIR_ERR_XML_DETAIL => ErrorNumber::XmlDetail,
        sys::VIR_ERR_INVALID_NETWORK => ErrorNumber::InvalidNetwork,
        sys::VIR_ERR_NETWORK_EXIST => ErrorNumber::NetworkExist,
        sys::VIR_ERR_SYSTEM_ERROR => ErrorNumber::SystemError,
        sys::VIR_ERR_RPC => ErrorNumber::Rpc,
        sys::VIR_ERR_GNUTLS_ERROR => ErrorNumber::GnutlsError,
        sys::VIR_WAR_NO_NETWORK => ErrorNumber::NoNetworkStart,
        sys::VIR_ERR_NO_DOMAIN => ErrorNumber::NoDomain,
        sys::VIR_ERR_NO_NETWORK => ErrorNumber::NoNetwork,
        sys::VIR_ERR_INVALID_MAC => ErrorNumber::InvalidMac,
        sys::VIR_ERR_AUTH_FAILED => ErrorNumber::AuthFailed,
        sys::VIR_ERR_INVALID_STORAGE_POOL => ErrorNumber::InvalidStoragePool,
        sys::VIR_ERR_INVALID_STORAGE_VOL => ErrorNumber::InvalidStorageVol,
        sys::VIR_WAR_NO_STORAGE => ErrorNumber::NoStorage,
        sys::VIR_ERR_NO_STORAGE_POOL => ErrorNumber::NoStoragePool,
        sys::VIR_ERR_NO_STORAGE_VOL => ErrorNumber::NoStorageVolume,
        sys::VIR_WAR_NO_NODE => ErrorNumber::NoNode,
        sys::VIR_ERR_INVALID_NODE_DEVICE => ErrorNumber::InvalidNodeDevice,
        sys::VIR_ERR_NO_NODE_DEVICE => ErrorNumber::NoNodeDevice,
        sys::VIR_ERR_NO_SECURITY_MODEL => ErrorNumber::NoSecurityModel,
        sys::VIR_ERR_OPERATION_INVALID => ErrorNumber::OperationInvalid,
        sys::VIR_WAR_NO_INTERFACE => ErrorNumber::NoInterfaceStart,
        sys::VIR_ERR_NO_INTERFACE => ErrorNumber::NoInterface,
        sys::VIR_ERR_INVALID_INTERFACE => ErrorNumber::InvalidInterface,
        sys::VIR_ERR_MULTIPLE_INTERFACES => ErrorNumber::MultipleInterfaces,
        sys::VIR_WAR_NO_NWFILTER => ErrorNumber::NoNwfilterStart,
        sys::VIR_ERR_INVALID_NWFILTER => ErrorNumber::InvalidNwfilter,
        sys::VIR_ERR_NO_NWFILTER => ErrorNumber::NoNwfilter,
        sys::VIR_ERR_BUILD_FIREWALL => ErrorNumber::BuildFirewall,
        sys::VIR_WAR_NO_SECRET => ErrorNumber::NoSecretStart,
        sys::VIR_ERR_INVALID_SECRET => ErrorNumber::InvalidSecret,
        sys::VIR_ERR_NO_SECRET => ErrorNumber::NoSecret,
        sys::VIR_ERR_CONFIG_UNSUPPORTED => ErrorNumber::ConfigUnsupported,
        sys::VIR_ERR_OPERATION_TIMEOUT => ErrorNumber::OperationTimeout,
        sys::VIR_ERR_MIGRATE_PERSIST_FAILED => ErrorNumber::MigratePersistFailed,
        sys::VIR_ERR_HOOK_SCRIPT_FAILED => ErrorNumber::HookScriptFailed,
        sys::VIR_ERR_INVALID_DOMAIN_SNAPSHOT => ErrorNumber::InvalidDomainSnapshot,
        sys::VIR_ERR_NO_DOMAIN_SNAPSHOT => ErrorNumber::NoDomainSnapshot,
        sys::VIR_ERR_INVALID_STREAM => ErrorNumber::InvalidStream,
        sys::VIR_ERR_ARGUMENT_UNSUPPORTED => ErrorNumber::ArgumentUnsupported,
        sys::VIR_ERR_STORAGE_PROBE_FAILED => ErrorNumber::StorageProbeFailed,
        sys::VIR_ERR_STORAGE_POOL_BUILT => ErrorNumber::StoragePoolBuilt,
        sys::VIR_ERR_SNAPSHOT_REVERT_RISKY => ErrorNumber::SnapshotRevertRisky,
        sys::VIR_ERR_OPERATION_ABORTED => ErrorNumber::OperationAborted,
        sys::VIR_ERR_AUTH_CANCELLED => ErrorNumber::AuthCancelled,
        sys::VIR_ERR_NO_DOMAIN_METADATA => ErrorNumber::NoDomainMetadata,
        sys::VIR_ERR_MIGRATE_UNSAFE => ErrorNumber::MigrateUnsafe,
        sys::VIR_ERR_OVERFLOW => ErrorNumber::Overflow,
        sys::VIR_ERR_BLOCK_COPY_ACTIVE => ErrorNumber::BlockCopyActive,
        sys::VIR_ERR_OPERATION_UNSUPPORTED => ErrorNumber::OperationUnsupported,
        sys::VIR_ERR_SSH => ErrorNumber::Ssh,
        sys::VIR_ERR_AGENT_UNRESPONSIVE => ErrorNumber::AgentUnresponsive,
        sys::VIR_ERR_RESOURCE_BUSY => ErrorNumber::ResourceBusy,
        sys::VIR_ERR_ACCESS_DENIED => ErrorNumber::AccessDenied,
        sys::VIR_ERR_DBUS_SERVICE => ErrorNumber::DbusService,
        sys::VIR_ERR_STORAGE_VOL_EXIST => ErrorNumber::StorageVolExist,
        sys::VIR_ERR_CPU_INCOMPATIBLE => ErrorNumber::CpuIncompatible,
        sys::VIR_ERR_XML_INVALID_SCHEMA => ErrorNumber::XmlInvalidSchema,
        sys::VIR_ERR_MIGRATE_FINISH_OK => ErrorNumber::MigrateFinishOk,
        sys::VIR_ERR_AUTH_UNAVAILABLE => ErrorNumber::AuthUnavailable,
        sys::VIR_ERR_NO_SERVER => ErrorNumber::NoServer,
        sys::VIR_ERR_NO_CLIENT => ErrorNumber::NoClient,
        sys::VIR_ERR_AGENT_UNSYNCED => ErrorNumber::AgentUnsynced,
        sys::VIR_ERR_LIBSSH => ErrorNumber::Libssh,
        sys::VIR_ERR_DEVICE_MISSING => ErrorNumber::DeviceMissing,
        sys::VIR_ERR_INVALID_NWFILTER_BINDING => ErrorNumber::InvalidNwfilterBinding,
        sys::VIR_ERR_NO_NWFILTER_BINDING => ErrorNumber::NoNwfilterBinding,
        sys::VIR_ERR_INVALID_DOMAIN_CHECKPOINT => ErrorNumber::InvalidDomainCheckpoint,
        sys::VIR_ERR_NO_DOMAIN_CHECKPOINT => ErrorNumber::NoDomainCheckpoint,
        sys::VIR_ERR_NO_DOMAIN_BACKUP => ErrorNumber::NoDomainBackup,
        sys::VIR_ERR_INVALID_NETWORK_PORT => ErrorNumber::InvalidNetworkPort,
        sys::VIR_ERR_NETWORK_PORT_EXIST => ErrorNumber::NetworkPortExists,
        sys::VIR_ERR_NO_NETWORK_PORT => ErrorNumber::NoNetworkPort,
        sys::VIR_ERR_NO_HOSTNAME => ErrorNumber::NoHostname,
        sys::VIR_ERR_CHECKPOINT_INCONSISTENT => ErrorNumber::CheckpointInconsistent,
        sys::VIR_ERR_MULTIPLE_DOMAINS => ErrorNumber::MultipleDomains,
        sys::VIR_ERR_NO_NETWORK_METADATA => ErrorNumber::NoNetworkMetadata,
        _ => ErrorNumber::Last => sys::VIR_ERR_INTERNAL_ERROR,
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.level() {
            ErrorLevel::None => {}
            ErrorLevel::Warning => write!(f, "warning: ")?,
            ErrorLevel::Error => write!(f, "error: ")?,
        }
        write!(
            f,
            "{} [code={:?} ({}), domain={:?} ({})]",
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
