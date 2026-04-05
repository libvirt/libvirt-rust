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
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::{c_char, c_int, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong, c_void};
use std::{mem, ptr, str};
use uuid::Uuid;

use crate::connect::Connect;
use crate::domain_snapshot::DomainSnapshot;
use crate::enumutil::{impl_enum, Enum};
use crate::error::Error;
use crate::stream::Stream;
use crate::typedparams::{from_params, to_params};
use crate::util::{c_ulong_to_u64, check_neg, check_null, check_zero};
use crate::{param_field_in, param_field_out};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainState {
    NoState,
    Running,
    Blocked,
    Paused,
    Shutdown,
    Shutoff,
    Crashed,
    PMSuspended,
}

pub type DomainStateEnum = Enum<DomainState, sys::virDomainState>;

impl_enum! {
    enum: DomainState,
    raw: sys::virDomainState,
    match: {
    sys::VIR_DOMAIN_NOSTATE => NoState,
    sys::VIR_DOMAIN_RUNNING => Running,
    sys::VIR_DOMAIN_BLOCKED => Blocked,
    sys::VIR_DOMAIN_PAUSED => Paused,
    sys::VIR_DOMAIN_SHUTDOWN => Shutdown,
    sys::VIR_DOMAIN_SHUTOFF => Shutoff,
    sys::VIR_DOMAIN_CRASHED => Crashed,
    sys::VIR_DOMAIN_PMSUSPENDED => PMSuspended,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainNoStateReason {
    Unknown,
}

pub type DomainNoStateReasonEnum = Enum<DomainNoStateReason, sys::virDomainNostateReason>;

impl_enum! {
    enum: DomainNoStateReason,
    raw: sys::virDomainNostateReason,
    match: {
    sys::VIR_DOMAIN_NOSTATE_UNKNOWN => Unknown,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainRunningReason {
    Unknown,
    Booted,
    Migrated,
    Restored,
    FromSnapshot,
    Unpaused,
    MigrationCancelled,
    SaveCancelled,
    Wakeup,
    Crashed,
    PostCopy,
    PostCopyFailed,
}

pub type DomainRunningReasonEnum = Enum<DomainRunningReason, sys::virDomainRunningReason>;

impl_enum! {
    enum: DomainRunningReason,
    raw: sys::virDomainRunningReason,
    match: {
    sys::VIR_DOMAIN_RUNNING_UNKNOWN => Unknown,
    sys::VIR_DOMAIN_RUNNING_BOOTED => Booted,
    sys::VIR_DOMAIN_RUNNING_MIGRATED => Migrated,
    sys::VIR_DOMAIN_RUNNING_RESTORED => Restored,
    sys::VIR_DOMAIN_RUNNING_FROM_SNAPSHOT => FromSnapshot,
    sys::VIR_DOMAIN_RUNNING_UNPAUSED => Unpaused,
    sys::VIR_DOMAIN_RUNNING_MIGRATION_CANCELED => MigrationCancelled,
    sys::VIR_DOMAIN_RUNNING_SAVE_CANCELED => SaveCancelled,
    sys::VIR_DOMAIN_RUNNING_WAKEUP => Wakeup,
    sys::VIR_DOMAIN_RUNNING_CRASHED => Crashed,
    sys::VIR_DOMAIN_RUNNING_POSTCOPY => PostCopy,
    sys::VIR_DOMAIN_RUNNING_POSTCOPY_FAILED => PostCopyFailed,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainBlockedReason {
    Unknown,
}

pub type DomainBlockedReasonEnum = Enum<DomainBlockedReason, sys::virDomainBlockedReason>;

impl_enum! {
    enum: DomainBlockedReason,
    raw: sys::virDomainBlockedReason,
    match: {
    sys::VIR_DOMAIN_BLOCKED_UNKNOWN => Unknown,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainPausedReason {
    Unknown,
    User,
    Migration,
    Save,
    Dump,
    IOError,
    Watchdog,
    FromSnapshot,
    ShuttingDown,
    Snapshot,
    Crashed,
    StartingUp,
    PostCopy,
    PostCopyFailed,
    APIError,
}

pub type DomainPausedReasonEnum = Enum<DomainPausedReason, sys::virDomainPausedReason>;

impl_enum! {
    enum: DomainPausedReason,
    raw: sys::virDomainPausedReason,
    match: {
    sys::VIR_DOMAIN_PAUSED_UNKNOWN => Unknown,
    sys::VIR_DOMAIN_PAUSED_USER => User,
    sys::VIR_DOMAIN_PAUSED_MIGRATION => Migration,
    sys::VIR_DOMAIN_PAUSED_SAVE => Save,
    sys::VIR_DOMAIN_PAUSED_DUMP => Dump,
    sys::VIR_DOMAIN_PAUSED_IOERROR => IOError,
    sys::VIR_DOMAIN_PAUSED_WATCHDOG => Watchdog,
    sys::VIR_DOMAIN_PAUSED_FROM_SNAPSHOT => FromSnapshot,
    sys::VIR_DOMAIN_PAUSED_SHUTTING_DOWN => ShuttingDown,
    sys::VIR_DOMAIN_PAUSED_SNAPSHOT => Snapshot,
    sys::VIR_DOMAIN_PAUSED_CRASHED => Crashed,
    sys::VIR_DOMAIN_PAUSED_STARTING_UP => StartingUp,
    sys::VIR_DOMAIN_PAUSED_POSTCOPY => PostCopy,
    sys::VIR_DOMAIN_PAUSED_POSTCOPY_FAILED => PostCopyFailed,
    sys::VIR_DOMAIN_PAUSED_API_ERROR => APIError,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainShutdownReason {
    Unknown,
    User,
}

pub type DomainShutdownReasonEnum = Enum<DomainShutdownReason, sys::virDomainShutdownReason>;

impl_enum! {
    enum: DomainShutdownReason,
    raw: sys::virDomainShutdownReason,
    match: {
    sys::VIR_DOMAIN_SHUTDOWN_UNKNOWN => Unknown,
    sys::VIR_DOMAIN_SHUTDOWN_USER => User,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainShutoffReason {
    Unknown,
    Shutdown,
    Destroyed,
    Crashed,
    Migrated,
    Saved,
    Failed,
    FromSnapshot,
    Daemon,
}

pub type DomainShutoffReasonEnum = Enum<DomainShutoffReason, sys::virDomainShutoffReason>;

impl_enum! {
    enum: DomainShutoffReason,
    raw: sys::virDomainShutoffReason,
    match: {
    sys::VIR_DOMAIN_SHUTOFF_UNKNOWN => Unknown,
    sys::VIR_DOMAIN_SHUTOFF_SHUTDOWN => Shutdown,
    sys::VIR_DOMAIN_SHUTOFF_DESTROYED => Destroyed,
    sys::VIR_DOMAIN_SHUTOFF_CRASHED => Crashed,
    sys::VIR_DOMAIN_SHUTOFF_MIGRATED => Migrated,
    sys::VIR_DOMAIN_SHUTOFF_SAVED => Saved,
    sys::VIR_DOMAIN_SHUTOFF_FAILED => Failed,
    sys::VIR_DOMAIN_SHUTOFF_FROM_SNAPSHOT => FromSnapshot,
    sys::VIR_DOMAIN_SHUTOFF_DAEMON => Daemon,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainCrashedReason {
    Unknown,
    Panicked,
}

pub type DomainCrashedReasonEnum = Enum<DomainCrashedReason, sys::virDomainCrashedReason>;

impl_enum! {
    enum: DomainCrashedReason,
    raw: sys::virDomainCrashedReason,
    match: {
    sys::VIR_DOMAIN_CRASHED_UNKNOWN => Unknown,
    sys::VIR_DOMAIN_CRASHED_PANICKED => Panicked,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DomainPMSuspendedReason {
    Unknown,
}

pub type DomainPMSuspendedReasonEnum =
    Enum<DomainPMSuspendedReason, sys::virDomainPMSuspendedReason>;

impl_enum! {
    enum: DomainPMSuspendedReason,
    raw: sys::virDomainPMSuspendedReason,
    match: {
    sys::VIR_DOMAIN_PMSUSPENDED_UNKNOWN => Unknown,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DomainStateReason {
    NoState(DomainNoStateReasonEnum),
    Running(DomainRunningReasonEnum),
    Blocked(DomainBlockedReasonEnum),
    Paused(DomainPausedReasonEnum),
    Shutdown(DomainShutdownReasonEnum),
    Shutoff(DomainShutoffReasonEnum),
    Crashed(DomainCrashedReasonEnum),
    PMSuspended(DomainPMSuspendedReasonEnum),
}

pub type DomainStateReasonEnum = Enum<DomainStateReason, c_int>;

impl Display for DomainStateReason {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            DomainStateReason::NoState(r) => write!(f, "{}", r),
            DomainStateReason::Running(r) => write!(f, "{}", r),
            DomainStateReason::Blocked(r) => write!(f, "{}", r),
            DomainStateReason::Paused(r) => write!(f, "{}", r),
            DomainStateReason::Shutdown(r) => write!(f, "{}", r),
            DomainStateReason::Shutoff(r) => write!(f, "{}", r),
            DomainStateReason::Crashed(r) => write!(f, "{}", r),
            DomainStateReason::PMSuspended(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DomainInfo {
    /// The running state
    pub state: DomainStateEnum,
    /// The maximum memory in KBytes allowed.
    pub max_mem: u64,
    /// The memory in KBytes used by the domain.
    pub memory: u64,
    /// The number of virtual CPUs for the domain.
    pub nr_virt_cpu: u32,
    /// The CPU time used in nanoseconds.
    pub cpu_time: u64,
}

impl DomainInfo {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainInfoPtr) -> DomainInfo {
        DomainInfo {
            state: DomainStateEnum::from_raw((*ptr).state as sys::virDomainState),
            max_mem: c_ulong_to_u64((*ptr).maxMem),
            memory: c_ulong_to_u64((*ptr).memory),
            nr_virt_cpu: (*ptr).nrVirtCpu as u32,
            cpu_time: (*ptr).cpuTime,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlockStats {
    /// Number of read requests
    pub rd_req: i64,
    /// Number of read bytes
    pub rd_bytes: i64,
    /// Number of write requests
    pub wr_req: i64,
    /// Number of write bytes
    pub wr_bytes: i64,
    /// Xen specific oo_req
    pub errs: i64,
}

impl BlockStats {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainBlockStatsPtr) -> BlockStats {
        BlockStats {
            rd_req: (*ptr).rd_req,
            rd_bytes: (*ptr).rd_bytes,
            wr_req: (*ptr).wr_req,
            wr_bytes: (*ptr).wr_bytes,
            errs: (*ptr).errs,
        }
    }
}

pub struct DomainStatsRecord {
    // TODO(sahid): needs to be implemented
    pub ptr: sys::virDomainStatsRecordPtr,
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    /// Logical size in bytes of the image (how much storage the guest
    /// will see).
    pub capacity: u64,
    /// Host storage in bytes occupied by the image (such as highest
    /// allocated extent if there are no holes, similar to 'du').
    pub allocation: u64,
    /// Host physical size in bytes of the image container (last
    /// offset, similar to 'ls')
    pub physical: u64,
}

impl BlockInfo {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainBlockInfoPtr) -> BlockInfo {
        BlockInfo {
            capacity: (*ptr).capacity,
            allocation: (*ptr).allocation,
            physical: (*ptr).physical,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CpuStats {
    /// cpu usage (sum of both vcpu and hypervisor usage) in nanoseconds
    pub cpu_time: Option<u64>,
    /// cpu time charged to user instructions in nanoseconds
    pub user_time: Option<u64>,
    /// cpu time charged to system instructions in nanoseconds
    pub system_time: Option<u64>,
    /// vcpu usage in nanoseconds (cpu_time excluding hypervisor time)
    pub vcpu_time: Option<u64>,
}

macro_rules! cpu_stats_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(sys::VIR_DOMAIN_CPU_STATS_CPUTIME, UInt64, $var.cpu_time),
            $dir!(sys::VIR_DOMAIN_CPU_STATS_USERTIME, UInt64, $var.user_time),
            $dir!(
                sys::VIR_DOMAIN_CPU_STATS_SYSTEMTIME,
                UInt64,
                $var.system_time
            ),
            $dir!(sys::VIR_DOMAIN_CPU_STATS_VCPUTIME, UInt64, $var.vcpu_time),
        ]
    };
}

impl CpuStats {
    fn from_vec(vec: Vec<sys::virTypedParameter>) -> CpuStats {
        let mut ret = CpuStats::default();
        let fields = cpu_stats_fields!(param_field_in, ret);
        from_params(vec, fields);
        ret
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryParameters {
    /// Represents the maximum memory the guest can use.
    pub hard_limit: Option<u64>,
    /// Represents the memory upper limit enforced during memory
    /// contention.
    pub soft_limit: Option<u64>,
    /// Represents the minimum memory guaranteed to be reserved for
    /// the guest.
    pub min_guarantee: Option<u64>,
    /// Represents the maximum swap plus memory the guest can use.
    pub swap_hard_limit: Option<u64>,
}

macro_rules! memory_parameters_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(sys::VIR_DOMAIN_MEMORY_HARD_LIMIT, UInt64, $var.hard_limit),
            $dir!(sys::VIR_DOMAIN_MEMORY_SOFT_LIMIT, UInt64, $var.soft_limit),
            $dir!(
                sys::VIR_DOMAIN_MEMORY_MIN_GUARANTEE,
                UInt64,
                $var.min_guarantee
            ),
            $dir!(
                sys::VIR_DOMAIN_MEMORY_SWAP_HARD_LIMIT,
                UInt64,
                $var.swap_hard_limit
            ),
        ]
    };
}

impl MemoryParameters {
    pub const VALUE_UNLIMITED: u64 = sys::VIR_DOMAIN_MEMORY_PARAM_UNLIMITED;

    pub fn from_vec(vec: Vec<sys::virTypedParameter>) -> MemoryParameters {
        let mut ret = MemoryParameters::default();
        let fields = memory_parameters_fields!(param_field_in, ret);
        from_params(vec, fields);
        ret
    }

    pub fn to_vec(&self) -> Vec<sys::virTypedParameter> {
        let fields = memory_parameters_fields!(param_field_out, self);
        to_params(fields)
    }
}

macro_rules! numa_parameters_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(sys::VIR_DOMAIN_NUMA_NODESET, String, $var.node_set),
            $dir!(sys::VIR_DOMAIN_NUMA_MODE, Int32, $var.mode),
        ]
    };
}

#[derive(Clone, Debug, Default)]
pub struct NUMAParameters {
    /// Lists the numa nodeset of a domain.
    pub node_set: Option<String>,
    /// Numa mode of a domain, as an int containing a
    /// DomainNumatuneMemMode value.
    pub mode: Option<i32>,
}

impl NUMAParameters {
    pub fn from_vec(vec: Vec<sys::virTypedParameter>) -> NUMAParameters {
        let mut ret = NUMAParameters::default();
        let fields = numa_parameters_fields!(param_field_in, ret);
        from_params(vec, fields);
        ret
    }

    pub fn to_vec(&self) -> Vec<sys::virTypedParameter> {
        let fields = numa_parameters_fields!(param_field_out, self);
        to_params(fields)
    }
}

macro_rules! migrate_parameters_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(
                sys::VIR_MIGRATE_PARAM_AUTO_CONVERGE_INCREMENT,
                Int32,
                $var.auto_converge_increment
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_AUTO_CONVERGE_INITIAL,
                Int32,
                $var.auto_converge_initial
            ),
            $dir!(sys::VIR_MIGRATE_PARAM_BANDWIDTH, UInt64, $var.bandwidth),
            $dir!(
                sys::VIR_MIGRATE_PARAM_BANDWIDTH_POSTCOPY,
                UInt64,
                $var.bandwidth_postcopy
            ),
            $dir!(sys::VIR_MIGRATE_PARAM_COMPRESSION, String, $var.compression),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_MT_DTHREADS,
                Int32,
                $var.compression_mt_dthreads
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_MT_LEVEL,
                Int32,
                $var.compression_mt_level
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_MT_THREADS,
                Int32,
                $var.compression_mt_threads
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_XBZRLE_CACHE,
                UInt64,
                $var.compression_xbzrle_cache
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_ZLIB_LEVEL,
                Int32,
                $var.compression_zlib_level
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_COMPRESSION_ZSTD_LEVEL,
                Int32,
                $var.compression_zstd_level
            ),
            $dir!(sys::VIR_MIGRATE_PARAM_DEST_NAME, String, $var.dest_name),
            $dir!(sys::VIR_MIGRATE_PARAM_DEST_XML, String, $var.dest_xml),
            $dir!(sys::VIR_MIGRATE_PARAM_DISKS_PORT, Int32, $var.disks_port),
            $dir!(sys::VIR_MIGRATE_PARAM_DISKS_URI, String, $var.disks_uri),
            $dir!(
                sys::VIR_MIGRATE_PARAM_GRAPHICS_URI,
                String,
                $var.graphics_uri
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_LISTEN_ADDRESS,
                String,
                $var.listen_address
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_MIGRATE_DISKS,
                VecString,
                $var.migrate_disks
            ),
            $dir!(
                sys::VIR_MIGRATE_PARAM_PARALLEL_CONNECTIONS,
                Int32,
                $var.parallel_connections
            ),
            $dir!(sys::VIR_MIGRATE_PARAM_PERSIST_XML, String, $var.persist_xml),
            $dir!(
                sys::VIR_MIGRATE_PARAM_TLS_DESTINATION,
                String,
                $var.tls_destination
            ),
            $dir!(sys::VIR_MIGRATE_PARAM_URI, String, $var.uri),
        ]
    };
}

#[derive(Clone, Debug, Default)]
pub struct MigrateParameters {
    pub auto_converge_increment: Option<i32>,
    pub auto_converge_initial: Option<i32>,
    pub bandwidth: Option<u64>,
    pub bandwidth_postcopy: Option<u64>,
    pub compression: Option<String>,
    pub compression_mt_dthreads: Option<i32>,
    pub compression_mt_level: Option<i32>,
    pub compression_mt_threads: Option<i32>,
    pub compression_xbzrle_cache: Option<u64>,
    pub compression_zlib_level: Option<i32>,
    pub compression_zstd_level: Option<i32>,
    pub dest_name: Option<String>,
    pub dest_xml: Option<String>,
    pub disks_port: Option<i32>,
    pub disks_uri: Option<String>,
    pub graphics_uri: Option<String>,
    pub listen_address: Option<String>,
    pub migrate_disks: Vec<String>,
    pub parallel_connections: Option<i32>,
    pub persist_xml: Option<String>,
    pub tls_destination: Option<String>,
    pub uri: Option<String>,
}

impl MigrateParameters {
    pub fn from_vec(vec: Vec<sys::virTypedParameter>) -> MigrateParameters {
        let mut ret = MigrateParameters::default();
        let fields = migrate_parameters_fields!(param_field_in, ret);
        from_params(vec, fields);
        ret
    }

    pub fn to_vec(&self) -> Vec<sys::virTypedParameter> {
        let fields = migrate_parameters_fields!(param_field_out, self);
        to_params(fields)
    }
}

#[derive(Clone, Debug)]
pub struct IPAddress {
    pub typed: i64,
    pub addr: String,
    pub prefix: u64,
}

impl IPAddress {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainIPAddressPtr) -> IPAddress {
        IPAddress {
            typed: (*ptr).type_ as i64,
            addr: c_chars_to_string!((*ptr).addr),
            prefix: (*ptr).prefix as u64,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    pub hwaddr: String,
    pub naddrs: u64,
    pub addrs: Vec<IPAddress>,
}

impl Interface {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainInterfacePtr) -> Interface {
        let naddrs = (*ptr).naddrs;
        let mut addrs = vec![];
        for x in 0..naddrs as isize {
            addrs.push(IPAddress::from_ptr((*ptr).addrs.offset(x)));
        }
        Interface {
            name: c_chars_to_string!((*ptr).name),
            hwaddr: c_chars_to_string!((*ptr).hwaddr),
            naddrs: naddrs as u64,
            addrs,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InterfaceStats {
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
}

impl InterfaceStats {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainInterfaceStatsPtr) -> InterfaceStats {
        InterfaceStats {
            rx_bytes: (*ptr).rx_bytes,
            rx_packets: (*ptr).rx_packets,
            rx_errs: (*ptr).rx_errs,
            rx_drop: (*ptr).rx_drop,
            tx_bytes: (*ptr).tx_bytes,
            tx_packets: (*ptr).tx_packets,
            tx_errs: (*ptr).tx_errs,
            tx_drop: (*ptr).tx_drop,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemoryStat {
    pub tag: u32,
    pub val: u64,
}

impl MemoryStat {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: *const sys::virDomainMemoryStatStruct) -> MemoryStat {
        MemoryStat {
            tag: (*ptr).tag as u32,
            val: (*ptr).val,
        }
    }
}

/// Information about the progress of a background job that is
/// affecting a domain.
#[derive(Clone, Debug, Default)]
pub struct JobStats {
    pub r#type: i32,

    pub auto_converge_throttle: Option<i32>,

    pub compression_bytes: Option<u64>,
    pub compression_cache: Option<u64>,
    pub compression_cache_misses: Option<u64>,
    pub compression_overflow: Option<u64>,
    pub compression_pages: Option<u64>,

    pub data_processed: Option<u64>,
    pub data_remaining: Option<u64>,
    pub data_total: Option<u64>,

    pub disk_bps: Option<u64>,
    pub disk_processed: Option<u64>,
    pub disk_remaining: Option<u64>,
    pub disk_temp_total: Option<u64>,
    pub disk_temp_used: Option<u64>,
    pub disk_total: Option<u64>,

    pub downtime: Option<u64>,
    pub downtime_net: Option<u64>,

    pub error_message: Option<String>,

    pub mem_bps: Option<u64>,
    pub mem_constant: Option<u64>,
    pub mem_dirty_rate: Option<u64>,
    pub mem_iteration: Option<u64>,
    pub mem_normal: Option<u64>,
    pub mem_normal_bytes: Option<u64>,
    pub mem_page_size: Option<u64>,
    pub mem_postcopy_reqs: Option<u64>,
    pub mem_processed: Option<u64>,
    pub mem_remaining: Option<u64>,
    pub mem_total: Option<u64>,

    pub operation: Option<i32>,

    pub setup_time: Option<u64>,

    pub success: Option<bool>,

    pub time_elapsed: Option<u64>,
    pub time_elapsed_net: Option<u64>,
    pub time_remaining: Option<u64>,
}

macro_rules! job_stats_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(
                sys::VIR_DOMAIN_JOB_AUTO_CONVERGE_THROTTLE,
                Int32,
                $var.auto_converge_throttle
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_COMPRESSION_BYTES,
                UInt64,
                $var.compression_bytes
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_COMPRESSION_CACHE,
                UInt64,
                $var.compression_cache
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_COMPRESSION_CACHE_MISSES,
                UInt64,
                $var.compression_cache_misses
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_COMPRESSION_OVERFLOW,
                UInt64,
                $var.compression_overflow
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_COMPRESSION_PAGES,
                UInt64,
                $var.compression_pages
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_DATA_PROCESSED,
                UInt64,
                $var.data_processed
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_DATA_REMAINING,
                UInt64,
                $var.data_remaining
            ),
            $dir!(sys::VIR_DOMAIN_JOB_DATA_TOTAL, UInt64, $var.data_total),
            $dir!(sys::VIR_DOMAIN_JOB_DISK_BPS, UInt64, $var.disk_bps),
            $dir!(
                sys::VIR_DOMAIN_JOB_DISK_PROCESSED,
                UInt64,
                $var.disk_processed
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_DISK_REMAINING,
                UInt64,
                $var.disk_remaining
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_DISK_TEMP_TOTAL,
                UInt64,
                $var.disk_temp_total
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_DISK_TEMP_USED,
                UInt64,
                $var.disk_temp_used
            ),
            $dir!(sys::VIR_DOMAIN_JOB_DISK_TOTAL, UInt64, $var.disk_total),
            $dir!(sys::VIR_DOMAIN_JOB_DOWNTIME, UInt64, $var.downtime),
            $dir!(sys::VIR_DOMAIN_JOB_DOWNTIME_NET, UInt64, $var.downtime_net),
            $dir!(sys::VIR_DOMAIN_JOB_ERRMSG, String, $var.error_message),
            $dir!(sys::VIR_DOMAIN_JOB_MEMORY_BPS, UInt64, $var.mem_bps),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_CONSTANT,
                UInt64,
                $var.mem_constant
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_DIRTY_RATE,
                UInt64,
                $var.mem_dirty_rate
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_ITERATION,
                UInt64,
                $var.mem_iteration
            ),
            $dir!(sys::VIR_DOMAIN_JOB_MEMORY_NORMAL, UInt64, $var.mem_normal),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_NORMAL_BYTES,
                UInt64,
                $var.mem_normal_bytes
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_PAGE_SIZE,
                UInt64,
                $var.mem_page_size
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_POSTCOPY_REQS,
                UInt64,
                $var.mem_postcopy_reqs
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_PROCESSED,
                UInt64,
                $var.mem_processed
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_MEMORY_REMAINING,
                UInt64,
                $var.mem_remaining
            ),
            $dir!(sys::VIR_DOMAIN_JOB_MEMORY_TOTAL, UInt64, $var.mem_total),
            $dir!(sys::VIR_DOMAIN_JOB_OPERATION, Int32, $var.operation),
            $dir!(sys::VIR_DOMAIN_JOB_SETUP_TIME, UInt64, $var.setup_time),
            $dir!(sys::VIR_DOMAIN_JOB_SUCCESS, Bool, $var.success),
            $dir!(sys::VIR_DOMAIN_JOB_TIME_ELAPSED, UInt64, $var.time_elapsed),
            $dir!(
                sys::VIR_DOMAIN_JOB_TIME_ELAPSED_NET,
                UInt64,
                $var.time_elapsed_net
            ),
            $dir!(
                sys::VIR_DOMAIN_JOB_TIME_REMAINING,
                UInt64,
                $var.time_remaining
            ),
        ]
    };
}

impl From<(i32, Vec<sys::virTypedParameter>)> for JobStats {
    fn from((r#type, params): (i32, Vec<sys::virTypedParameter>)) -> Self {
        let mut stats = Self {
            r#type,
            ..Default::default()
        };

        let fields = job_stats_fields!(param_field_in, stats);

        from_params(params, fields);

        stats
    }
}

/// Structure representing the CFS scheduler cpu bandwidth parameters
/// see <https://www.kernel.org/doc/html/latest/scheduler/sched-bwc.html>
#[derive(Clone, Debug, Default)]
pub struct SchedBandwidth {
    pub period: Option<u64>,
    pub quota: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct SchedulerInfo {
    pub scheduler_type: String,
    // cpu shares for the domain.
    pub cpu_shares: Option<u64>,
    // Bandwidth allocated for the vcpu threads.
    pub vcpu_bw: SchedBandwidth,
    // Bandwidth allocated for the emulator threads.
    pub emulator_bw: SchedBandwidth,
    // Bandwidth allocated for the Domain.
    pub global_bw: SchedBandwidth,
    // Bandwidth allocated for the io threads..
    pub iothread_bw: SchedBandwidth,
    // Credit scheduler relative weight
    pub weight: Option<u32>,
    // Credit scheduler cap
    pub cap: Option<u32>,
    // Allocation scheduler reservation
    pub reservation: Option<i64>,
    // Allocation scheduler limit
    pub limit: Option<i64>,
    // Allocation scheduler shares
    pub shares: Option<i32>,
}

macro_rules! scheduler_info_fields {
    ($dir:ident, $var:ident) => {
        vec![
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_CPU_SHARES,
                UInt64,
                $var.cpu_shares
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_VCPU_PERIOD,
                UInt64,
                $var.vcpu_bw.period
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_VCPU_QUOTA,
                Int64,
                $var.vcpu_bw.quota
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_EMULATOR_PERIOD,
                UInt64,
                $var.emulator_bw.period
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_EMULATOR_QUOTA,
                Int64,
                $var.emulator_bw.quota
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_GLOBAL_PERIOD,
                UInt64,
                $var.global_bw.period
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_GLOBAL_QUOTA,
                Int64,
                $var.global_bw.quota
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_IOTHREAD_PERIOD,
                UInt64,
                $var.iothread_bw.period
            ),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_IOTHREAD_QUOTA,
                Int64,
                $var.iothread_bw.quota
            ),
            $dir!(sys::VIR_DOMAIN_SCHEDULER_WEIGHT, UInt32, $var.weight),
            $dir!(sys::VIR_DOMAIN_SCHEDULER_CAP, UInt32, $var.cap),
            $dir!(
                sys::VIR_DOMAIN_SCHEDULER_RESERVATION,
                Int64,
                $var.reservation
            ),
            $dir!(sys::VIR_DOMAIN_SCHEDULER_LIMIT, Int64, $var.limit),
            $dir!(sys::VIR_DOMAIN_SCHEDULER_SHARES, Int32, $var.shares),
        ]
    };
}

impl SchedulerInfo {
    pub fn from_vec(vec: Vec<sys::virTypedParameter>, scheduler_type: String) -> SchedulerInfo {
        let mut ret = SchedulerInfo {
            scheduler_type,
            ..Default::default()
        };
        let fields = scheduler_info_fields!(param_field_in, ret);
        from_params(vec, fields);
        ret
    }

    pub fn to_vec(&self) -> Vec<sys::virTypedParameter> {
        let fields = scheduler_info_fields!(param_field_out, self);
        to_params(fields)
    }
}

/// Provides APIs for the management of domains.
///
/// See <https://libvirt.org/html/libvirt-libvirt-domain.html>
#[derive(Debug)]
pub struct Domain {
    ptr: sys::virDomainPtr,
}

unsafe impl Send for Domain {}
unsafe impl Sync for Domain {}

impl Drop for Domain {
    fn drop(&mut self) {
        if let Err(e) = check_neg!(unsafe { sys::virDomainFree(self.as_ptr()) }) {
            panic!("Unable to drop reference on domain: {e}")
        }
    }
}

impl Clone for Domain {
    /// Creates a copy of a domain.
    ///
    /// Increments the internal reference counter on the given
    /// domain.
    fn clone(&self) -> Self {
        if let Err(e) = check_neg!(unsafe { sys::virDomainRef(self.as_ptr()) }) {
            panic!("Unable to add reference on domain: {e}")
        }
        unsafe { Domain::from_ptr(self.as_ptr()) }
    }
}

impl Domain {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    /// The rust wrapper will own the reference count
    /// for the C object upon return.
    pub unsafe fn from_ptr(ptr: sys::virDomainPtr) -> Domain {
        Domain { ptr }
    }

    /// # Safety
    ///
    /// The pointer returned by this method is a copy of
    /// a pointer that is normally tracked by reference
    /// counting in the underlying implementation. Creating
    /// a copy of the pointer explicitly circumvents that
    /// reference counting. The returned pointer may be
    /// invalidated if this object is dropped.
    pub unsafe fn as_ptr(&self) -> sys::virDomainPtr {
        self.ptr
    }

    pub fn connect(&self) -> Result<Connect, Error> {
        let ptr = check_null!(unsafe { sys::virDomainGetConnect(self.as_ptr()) })?;
        if let Err(e) = check_neg!(unsafe { sys::virConnectRef(ptr) }) {
            panic!("Unable to add reference on connection: {e}")
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    /// Extracts domain state.
    ///
    /// Each state can be accompanied with a reason (if known) which
    /// led to the state.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetState>
    pub fn state(&self) -> Result<(DomainStateEnum, DomainStateReasonEnum), Error> {
        let mut state: c_int = -1;
        let mut reason: c_int = -1;
        let _ = check_neg!(unsafe {
            sys::virDomainGetState(self.as_ptr(), &mut state, &mut reason, 0)
        });
        let state = DomainStateEnum::from_raw(state as sys::virDomainState);
        let reason = match state {
            Enum::Known(k) => DomainStateReasonEnum::Known(match k {
                DomainState::NoState => DomainStateReason::NoState(
                    DomainNoStateReasonEnum::from_raw(reason as sys::virDomainNostateReason),
                ),
                DomainState::Running => DomainStateReason::Running(
                    DomainRunningReasonEnum::from_raw(reason as sys::virDomainRunningReason),
                ),
                DomainState::Blocked => DomainStateReason::Blocked(
                    DomainBlockedReasonEnum::from_raw(reason as sys::virDomainBlockedReason),
                ),
                DomainState::Paused => DomainStateReason::Paused(DomainPausedReasonEnum::from_raw(
                    reason as sys::virDomainPausedReason,
                )),
                DomainState::Shutdown => DomainStateReason::Shutdown(
                    DomainShutdownReasonEnum::from_raw(reason as sys::virDomainShutdownReason),
                ),
                DomainState::Shutoff => DomainStateReason::Shutoff(
                    DomainShutoffReasonEnum::from_raw(reason as sys::virDomainShutoffReason),
                ),
                DomainState::Crashed => DomainStateReason::Crashed(
                    DomainCrashedReasonEnum::from_raw(reason as sys::virDomainCrashedReason),
                ),
                DomainState::PMSuspended => {
                    DomainStateReason::PMSuspended(DomainPMSuspendedReasonEnum::from_raw(
                        reason as sys::virDomainPMSuspendedReason,
                    ))
                }
            }),
            Enum::Unknown(_) => DomainStateReasonEnum::Unknown(reason),
        };
        Ok((state, reason))
    }

    /// Get the public name of the domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetName>
    pub fn name(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virDomainGetName(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Get the type of domain operating system.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetOSType>
    pub fn os_type(&self) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virDomainGetOSType(self.as_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Get the hostname for that domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetHostname>
    pub fn hostname(&self, flags: u32) -> Result<String, Error> {
        let n = check_null!(unsafe { sys::virDomainGetHostname(self.as_ptr(), flags as c_uint) })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Returns the domain UUID
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetUUID>
    pub fn uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [c_uchar; sys::VIR_UUID_BUFLEN as usize] = [0; sys::VIR_UUID_BUFLEN as usize];
        let _ = check_neg!(unsafe { sys::virDomainGetUUID(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(Uuid::from_bytes(uuid))
    }

    /// Get the UUID for a domain as string.
    ///
    /// For more information about UUID see RFC4122.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetUUIDString>
    pub fn uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let _ =
            check_neg!(unsafe { sys::virDomainGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) })?;
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Get the hypervisor ID number for the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetID>
    pub fn id(&self) -> Option<u32> {
        let ret = unsafe { sys::virDomainGetID(self.as_ptr()) };
        if ret as i32 == -1 {
            return None;
        }
        Some(ret)
    }

    /// Provide an XML description of the domain. The description may
    /// be reused later to relaunch the domain with [`create_domain_xml()`].
    ///
    /// [`create_domain_xml()`]: Connect::create_domain_xml
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetXMLDesc>
    pub fn xml_desc(&self, flags: sys::virDomainXMLFlags) -> Result<String, Error> {
        let xml = check_null!(unsafe { sys::virDomainGetXMLDesc(self.as_ptr(), flags) })?;
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools. The domain will
    /// be paused only if restoring from managed state created from a
    /// paused domain.For more control, see [`create_with_flags()`].
    ///
    /// [`create_with_flags()`]: Domain::create_with_flags
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainCreate>
    pub fn create(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainCreate(self.as_ptr()) })?;
        Ok(())
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainCreateWithFlags>
    pub fn create_with_flags(&self, flags: sys::virDomainCreateFlags) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainCreateWithFlags(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Extract information about a domain. Note that if the
    /// connection used to get the domain is limited only a partial
    /// set of the information can be extracted.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetInfo>
    pub fn info(&self) -> Result<DomainInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let _ = check_neg!(unsafe { sys::virDomainGetInfo(self.as_ptr(), pinfo.as_mut_ptr()) })?;
        Ok(unsafe { DomainInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainDestroy>
    pub fn destroy(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainDestroy(self.as_ptr()) })?;
        Ok(())
    }

    /// Reset a domain immediately without any guest OS shutdown.
    /// Reset emulates the power reset button on a machine, where all
    /// hardware sees the RST line set and reinitializes internal
    /// state.
    ///
    /// Note that there is a risk of data loss caused by reset without
    /// any guest OS shutdown.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainReset>
    pub fn reset(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainReset(self.as_ptr(), 0) })?;
        Ok(())
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainDestroyFlags>
    pub fn destroy_flags(&self, flags: sys::virDomainDestroyFlagsValues) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainDestroyFlags(self.as_ptr(), flags) })?;
        Ok(())
    }

    /// Shutdown a domain
    ///
    /// The domain object is still usable thereafter, but the domain
    /// OS is being stopped. Note that the guest OS may ignore the
    /// request. Additionally, the hypervisor may check and support
    /// the domain 'on_poweroff' XML setting resulting in a domain
    /// that reboots instead of shutting down. For guests that react
    /// to a shutdown request, the differences from [`destroy()`] are
    /// that the guests disk storage will be in a stable state rather
    /// than having the (virtual) power cord pulled, and this command
    /// returns as soon as the shutdown request is issued rather than
    /// blocking until the guest is no longer running.
    ///
    /// [`destroy()`]: Domain::destroy
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainShutdown>
    pub fn shutdown(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainShutdown(self.as_ptr()) })?;
        Ok(())
    }

    /// Shutdown a domain, the domain object is still usable thereafter
    /// but the domain OS is being stopped. Note that the guest OS may
    /// ignore the request. Additionally, the hypervisor may check and
    /// support the domain 'on_poweroff' XML setting resulting in a domain
    /// that reboots instead of shutting down. For guests that react to a
    /// shutdown request, the differences from [`Domain::destroy()`] are that
    /// the guest's disk storage will be in a stable state rather
    /// than having the (virtual) power cord pulled, and this command returns
    /// as soon as the shutdown request is issued rather than blocking until
    /// the guest is no longer running.
    ///
    /// If the domain is transient and has any snapshot metadata
    /// (see virDomainSnapshotNum()), then that metadata will automatically
    /// be deleted when the domain quits.
    ///
    /// If flags is set to zero, then the hypervisor will choose the method of
    /// shutdown it considers best. To have greater control pass one or more of
    /// the [`sys::virDomainShutdownFlagValues`]. The order in which the hypervisor tries
    /// each shutdown method is undefined, and a hypervisor is not required to
    /// support all methods.
    ///
    /// To use guest agent [`sys::VIR_DOMAIN_SHUTDOWN_GUEST_AGENT`] the domain XML must
    /// have \<channel\> configured.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainShutdownFlags>
    pub fn shutdown_flags(&self, flags: sys::virDomainShutdownFlagValues) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainShutdownFlags(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Reboot a domain.
    ///
    /// The domain object is still usable thereafter.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainReboot>
    pub fn reboot(&self, flags: sys::virDomainRebootFlagValues) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainReboot(self.as_ptr(), flags) })?;
        Ok(())
    }

    /// Suspend a domain.
    ///
    /// Suspends an active domain, the process is frozen without
    /// further access to CPU resources and I/O but the memory used by
    /// the domain at the hypervisor level will stay allocated. Use
    /// `resume` to reactivate the domain.  This function may
    /// require privileged access.  Moreover, suspend may not be
    /// supported if domain is in some special state like
    /// [`PMSuspended`].
    ///
    /// [`PMSuspended`]: DomainState::PMSuspended
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSuspend>
    pub fn suspend(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainSuspend(self.as_ptr()) })?;
        Ok(())
    }

    /// Resume a suspended domain.
    ///
    /// the process is restarted from the state where it was frozen by
    /// calling [`suspend()`]. This function may require privileged
    /// access Moreover, resume may not be supported if domain is in
    /// some special state like [`PMSuspended`].
    ///
    /// [`suspend()`]: Domain::suspend
    /// [`PMSuspended`]: DomainState::PMSuspended
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainResume>
    pub fn resume(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainResume(self.as_ptr()) })?;
        Ok(())
    }

    /// Wakeup the domain from a power-management sleep state
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainPMWakeup>
    pub fn pm_wakeup(&self, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainPMWakeup(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Determine if the domain is currently running.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainIsActive>
    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virDomainIsActive(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Determine if the domain has a persistent configuration which means it will still exist
    /// after shutting down.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainIsPersistent>
    pub fn is_persistent(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virDomainIsPersistent(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainUndefine>
    pub fn undefine(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainUndefine(self.as_ptr()) })?;
        Ok(())
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainUndefineFlags>
    pub fn undefine_flags(&self, flags: sys::virDomainUndefineFlagsValues) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainUndefineFlags(self.as_ptr(), flags) })?;
        Ok(())
    }

    /// Determine if the active domain configuration is updated
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainIsUpdated>
    pub fn is_updated(&self) -> Result<bool, Error> {
        let ret = check_neg!(unsafe { sys::virDomainIsUpdated(self.as_ptr()) })?;
        Ok(ret == 1)
    }

    /// Returns the domain autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetAutostart>
    pub fn autostart(&self) -> Result<bool, Error> {
        let mut autostart: c_int = 0;
        let _ = check_neg!(unsafe { sys::virDomainGetAutostart(self.as_ptr(), &mut autostart) })?;
        Ok(autostart == 1)
    }

    /// Updates the domain autostart behaviour
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetAutostart>
    pub fn set_autostart(&self, autostart: bool) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainSetAutostart(self.as_ptr(), autostart as c_int) })?;
        Ok(())
    }

    /// Updates the maximum memory limit
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMaxMemory>
    pub fn set_max_memory(&self, memory: u64) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainSetMaxMemory(self.as_ptr(), memory as c_ulong) })?;
        Ok(())
    }

    /// Returns the maximum memory limit
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetMaxMemory>
    pub fn max_memory(&self) -> Result<u64, Error> {
        let ret = check_zero!(unsafe { sys::virDomainGetMaxMemory(self.as_ptr()) })?;
        Ok(c_ulong_to_u64(ret))
    }

    /// Returns the maximum vCPU count
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetMaxVcpus>
    pub fn max_vcpus(&self) -> Result<u64, Error> {
        let ret = check_neg!(unsafe { sys::virDomainGetMaxVcpus(self.as_ptr()) })?;
        Ok(ret as u64)
    }

    /// Updates the domain memory balloon target
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMemory>
    pub fn set_memory(&self, memory: u64) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainSetMemory(self.as_ptr(), memory as c_ulong) });
        Ok(())
    }

    /// Updates the domain memory balloon target
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMemoryFlags>
    pub fn set_memory_flags(
        &self,
        memory: u64,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainSetMemoryFlags(self.as_ptr(), memory as c_ulong, flags as c_uint)
        })?;
        Ok(())
    }

    /// Updates the memory stats polling interval
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMemoryStatsPeriod>
    pub fn set_memory_stats_period(
        &self,
        period: i32,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainSetMemoryStatsPeriod(self.as_ptr(), period as c_int, flags as c_uint)
        })?;
        Ok(())
    }

    /// Updates the domain vCPU count
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetVcpus>
    pub fn set_vcpus(&self, vcpus: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainSetVcpus(self.as_ptr(), vcpus as c_uint) })?;
        Ok(())
    }

    /// Updates the domain vCPU count
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetVcpusFlags>
    pub fn set_vcpus_flags(&self, vcpus: u32, flags: sys::virDomainVcpuFlags) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainSetVcpusFlags(self.as_ptr(), vcpus as c_uint, flags as c_uint)
        })?;
        Ok(())
    }

    /// Returns the domani vCPU count
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetVcpusFlags>
    pub fn vcpus_flags(&self, flags: sys::virDomainVcpuFlags) -> Result<u32, Error> {
        let ret =
            check_neg!(unsafe { sys::virDomainGetVcpusFlags(self.as_ptr(), flags as c_uint) })?;
        Ok(ret as u32)
    }

    /// Updates the maximum domain migration data rate
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateSetMaxSpeed>
    pub fn migrate_set_max_speed(&self, bandwidth: u64, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateSetMaxSpeed(self.as_ptr(), bandwidth as c_ulong, flags as c_uint)
        })?;
        Ok(())
    }

    /// Returns the maximum domain migration data rate
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateGetMaxSpeed>
    pub fn migrate_max_speed(&self, flags: u32) -> Result<u64, Error> {
        let mut bandwidth: c_ulong = 0;
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateGetMaxSpeed(self.as_ptr(), &mut bandwidth, flags as c_uint)
        })?;
        Ok(c_ulong_to_u64(bandwidth))
    }

    /// Updates the domain migration compression cache size
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateSetCompressionCache>
    pub fn migrate_set_compression_cache(&self, size: u64, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateSetCompressionCache(
                self.as_ptr(),
                size as c_ulonglong,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returnjs the domain migration compression cache size
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateGetCompressionCache>
    pub fn migrate_compression_cache(&self, flags: u32) -> Result<u64, Error> {
        let mut size: c_ulonglong = 0;
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateGetCompressionCache(self.as_ptr(), &mut size, flags as c_uint)
        })?;
        Ok(size)
    }

    /// Updates the domain migration permitted downtime
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateSetMaxDowntime>
    pub fn migrate_set_max_downtime(&self, downtime: u64, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateSetMaxDowntime(
                self.as_ptr(),
                downtime as c_ulonglong,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Updates the domain virtual clock
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetTime>
    pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainSetTime(
                self.as_ptr(),
                seconds as c_longlong,
                nseconds as c_uint,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returns the domain virtual clock
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetTime>
    pub fn time(&self, flags: u32) -> Result<(i64, i32), Error> {
        let mut seconds: c_longlong = 0;
        let mut nseconds: c_uint = 0;
        let _ = check_neg!(unsafe {
            sys::virDomainGetTime(self.as_ptr(), &mut seconds, &mut nseconds, flags as c_uint)
        })?;
        Ok((seconds, nseconds as i32))
    }

    /// Returns information for a domain block device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetBlockInfo>
    pub fn block_info(&self, disk: &str, flags: u32) -> Result<BlockInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let disk_buf = CString::new(disk)?;
        let _ = check_neg!(unsafe {
            sys::virDomainGetBlockInfo(
                self.as_ptr(),
                disk_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                flags as c_uint,
            )
        });
        Ok(unsafe { BlockInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    /// Returns I/O statistics for a domain block device
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainBlockStats>
    pub fn block_stats(&self, disk: &str) -> Result<BlockStats, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let disk_buf = CString::new(disk)?;
        let _ = check_neg!(unsafe {
            sys::virDomainBlockStats(
                self.as_ptr(),
                disk_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                mem::size_of::<sys::virDomainBlockStatsStruct>(),
            )
        })?;
        Ok(unsafe { BlockStats::from_ptr(&mut pinfo.assume_init()) })
    }

    /// Pin a domain vCPU to host pCPUs
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainPinVcpu>
    pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainPinVcpu(
                self.as_ptr(),
                vcpu as c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as c_int,
            )
        })?;
        Ok(())
    }

    /// Pin a domain vCPU to host pCPUs
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainPinVcpuFlags>
    pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainPinVcpuFlags(
                self.as_ptr(),
                vcpu as c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as c_int,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Pin the domain emulator threads to host pCPUs
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainPinEmulator>
    pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainPinEmulator(
                self.as_ptr(),
                cpumap.as_ptr() as *mut _,
                cpumap.len() as c_int,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Rename a domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainRename>
    pub fn rename(&self, new_name: &str, flags: u32) -> Result<(), Error> {
        let new_name_buf = CString::new(new_name)?;
        let _ = check_neg!(unsafe {
            sys::virDomainRename(self.as_ptr(), new_name_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(())
    }

    /// Set the guest OS user password
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetUserPassword>
    pub fn set_user_password(&self, user: &str, password: &str, flags: u32) -> Result<(), Error> {
        let user_buf = CString::new(user)?;
        let password_buf = CString::new(password)?;
        let _ = check_neg!(unsafe {
            sys::virDomainSetUserPassword(
                self.as_ptr(),
                user_buf.as_ptr(),
                password_buf.as_ptr(),
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Set the threshold for block device events
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetBlockThreshold>
    pub fn set_block_threshold(&self, dev: &str, threshold: u64, flags: u32) -> Result<(), Error> {
        let dev_buf = CString::new(dev)?;
        let _ = check_neg!(unsafe {
            sys::virDomainSetBlockThreshold(
                self.as_ptr(),
                dev_buf.as_ptr(),
                threshold as c_ulonglong,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Connect a file descriptor to the domain graphical console
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainOpenGraphics>
    pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainOpenGraphics(self.as_ptr(), idx as c_uint, fd as c_int, flags as c_uint)
        })?;
        Ok(())
    }

    /// Return a file descriptor connected to the domain graphical console
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainOpenGraphicsFD>
    pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Result<u32, Error> {
        let ret = check_neg!(unsafe {
            sys::virDomainOpenGraphicsFD(self.as_ptr(), idx as c_uint, flags as c_uint)
        })?;
        Ok(ret as u32)
    }

    /// Connect a stream to a domain channel
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainOpenChannel>
    pub fn open_channel(
        &self,
        name: Option<&str>,
        stream: &Stream,
        flags: u32,
    ) -> Result<(), Error> {
        let name_buf = some_string_to_cstring!(name);
        let _ = check_neg!(unsafe {
            sys::virDomainOpenChannel(
                self.as_ptr(),
                some_cstring_to_c_chars!(name_buf),
                stream.as_ptr(),
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Connect a stream to a domain serial port or console
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainOpenConsole>
    pub fn open_console(
        &self,
        name: Option<&str>,
        stream: &Stream,
        flags: u32,
    ) -> Result<(), Error> {
        let name_buf = some_string_to_cstring!(name);
        let _ = check_neg!(unsafe {
            sys::virDomainOpenConsole(
                self.as_ptr(),
                some_cstring_to_c_chars!(name_buf),
                stream.as_ptr(),
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Return a list of guest OS interfaces
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainInterfaceAddresses>
    pub fn interface_addresses(
        &self,
        source: sys::virDomainInterfaceAddressesSource,
        flags: u32,
    ) -> Result<Vec<Interface>, Error> {
        let mut addresses: *mut sys::virDomainInterfacePtr = ptr::null_mut();
        let size = check_neg!(unsafe {
            sys::virDomainInterfaceAddresses(self.as_ptr(), &mut addresses, source, flags)
        })?;

        let mut array: Vec<Interface> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { Interface::from_ptr(*addresses.offset(x)) });
        }
        unsafe { libc::free(addresses as *mut c_void) };

        Ok(array)
    }

    /// Return statistics for a domain network interface
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainInterfaceStats>
    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let path_buf = CString::new(path)?;
        let _ = check_neg!(unsafe {
            sys::virDomainInterfaceStats(
                self.as_ptr(),
                path_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                mem::size_of::<sys::virDomainInterfaceStatsStruct>(),
            )
        })?;
        Ok(unsafe { InterfaceStats::from_ptr(&mut pinfo.assume_init()) })
    }

    /// Returns statistics for guest OS memory usage
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMemoryStats>
    pub fn memory_stats(&self, flags: u32) -> Result<Vec<MemoryStat>, Error> {
        let mut pinfo: Vec<sys::virDomainMemoryStatStruct> =
            Vec::with_capacity(sys::VIR_DOMAIN_MEMORY_STAT_NR as usize);
        let ret = check_neg!(unsafe {
            sys::virDomainMemoryStats(
                self.as_ptr(),
                pinfo.as_mut_ptr(),
                sys::VIR_DOMAIN_MEMORY_STAT_NR,
                flags as c_uint,
            )
        })?;
        // low-level operation that is confirmed by return from
        // libvirt.
        unsafe { pinfo.set_len(ret as usize) };

        let mut stats: Vec<MemoryStat> = Vec::with_capacity(ret as usize);
        for x in pinfo.iter().take(ret as usize) {
            stats.push(unsafe { MemoryStat::from_ptr(x) });
        }
        Ok(stats)
    }

    /// Get statistics for a single domain's CPU usage.
    ///
    /// This method will return an error if the hypervisor is using cgroups v2 and attempts
    /// to retrieve per-cpu statistics `(start_cpu != -1 && ncpus != 1)`
    /// ## Arguments
    /// * `start_cpu` - which cpu to start with, or `-1` for summary
    /// * `ncpus` - how many cpus to query
    /// * `flags` - currently unused, caller should pass `0`
    ///
    /// ## Examples
    /// Retrieve stats for all CPUs:
    ///
    /// [`domain::cpu_stats(-1, 1, 0)`]
    ///
    /// Retrieve stats for CPU 0 (cgroups v1 only):
    ///
    /// [`domain::cpu_stats(0, 1, 0)`]
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetCPUStats>
    pub fn cpu_stats(
        &self,
        start_cpu: i32,
        ncpus: u32,
        flags: u32,
    ) -> Result<Vec<CpuStats>, Error> {
        // As special cases, if params is NULL and nparams is 0 and ncpus is 1, and the return value
        // will be how many statistics are available for the given start_cpu. This number may be
        // different for start_cpu of -1 than for any non-negative value, but will be the same for
        // all non-negative start_cpu. Likewise, if params is NULL and nparams is 0 and ncpus is 0,
        // the number of cpus available to query is returned. From the host perspective, this would
        // typically match the cpus member of virNodeGetInfo(), but might be less due to host cpu hotplug.
        let nparams = check_neg!(unsafe {
            sys::virDomainGetCPUStats(self.as_ptr(), ptr::null_mut(), 0, start_cpu, 1, flags)
        })?;

        let total_params = (nparams as usize) * (ncpus as usize);
        let mut params: Vec<sys::virTypedParameter> =
            unsafe { vec![std::mem::zeroed(); total_params] };

        let result = check_neg!(unsafe {
            sys::virDomainGetCPUStats(
                self.as_ptr(),
                params.as_mut_ptr(),
                params.len() as u32,
                start_cpu,
                ncpus,
                flags,
            )
        })?;

        let mut cpu_stats = Vec::new();

        for i in 0..ncpus as usize {
            let start_idx = i * (nparams as usize);
            let end_idx = start_idx + result as usize;

            let cpu_params = params[start_idx..end_idx].to_vec();
            cpu_stats.push(CpuStats::from_vec(cpu_params));
        }

        Ok(cpu_stats)
    }

    /// Get progress statistics about a background job running on this domain.
    /// This method will return an error if the domain isn't active
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetJobStats>
    pub fn job_stats(&self, flags: sys::virDomainGetJobStatsFlags) -> Result<JobStats, Error> {
        let mut r#type: c_int = 0;

        // We allow libvirt to allocate the params structure for us. libvirt will populate
        // nparams with the number of typed params returned.
        let mut nparams: c_int = 0;
        let mut params: sys::virTypedParameterPtr = ptr::null_mut();

        let _ = check_neg!(unsafe {
            sys::virDomainGetJobStats(
                self.as_ptr(),
                &mut r#type,
                &mut params,
                &mut nparams,
                flags as c_uint,
            )
        })?;

        let res: Vec<sys::virTypedParameter> =
            unsafe { Vec::from_raw_parts(params, nparams as usize, nparams as usize) };

        Ok((r#type, res).into())
    }

    /// Get progress information about a background job running on this domain.
    /// NOTE: Only a subset of the fields in JobStats are populated by this method. If you want to
    /// populate more fields then you should use [`Self::job_stats`].
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetJobInfo>
    pub fn job_info(&self) -> Result<JobStats, Error> {
        let mut job_info = mem::MaybeUninit::uninit();
        let _ =
            check_neg!(unsafe { sys::virDomainGetJobInfo(self.as_ptr(), job_info.as_mut_ptr()) })?;

        let ptr: sys::virDomainJobInfoPtr = unsafe { &mut job_info.assume_init() };

        Ok(unsafe {
            JobStats {
                r#type: (*ptr).type_,
                time_elapsed: Some((*ptr).timeElapsed as u64),
                time_remaining: Some((*ptr).timeRemaining as u64),
                data_total: Some((*ptr).dataTotal as u64),
                data_processed: Some((*ptr).dataProcessed as u64),
                data_remaining: Some((*ptr).dataRemaining as u64),
                mem_total: Some((*ptr).memTotal as u64),
                mem_processed: Some((*ptr).memProcessed as u64),
                mem_remaining: Some((*ptr).memRemaining as u64),
                disk_total: Some((*ptr).fileTotal as u64),
                disk_processed: Some((*ptr).fileProcessed as u64),
                disk_remaining: Some((*ptr).fileRemaining as u64),
                ..Default::default()
            }
        })
    }

    /// Attach a device to the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainAttachDevice>
    pub fn attach_device(&self, xml: &str) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe { sys::virDomainAttachDevice(self.as_ptr(), xml_buf.as_ptr()) });
        Ok(())
    }

    /// Attach a device to the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainAttachDeviceFlags>
    pub fn attach_device_flags(&self, xml: &str, flags: u32) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe {
            sys::virDomainAttachDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(())
    }

    /// Detach a device from the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainDetachDevice>
    pub fn detach_device(&self, xml: &str) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe { sys::virDomainDetachDevice(self.as_ptr(), xml_buf.as_ptr()) })?;
        Ok(())
    }

    /// Detach a device from the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainDetachDeviceFlags>
    pub fn detach_device_flags(&self, xml: &str, flags: u32) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe {
            sys::virDomainDetachDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(())
    }

    /// Update a domain on the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainUpdateDeviceFlags>
    pub fn update_device_flags(&self, xml: &str, flags: u32) -> Result<(), Error> {
        let xml_buf = CString::new(xml)?;
        let _ = check_neg!(unsafe {
            sys::virDomainUpdateDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(())
    }

    /// Save the domain state to disk
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainManagedSave>
    pub fn managed_save(&self, flags: u32) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virDomainManagedSave(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Determine if the domain has saved state on disk
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainHasManagedSaveImage>
    pub fn has_managed_save(&self, flags: u32) -> Result<bool, Error> {
        let ret = check_neg!(unsafe {
            sys::virDomainHasManagedSaveImage(self.as_ptr(), flags as c_uint)
        })?;
        Ok(ret == 1)
    }

    /// Remove the domain saved state
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainManagedSaveRemove>
    pub fn managed_save_remove(&self, flags: u32) -> Result<(), Error> {
        let _ =
            check_neg!(unsafe { sys::virDomainManagedSaveRemove(self.as_ptr(), flags as c_uint) })?;
        Ok(())
    }

    /// Initiate a core dump of the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainCoreDump>
    pub fn core_dump(&self, to: &str, flags: u32) -> Result<(), Error> {
        let to_buf = CString::new(to)?;
        let _ = check_neg!(unsafe {
            sys::virDomainCoreDump(self.as_ptr(), to_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(())
    }

    /// Initiate a core dump of the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainCoreDumpWithFormat>
    pub fn core_dump_with_format(&self, to: &str, format: u32, flags: u32) -> Result<(), Error> {
        let to_buf = CString::new(to)?;
        let _ = check_neg!(unsafe {
            sys::virDomainCoreDumpWithFormat(
                self.as_ptr(),
                to_buf.as_ptr(),
                format as c_uint,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Update the domain metadata XML
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMetadata>
    pub fn set_metadata(
        &self,
        kind: i32,
        metadata: Option<&str>,
        key: Option<&str>,
        uri: Option<&str>,
        flags: u32,
    ) -> Result<(), Error> {
        let metadata_buf = some_string_to_cstring!(metadata);
        let key_buf = some_string_to_cstring!(key);
        let uri_buf = some_string_to_cstring!(uri);
        let _ = check_neg!(unsafe {
            sys::virDomainSetMetadata(
                self.as_ptr(),
                kind as c_int,
                some_cstring_to_c_chars!(metadata_buf),
                some_cstring_to_c_chars!(key_buf),
                some_cstring_to_c_chars!(uri_buf),
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returns the domain metadata XML
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetMetadata>
    pub fn metadata(&self, kind: i32, uri: Option<&str>, flags: u32) -> Result<String, Error> {
        let uri_buf = some_string_to_cstring!(uri);
        let n = check_null!(unsafe {
            sys::virDomainGetMetadata(
                self.as_ptr(),
                kind as c_int,
                some_cstring_to_c_chars!(uri_buf),
                flags as c_uint,
            )
        })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Resize a block device on a running domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainBlockResize>
    pub fn block_resize(&self, disk: &str, size: u64, flags: u32) -> Result<(), Error> {
        let disk_buf = CString::new(disk)?;
        let _ = check_neg!(unsafe {
            sys::virDomainBlockResize(
                self.as_ptr(),
                disk_buf.as_ptr(),
                size as c_ulonglong,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returns the domain memory parameters
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetMemoryParameters>
    pub fn memory_parameters(&self, flags: u32) -> Result<MemoryParameters, Error> {
        let mut nparams: c_int = 0;
        let _ = check_neg!(unsafe {
            sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as c_uint,
            )
        })?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let _ = check_neg!(unsafe {
            sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as c_uint,
            )
        })?;
        unsafe { params.set_len(nparams as usize) };
        Ok(MemoryParameters::from_vec(params))
    }

    /// Updates the dmoain memory parameters
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetMemoryParameters>
    pub fn set_memory_parameters(&self, params: MemoryParameters, flags: u32) -> Result<(), Error> {
        let mut cparams = params.to_vec();
        let _ = check_neg!(unsafe {
            sys::virDomainSetMemoryParameters(
                self.as_ptr(),
                cparams.as_mut_ptr(),
                cparams.len() as c_int,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrate>
    pub fn migrate(
        &self,
        dconn: &Connect,
        flags: u32,
        dname: Option<&str>,
        uri: Option<&str>,
        bandwidth: u64,
    ) -> Result<Domain, Error> {
        let dname_buf = some_string_to_cstring!(dname);
        let uri_buf = some_string_to_cstring!(uri);
        let ptr = check_null!(unsafe {
            sys::virDomainMigrate(
                self.as_ptr(),
                dconn.as_ptr(),
                flags as c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                some_cstring_to_c_chars!(uri_buf),
                bandwidth as c_ulong,
            )
        })?;
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrate2>
    pub fn migrate2(
        &self,
        dconn: &Connect,
        dxml: Option<&str>,
        flags: u32,
        dname: Option<&str>,
        uri: Option<&str>,
        bandwidth: u64,
    ) -> Result<Domain, Error> {
        let dxml_buf = some_string_to_cstring!(dxml);
        let dname_buf = some_string_to_cstring!(dname);
        let uri_buf = some_string_to_cstring!(uri);
        let ptr = check_null!(unsafe {
            sys::virDomainMigrate2(
                self.as_ptr(),
                dconn.as_ptr(),
                some_cstring_to_c_chars!(dxml_buf),
                flags as c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                some_cstring_to_c_chars!(uri_buf),
                bandwidth as c_ulong,
            )
        })?;
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrate3>
    pub fn migrate3(
        &self,
        dconn: &Connect,
        parameters: MigrateParameters,
        flags: u32,
    ) -> Result<Domain, Error> {
        let params = parameters.to_vec();
        let ptr = check_null!(unsafe {
            sys::virDomainMigrate3(
                self.as_ptr(),
                dconn.as_ptr(),
                params.clone().as_mut_ptr(),
                params.len() as c_uint,
                flags as c_uint,
            )
        })?;
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateToURI>
    pub fn migrate_to_uri(
        &self,
        duri: &str,
        flags: u32,
        dname: Option<&str>,
        bandwidth: u64,
    ) -> Result<(), Error> {
        let duri_buf = CString::new(duri)?;
        let dname_buf = some_string_to_cstring!(dname);
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateToURI(
                self.as_ptr(),
                duri_buf.as_ptr(),
                flags as c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                bandwidth as c_ulong,
            )
        })?;
        Ok(())
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateToURI2>
    pub fn migrate_to_uri2(
        &self,
        dconn_uri: Option<&str>,
        mig_uri: Option<&str>,
        dxml: Option<&str>,
        flags: u32,
        dname: Option<&str>,
        bandwidth: u64,
    ) -> Result<(), Error> {
        let dconn_uri_buf = some_string_to_cstring!(dconn_uri);
        let mig_uri_buf = some_string_to_cstring!(mig_uri);
        let dxml_buf = some_string_to_cstring!(dxml);
        let dname_buf = some_string_to_cstring!(dname);
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateToURI2(
                self.as_ptr(),
                some_cstring_to_c_chars!(dconn_uri_buf),
                some_cstring_to_c_chars!(mig_uri_buf),
                some_cstring_to_c_chars!(dxml_buf),
                flags as c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                bandwidth as c_ulong,
            )
        })?;
        Ok(())
    }

    /// Migrate the domain to another host
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainMigrateToURI3>
    pub fn migrate_to_uri3(
        &self,
        dconn_uri: Option<&str>,
        parameters: MigrateParameters,
        flags: u32,
    ) -> Result<(), Error> {
        let params = parameters.to_vec();
        let dconn_uri_buf = some_string_to_cstring!(dconn_uri);
        let _ = check_neg!(unsafe {
            sys::virDomainMigrateToURI3(
                self.as_ptr(),
                some_cstring_to_c_chars!(dconn_uri_buf),
                params.clone().as_mut_ptr(),
                params.len() as c_uint,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Returns the domain NUMA parameters
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetNumaParameters>
    pub fn numa_parameters(&self, flags: u32) -> Result<NUMAParameters, Error> {
        let mut nparams: c_int = 0;
        let _ = check_neg!(unsafe {
            sys::virDomainGetNumaParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as c_uint,
            )
        })?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let _ = check_neg!(unsafe {
            sys::virDomainGetNumaParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as c_uint,
            )
        })?;
        unsafe { params.set_len(nparams as usize) };
        let nparams = NUMAParameters::from_vec(params.clone());
        unsafe { typed_params_release_c_chars!(params) };

        Ok(nparams)
    }

    /// Updates the domain NUMA parameters
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetNumaParameters>
    pub fn set_numa_parameters(&self, params: NUMAParameters, flags: u32) -> Result<(), Error> {
        let mut cparams = params.to_vec();
        let _ = check_neg!(unsafe {
            sys::virDomainSetNumaParameters(
                self.as_ptr(),
                cparams.as_mut_ptr(),
                cparams.len() as c_int,
                flags as c_uint,
            )
        })?;
        unsafe { typed_params_release_c_chars!(cparams) };
        Ok(())
    }

    /// Returns a list of domain snapshot objects
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainListAllSnapshots>
    pub fn list_all_snapshots(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
        let size = check_neg!(unsafe {
            sys::virDomainListAllSnapshots(self.as_ptr(), &mut snaps, flags as c_uint)
        })?;

        let mut array: Vec<DomainSnapshot> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { DomainSnapshot::from_ptr(*snaps.offset(x)) });
        }
        unsafe { libc::free(snaps as *mut c_void) };

        Ok(array)
    }

    /// Get the cpu scheduler type for the domain
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetSchedulerType>
    pub fn scheduler_type(&self) -> Result<(String, i32), Error> {
        let mut nparams: c_int = -1;
        let sched_type =
            check_null!(unsafe { sys::virDomainGetSchedulerType(self.as_ptr(), &mut nparams) })?;
        Ok((unsafe { c_chars_to_string!(sched_type) }, nparams))
    }

    /// Get the scheduler parameters for the domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetSchedulerParameters>
    pub fn scheduler_parameters(&self) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.scheduler_type()?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let _ = check_neg!(unsafe {
            sys::virDomainGetSchedulerParameters(self.as_ptr(), params.as_mut_ptr(), &mut nparams)
        })?;
        unsafe { params.set_len(nparams as usize) };
        Ok(SchedulerInfo::from_vec(params, sched_type))
    }

    /// Get the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the domain modification [`Impact`]: [`VIR_DOMAIN_AFFECT_CURRENT`],
    ///   [`VIR_DOMAIN_AFFECT_LIVE`] or [`VIR_DOMAIN_AFFECT_CONFIG`].
    ///
    /// [`Impact`]: sys::virDomainModificationImpact
    /// [`VIR_DOMAIN_AFFECT_CURRENT`]: sys::VIR_DOMAIN_AFFECT_CURRENT
    /// [`VIR_DOMAIN_AFFECT_LIVE`]: sys::VIR_DOMAIN_AFFECT_LIVE
    /// [`VIR_DOMAIN_AFFECT_CONFIG`]: sys::VIR_DOMAIN_AFFECT_CONFIG
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainGetSchedulerParametersFlags>
    pub fn scheduler_parameters_flags(
        &self,
        flags: sys::virDomainModificationImpact,
    ) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.scheduler_type()?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let _ = check_neg!(unsafe {
            sys::virDomainGetSchedulerParametersFlags(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as c_uint,
            )
        })?;
        unsafe { params.set_len(nparams as usize) };
        Ok(SchedulerInfo::from_vec(params, sched_type))
    }

    /// Set the scheduler parameters for the domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetSchedulerParameters>
    pub fn set_scheduler_parameters(&self, sched_info: &SchedulerInfo) -> Result<(), Error> {
        let mut params = sched_info.to_vec();
        let _ = check_neg!(unsafe {
            sys::virDomainSetSchedulerParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                params.len() as c_int,
            )
        })?;
        Ok(())
    }

    /// Set the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the domain modification [`Impact`]: [`VIR_DOMAIN_AFFECT_CURRENT`],
    ///   [`VIR_DOMAIN_AFFECT_LIVE`] or [`VIR_DOMAIN_AFFECT_CONFIG`].
    ///
    /// [`Impact`]: sys::virDomainModificationImpact
    /// [`VIR_DOMAIN_AFFECT_CURRENT`]: sys::VIR_DOMAIN_AFFECT_CURRENT
    /// [`VIR_DOMAIN_AFFECT_LIVE`]: sys::VIR_DOMAIN_AFFECT_LIVE
    /// [`VIR_DOMAIN_AFFECT_CONFIG`]: sys::VIR_DOMAIN_AFFECT_CONFIG
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSetSchedulerParametersFlags>
    pub fn set_scheduler_parameters_flags(
        &self,
        sched_info: &SchedulerInfo,
        flags: sys::virDomainModificationImpact,
    ) -> Result<(), Error> {
        let mut params = sched_info.to_vec();
        let _ = check_neg!(unsafe {
            sys::virDomainSetSchedulerParametersFlags(
                self.as_ptr(),
                params.as_mut_ptr(),
                params.len() as c_int,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Send key(s) to the guest.
    /// # Arguments
    ///
    /// * `codeset` - Specifies the code set of keycodes.
    /// * `holdtime` - Specifies the duration (in milliseconds) that the keys will be held.
    /// * `keycodes` - Specifies the array of keycodes.
    /// * `nkeycodes` - Specifies the number of keycodes.
    /// * `flags` - Extra flags; not used yet, so callers should always pass 0..
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainSendKey>
    pub fn send_key(
        &self,
        codeset: sys::virKeycodeSet,
        holdtime: u32,
        keycodes: *mut u32,
        nkeycodes: i32,
        flags: u32,
    ) -> Result<(), Error> {
        let _ = check_neg!(unsafe {
            sys::virDomainSendKey(
                self.as_ptr(),
                codeset as c_uint,
                holdtime as c_uint,
                keycodes as *mut c_uint,
                nkeycodes as c_int,
                flags as c_uint,
            )
        })?;
        Ok(())
    }

    /// Take a screenshot of current domain console as a stream.
    /// Returns a string representing the mime-type of the image format.
    /// # Arguments
    ///
    /// * `domain` - a domain object
    /// * `stream` - stream to use as output
    /// * `screen` - monitor ID to take screenshot from
    /// * `flags` - extra flags; not used yet, so callers should always pass 0
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain.html#virDomainScreenshot>
    pub fn screenshot(&self, stream: &Stream, screen: u32, flags: u32) -> Result<String, Error> {
        let n = check_null!(unsafe {
            sys::virDomainScreenshot(
                self.as_ptr(),
                stream.as_ptr(),
                screen as c_uint,
                flags as c_uint,
            )
        })?;
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Send an arbitrary monitor command cmd to domain through the QEMU monitor.
    ///
    /// * `cmd` - the QEMU monitor command string
    /// * `flags` - bitwise-or of supported [`virDomainQemuMonitorCommandFlags`]
    ///
    /// [`virDomainQemuMonitorCommandFlags`]: sys::virDomainQemuMonitorCommandFlags
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-qemu.html#virDomainQemuMonitorCommand>
    #[cfg(feature = "qemu")]
    pub fn qemu_monitor_command(&self, cmd: &str, flags: u32) -> Result<String, Error> {
        let mut result: *mut c_char = std::ptr::null_mut();
        let cmd_buf = CString::new(cmd)?;
        let _ = check_neg!(unsafe {
            sys::virDomainQemuMonitorCommand(
                self.as_ptr(),
                cmd_buf.as_ptr(),
                &mut result,
                flags as c_uint,
            )
        })?;
        Ok(unsafe { c_chars_to_string!(result) })
    }

    /// Send an arbitrary agent command to the domain through the QEMU guest agent.
    ///
    /// * `cmd` - the QEMU guest agent command string
    /// * `flags` - bitwise-or of supported execution flags
    /// * `timeout` - the timeout in seconds, or one of the [`virDomainQemuAgentCommandTimeoutValues`] flags
    ///
    /// [`virDomainQemuAgentCommandTimeoutValues`]: sys::virDomainQemuAgentCommandTimeoutValues
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-qemu.html#virDomainQemuAgentCommand>
    #[cfg(feature = "qemu")]
    pub fn qemu_agent_command(&self, cmd: &str, timeout: i32, flags: u32) -> Result<String, Error> {
        let cmd_buf = CString::new(cmd)?;
        let ret = check_null!(unsafe {
            sys::virDomainQemuAgentCommand(
                self.as_ptr(),
                cmd_buf.as_ptr(),
                timeout as c_int,
                flags as c_uint,
            )
        })?;
        Ok(unsafe { c_chars_to_string!(ret) })
    }

    /// Get a handle to a named snapshot.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotLookupByName>
    pub fn lookup_snapshot_by_name(
        dom: &Domain,
        name: &str,
        flags: u32,
    ) -> Result<DomainSnapshot, Error> {
        let name_buf = CString::new(name)?;
        let ptr = check_null!(unsafe {
            sys::virDomainSnapshotLookupByName(dom.as_ptr(), name_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Create a new domain snapshot
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotCreateXML>
    pub fn create_snapshot_xml(&self, xml: &str, flags: u32) -> Result<DomainSnapshot, Error> {
        let xml_buf = CString::new(xml)?;
        let ptr = check_null!(unsafe {
            sys::virDomainSnapshotCreateXML(self.as_ptr(), xml_buf.as_ptr(), flags as c_uint)
        })?;
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Get a handle to the current snapshot
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotCurrent>
    pub fn current_snapshot(&self, flags: u32) -> Result<DomainSnapshot, Error> {
        let ptr =
            check_null!(unsafe { sys::virDomainSnapshotCurrent(self.as_ptr(), flags as c_uint) })?;
        Ok(unsafe { DomainSnapshot::from_ptr(ptr) })
    }

    /// Return the number of snapshots for this domain.
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-domain-snapshot.html#virDomainSnapshotNum>
    pub fn num_snapshots(&self, flags: u32) -> Result<u32, Error> {
        let ret = check_neg!(unsafe { sys::virDomainSnapshotNum(self.as_ptr(), flags as c_uint) })?;
        Ok(ret as u32)
    }
}
