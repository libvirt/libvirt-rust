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

use std::ffi::CStr;
use std::{mem, ptr, str};

use connect::sys::virConnectPtr;
use domain_snapshot::sys::virDomainSnapshotPtr;
use stream::sys::virStreamPtr;
use typedparam::sys::{virTypedParameter, virTypedParameterPtr};

use connect::Connect;
use domain_snapshot::DomainSnapshot;
use error::Error;
use stream::Stream;

pub mod sys {
    extern crate libc;

    use typedparam::sys::virTypedParameterPtr;

    #[repr(C)]
    pub struct virDomain {}

    pub type virDomainPtr = *mut virDomain;

    #[repr(C)]
    #[derive(Default)]
    pub struct virDomainInfo {
        pub state: libc::c_ulong,
        pub maxMem: libc::c_ulong,
        pub memory: libc::c_ulong,
        pub nrVirtCpu: libc::c_uint,
        pub cpuTime: libc::c_ulong,
    }

    pub type virDomainInfoPtr = *mut virDomainInfo;

    #[repr(C)]
    pub struct virDomainStatsRecord {
        pub dom: virDomainPtr,
        pub params: virTypedParameterPtr,
        pub nparams: libc::c_uint,
    }

    pub type virDomainStatsRecordPtr = *mut virDomainStatsRecord;

    #[repr(C)]
    #[derive(Default)]
    pub struct virDomainBlockInfo {
        pub capacity: libc::c_ulonglong,
        pub allocation: libc::c_ulonglong,
        pub physical: libc::c_ulonglong,
    }

    pub type virDomainBlockInfoPtr = *mut virDomainBlockInfo;

    #[repr(C)]
    #[derive(Default)]
    pub struct virDomainInterfaceStats {
        pub rx_bytes: libc::c_longlong,
        pub rx_packets: libc::c_longlong,
        pub rx_errs: libc::c_longlong,
        pub rx_drop: libc::c_longlong,
        pub tx_bytes: libc::c_longlong,
        pub tx_packets: libc::c_longlong,
        pub tx_errs: libc::c_longlong,
        pub tx_drop: libc::c_longlong,
    }

    pub type virDomainInterfaceStatsPtr = *mut virDomainInterfaceStats;

    #[repr(C)]
    #[derive(Default)]
    pub struct virDomainMemoryStats {
        pub tag: libc::c_int,
        pub val: libc::c_ulonglong,
    }

    pub type virDomainMemoryStatsPtr = *mut virDomainMemoryStats;
}

#[link(name = "virt")]
extern "C" {
    fn virDomainLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virDomainPtr;
    fn virDomainLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainLookupByUUIDString(
        c: virConnectPtr,
        uuid: *const libc::c_char,
    ) -> sys::virDomainPtr;
    fn virDomainCreate(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainCreateXML(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virDomainPtr;
    fn virDomainDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainDefineXMLFlags(
        c: virConnectPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> sys::virDomainPtr;
    fn virDomainCreateWithFlags(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainDestroy(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainDestroyFlags(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainUndefine(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainFree(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainShutdown(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainReboot(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSuspend(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainResume(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsActive(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsUpdated(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainGetName(ptr: sys::virDomainPtr) -> *const libc::c_char;
    fn virDomainReset(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainGetState(
        ptr: sys::virDomainPtr,
        state: *mut libc::c_int,
        reason: *mut libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetOSType(ptr: sys::virDomainPtr) -> *mut libc::c_char;
    fn virDomainGetHostname(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *mut libc::c_char;
    fn virDomainGetUUIDString(ptr: sys::virDomainPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virDomainGetXMLDesc(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *mut libc::c_char;
    fn virDomainGetAutostart(ptr: sys::virDomainPtr, autostart: *mut libc::c_int) -> libc::c_int;
    fn virDomainSetAutostart(ptr: sys::virDomainPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virDomainGetID(ptr: sys::virDomainPtr) -> libc::c_uint;
    fn virDomainSetMaxMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainGetMaxMemory(ptr: sys::virDomainPtr) -> libc::c_ulong;
    fn virDomainGetMaxVcpus(ptr: sys::virDomainPtr) -> libc::c_ulong;
    fn virDomainSetMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainSetMemoryFlags(
        ptr: sys::virDomainPtr,
        memory: libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetMemoryStatsPeriod(
        ptr: sys::virDomainPtr,
        period: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetVcpus(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainSetVcpusFlags(
        ptr: sys::virDomainPtr,
        vcpus: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetVcpusFlags(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainRestore(c: virConnectPtr, source: *const libc::c_char) -> libc::c_int;
    fn virDomainRestoreFlags(
        c: virConnectPtr,
        source: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetConnect(ptr: sys::virDomainPtr) -> virConnectPtr;
    fn virDomainGetInfo(ptr: sys::virDomainPtr, ninfo: sys::virDomainInfoPtr) -> libc::c_int;
    fn virDomainMigrateSetMaxSpeed(
        ptr: sys::virDomainPtr,
        bandwidth: libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMigrateGetMaxSpeed(
        ptr: sys::virDomainPtr,
        bandwidth: *mut libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMigrateSetCompressionCache(
        ptr: sys::virDomainPtr,
        size: libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMigrateGetCompressionCache(
        ptr: sys::virDomainPtr,
        size: *mut libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMigrateSetMaxDowntime(
        ptr: sys::virDomainPtr,
        downtime: libc::c_ulong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetTime(
        ptr: sys::virDomainPtr,
        seconds: *mut libc::c_long,
        nseconds: *mut libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetTime(
        ptr: sys::virDomainPtr,
        seconds: libc::c_long,
        nseconds: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetBlockInfo(
        ptr: sys::virDomainPtr,
        disk: *const libc::c_char,
        info: sys::virDomainBlockInfoPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainPinVcpu(
        ptr: sys::virDomainPtr,
        vcpu: libc::c_uint,
        vcpumap: *const libc::c_uchar,
        maplen: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainPinVcpuFlags(
        ptr: sys::virDomainPtr,
        vcpu: libc::c_uint,
        vcpumap: *const libc::c_uchar,
        maplen: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainPinEmulator(
        ptr: sys::virDomainPtr,
        vcpumap: *const libc::c_uchar,
        maplen: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainRename(
        ptr: sys::virDomainPtr,
        new_name: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetUserPassword(
        ptr: sys::virDomainPtr,
        user: *const libc::c_char,
        pass: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetBlockThreshold(
        ptr: sys::virDomainPtr,
        dev: *const libc::c_char,
        threshold: libc::c_ulonglong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainOpenGraphics(
        ptr: sys::virDomainPtr,
        idx: libc::c_uint,
        fd: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainOpenGraphicsFD(
        ptr: sys::virDomainPtr,
        idx: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainOpenChannel(
        ptr: sys::virDomainPtr,
        name: *const libc::c_char,
        st: virStreamPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainOpenConsole(
        ptr: sys::virDomainPtr,
        dev_name: *const libc::c_char,
        st: virStreamPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainInterfaceStats(
        ptr: sys::virDomainPtr,
        path: *const libc::c_char,
        stats: sys::virDomainInterfaceStatsPtr,
        size: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMemoryStats(
        ptr: sys::virDomainPtr,
        stats: sys::virDomainMemoryStatsPtr,
        nr_stats: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSaveImageGetXMLDesc(
        ptr: virConnectPtr,
        file: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virDomainSaveImageDefineXML(
        ptr: virConnectPtr,
        file: *const libc::c_char,
        dxml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainAttachDevice(ptr: sys::virDomainPtr, xml: *const libc::c_char) -> libc::c_int;
    fn virDomainAttachDeviceFlags(
        ptr: sys::virDomainPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainDetachDevice(ptr: sys::virDomainPtr, xml: *const libc::c_char) -> libc::c_int;
    fn virDomainDetachDeviceFlags(
        ptr: sys::virDomainPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainUpdateDeviceFlags(
        ptr: sys::virDomainPtr,
        xml: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainManagedSave(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainHasManagedSaveImage(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainManagedSaveRemove(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainCoreDump(
        ptr: sys::virDomainPtr,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainCoreDumpWithFormat(
        ptr: sys::virDomainPtr,
        to: *const libc::c_char,
        format: libc::c_uint,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetMetadata(
        ptr: sys::virDomainPtr,
        kind: libc::c_int,
        meta: *const libc::c_char,
        key: *const libc::c_char,
        uri: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetMetadata(
        ptr: sys::virDomainPtr,
        kind: libc::c_int,
        uri: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut libc::c_char;
    fn virDomainBlockResize(
        ptr: sys::virDomainPtr,
        disk: *const libc::c_char,
        size: libc::c_ulonglong,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetMemoryParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: *mut libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetMemoryParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainGetNumaParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: *mut libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetNumaParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainMigrate(
        ptr: sys::virDomainPtr,
        dconn: virConnectPtr,
        flags: libc::c_uint,
        dname: *const libc::c_char,
        uri: *const libc::c_char,
        bandwidth: libc::c_ulong,
    ) -> sys::virDomainPtr;
    fn virDomainMigrate2(
        ptr: sys::virDomainPtr,
        dconn: virConnectPtr,
        dxml: *const libc::c_char,
        flags: libc::c_uint,
        dname: *const libc::c_char,
        uri: *const libc::c_char,
        bandwidth: libc::c_ulong,
    ) -> sys::virDomainPtr;
    fn virDomainMigrateToURI(
        ptr: sys::virDomainPtr,
        duri: *const libc::c_char,
        flags: libc::c_uint,
        dname: *const libc::c_char,
        bandwidth: libc::c_ulong,
    ) -> sys::virDomainPtr;
    fn virDomainMigrateToURI2(
        ptr: sys::virDomainPtr,
        dconnuri: *const libc::c_char,
        miguri: *const libc::c_char,
        dxml: *const libc::c_char,
        flags: libc::c_uint,
        dname: *const libc::c_char,
        bandwidth: libc::c_ulong,
    ) -> sys::virDomainPtr;
    fn virDomainListAllSnapshots(
        ptr: sys::virDomainPtr,
        snaps: *mut *mut virDomainSnapshotPtr,
        flags: libc::c_uint,
    ) -> libc::c_int;

    fn virDomainGetSchedulerType(
        ptr: sys::virDomainPtr,
        nparams: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn virDomainGetSchedulerParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: *mut libc::c_int,
    ) -> libc::c_int;
    fn virDomainSetSchedulerParameters(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: libc::c_int,
    ) -> libc::c_int;
    fn virDomainGetSchedulerParametersFlags(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: *mut libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn virDomainSetSchedulerParametersFlags(
        ptr: sys::virDomainPtr,
        params: virTypedParameterPtr,
        nparams: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;

}

pub type DomainXMLFlags = self::libc::c_uint;
pub const VIR_DOMAIN_XML_SECURE: DomainXMLFlags = 1 << 0;
pub const VIR_DOMAIN_XML_INACTIVE: DomainXMLFlags = 1 << 1;
pub const VIR_DOMAIN_XML_UPDATE_CPU: DomainXMLFlags = 1 << 2;
pub const VIR_DOMAIN_XML_MIGRATABLE: DomainXMLFlags = 1 << 3;

pub type DomainCreateFlags = self::libc::c_uint;
pub const VIR_DOMAIN_NONE: DomainCreateFlags = 0;
pub const VIR_DOMAIN_START_PAUSED: DomainCreateFlags = 1 << 0;
pub const VIR_DOMAIN_START_AUTODESTROY: DomainCreateFlags = 1 << 1;
pub const VIR_DOMAIN_START_BYPASS_CACHE: DomainCreateFlags = 1 << 2;
pub const VIR_DOMAIN_START_FORCE_BOOT: DomainCreateFlags = 1 << 3;
pub const VIR_DOMAIN_START_VALIDATE: DomainCreateFlags = 1 << 4;

pub type DomainDestroyFlags = self::libc::c_uint;
pub const VIR_DOMAIN_DESTROY_DEFAULT: DomainDestroyFlags = 0;
pub const VIR_DOMAIN_DESTROY_GRACEFUL: DomainDestroyFlags = 1 << 0;

pub type DomainModImpactFlags = self::libc::c_uint;
pub const VIR_DOMAIN_AFFECT_CURRENT: DomainModImpactFlags = 0;
pub const VIR_DOMAIN_AFFECT_LIVE: DomainModImpactFlags = 1 << 0;
pub const VIR_DOMAIN_AFFECT_CONFIG: DomainModImpactFlags = 1 << 1;

pub type DomainMemoryModFlags = self::libc::c_uint;
pub const VIR_DOMAIN_MEM_CURRENT: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_MEM_LIVE: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_MEM_CONFIG: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_MEM_MAXIMUM: DomainMemoryModFlags = 1 << 2;

pub type DomainVcpuFlags = self::libc::c_uint;
pub const VIR_DOMAIN_VCPU_CURRENT: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_VCPU_LIVE: DomainVcpuFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_VCPU_CONFIG: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_VCPU_MAXIMUM: DomainVcpuFlags = 1 << 2;
pub const VIR_DOMAIN_VCPU_GUEST: DomainVcpuFlags = 1 << 3;
pub const VIR_DOMAIN_VCPU_HOTPLUGGABLE: DomainVcpuFlags = 1 << 4;

pub type DomainMigrateFlags = self::libc::c_uint;
pub const VIR_MIGRATE_LIVE: DomainMigrateFlags = 1 << 0;
pub const VIR_MIGRATE_PEER2PEER: DomainMigrateFlags = 1 << 1;
pub const VIR_MIGRATE_TUNNELLED: DomainMigrateFlags = 1 << 2;
pub const VIR_MIGRATE_PERSIST_DEST: DomainMigrateFlags = 1 << 3;
pub const VIR_MIGRATE_UNDEFINE_SOURCE: DomainMigrateFlags = 1 << 4;
pub const VIR_MIGRATE_PAUSED: DomainMigrateFlags = 1 << 5;
pub const VIR_MIGRATE_NON_SHARED_DISK: DomainMigrateFlags = 1 << 6;
pub const VIR_MIGRATE_NON_SHARED_INC: DomainMigrateFlags = 1 << 7;
pub const VIR_MIGRATE_CHANGE_PROTECTION: DomainMigrateFlags = 1 << 8;
pub const VIR_MIGRATE_UNSAFE: DomainMigrateFlags = 1 << 9;
pub const VIR_MIGRATE_OFFLINE: DomainMigrateFlags = 1 << 10;
pub const VIR_MIGRATE_COMPRESSED: DomainMigrateFlags = 1 << 11;
pub const VIR_MIGRATE_ABORT_ON_ERROR: DomainMigrateFlags = 1 << 12;
pub const VIR_MIGRATE_AUTO_CONVERGE: DomainMigrateFlags = 1 << 13;
pub const VIR_MIGRATE_RDMA_PIN_ALL: DomainMigrateFlags = 1 << 14;
pub const VIR_MIGRATE_POSTCOPY: DomainMigrateFlags = 1 << 15;
pub const VIR_MIGRATE_TLS: DomainMigrateFlags = 1 << 16;

pub type DomainDefineFlags = self::libc::c_uint;
pub const VIR_DOMAIN_DEFINE_VALIDATE: DomainDefineFlags = 1 << 0;

pub type DomainSaveRestoreFlags = self::libc::c_uint;
pub const VIR_DOMAIN_SAVE_BYPASS_CACHE: DomainSaveRestoreFlags = 1 << 0;
pub const VIR_DOMAIN_SAVE_RUNNING: DomainSaveRestoreFlags = 1 << 1;
pub const VIR_DOMAIN_SAVE_PAUSED: DomainSaveRestoreFlags = 1 << 2;

pub type DomainNumatuneMemMode = self::libc::c_int;
pub const VIR_DOMAIN_NUMATUNE_MEM_STRICT: DomainNumatuneMemMode = 0;
pub const VIR_DOMAIN_NUMATUNE_MEM_PREFERRED: DomainNumatuneMemMode = 1;
pub const VIR_DOMAIN_NUMATUNE_MEM_INTERLEAVE: DomainNumatuneMemMode = 2;

pub type DomainState = self::libc::c_uint;
pub const VIR_DOMAIN_NOSTATE: DomainState = 0;
pub const VIR_DOMAIN_RUNNING: DomainState = 1;
pub const VIR_DOMAIN_BLOCKED: DomainState = 2;
pub const VIR_DOMAIN_PAUSED: DomainState = 3;
pub const VIR_DOMAIN_SHUTDOWN: DomainState = 4;
pub const VIR_DOMAIN_SHUTOFF: DomainState = 5;
pub const VIR_DOMAIN_CRASHED: DomainState = 6;
pub const VIR_DOMAIN_PMSUSPENDED: DomainState = 7;

#[derive(Clone, Debug)]
pub struct DomainInfo {
    /// The running state, one of virDomainState.
    pub state: DomainState,
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
    pub fn from_ptr(ptr: sys::virDomainInfoPtr) -> DomainInfo {
        unsafe {
            DomainInfo {
                state: (*ptr).state as DomainState,
                max_mem: (*ptr).maxMem as u64,
                memory: (*ptr).memory as u64,
                nr_virt_cpu: (*ptr).nrVirtCpu as u32,
                cpu_time: (*ptr).cpuTime as u64,
            }
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
    pub fn from_ptr(ptr: sys::virDomainBlockInfoPtr) -> BlockInfo {
        unsafe {
            BlockInfo {
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).capacity as u64,
                physical: (*ptr).capacity as u64,
            }
        }
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

impl MemoryParameters {
    pub fn from_vec(vec: Vec<virTypedParameter>) -> MemoryParameters {
        unsafe {
            let mut ret = MemoryParameters::default();
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "hard_limit" => ret.hard_limit = Some(param.value as u64),
                    "soft_limit" => ret.soft_limit = Some(param.value as u64),
                    "min_guarantee" => ret.min_guarantee = Some(param.value as u64),
                    "swap_hard_limit" => ret.swap_hard_limit = Some(param.value as u64),
                    unknow => panic!("Field not implemented for MemoryParameters, {:?}", unknow),
                }
            }
            ret
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NUMAParameters {
    /// Lists the numa nodeset of a domain.
    pub node_set: Option<String>,
    /// Numa mode of a domain, as an int containing a
    /// DomainNumatuneMemMode value.
    pub mode: Option<DomainNumatuneMemMode>,
}

impl NUMAParameters {
    pub fn from_vec(vec: Vec<virTypedParameter>) -> NUMAParameters {
        unsafe {
            let mut ret = NUMAParameters::default();
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "numa_nodeset" => {
                        ret.node_set = Some(c_chars_to_string!(param.value as *mut libc::c_char))
                    }
                    "numa_mode" => ret.mode = Some(param.value as libc::c_int),
                    unknow => panic!("Field not implemented for NUMAParameters, {:?}", unknow),
                }
            }
            ret
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
    pub fn from_ptr(ptr: sys::virDomainInterfaceStatsPtr) -> InterfaceStats {
        unsafe {
            InterfaceStats {
                rx_bytes: (*ptr).rx_bytes as i64,
                rx_packets: (*ptr).rx_packets as i64,
                rx_errs: (*ptr).rx_errs as i64,
                rx_drop: (*ptr).rx_drop as i64,
                tx_bytes: (*ptr).tx_bytes as i64,
                tx_packets: (*ptr).tx_packets as i64,
                tx_errs: (*ptr).tx_errs as i64,
                tx_drop: (*ptr).tx_drop as i64,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemoryStats {
    pub tag: i32,
    pub val: u64,
}

impl MemoryStats {
    pub fn from_ptr(ptr: sys::virDomainMemoryStatsPtr) -> MemoryStats {
        unsafe {
            MemoryStats {
                tag: (*ptr).tag as i32,
                val: (*ptr).val as u64,
            }
        }
    }
}

/// Structure representing the CFS scheduler cpu bandwidth parameters
/// see https://www.kernel.org/doc/html/latest/scheduler/sched-bwc.html
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
}

impl SchedulerInfo {
    pub fn from_vec(vec: Vec<virTypedParameter>, scheduler_type: String) -> SchedulerInfo {
        unsafe {
            let mut ret = SchedulerInfo::default();
            ret.scheduler_type = scheduler_type;
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "cpu_shares" => ret.cpu_shares = Some(param.value as u64),
                    "vcpu_period" => ret.vcpu_bw.period = Some(param.value as u64),
                    "vcpu_quota" => ret.vcpu_bw.quota = Some(param.value as i64),
                    "emulator_period" => ret.emulator_bw.period = Some(param.value as u64),
                    "emulator_quota" => ret.emulator_bw.quota = Some(param.value as i64),
                    "global_period" => ret.global_bw.period = Some(param.value as u64),
                    "global_quota" => ret.global_bw.quota = Some(param.value as i64),
                    "iothread_period" => ret.iothread_bw.period = Some(param.value as u64),
                    "iothread_quota" => ret.iothread_bw.quota = Some(param.value as i64),
                    unknow => panic!("Field not implemented for SchedulerInfo, {:?}", unknow),
                }
            }
            ret
        }
    }

    pub fn to_vec(&self) -> Vec<virTypedParameter> {
        let mut cparams: Vec<virTypedParameter> = Vec::new();

        if let Some(shares) = self.cpu_shares {
            cparams.push(virTypedParameter {
                field: to_arr("cpu_shares\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                value: shares,
            });
        }

        if let Some(period) = self.vcpu_bw.period {
            cparams.push(virTypedParameter {
                field: to_arr("vcpu_period\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                value: period,
            });
        }
        if let Some(quota) = self.vcpu_bw.quota {
            cparams.push(virTypedParameter {
                field: to_arr("vcpu_quota\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_LLONG,
                value: quota as u64,
            });
        }
        if let Some(period) = self.emulator_bw.period {
            cparams.push(virTypedParameter {
                field: to_arr("emulator_period\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                value: period,
            });
        }
        if let Some(quota) = self.emulator_bw.quota {
            cparams.push(virTypedParameter {
                field: to_arr("emulator_quota\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_LLONG,
                value: quota as u64,
            });
        }
        if let Some(period) = self.global_bw.period {
            cparams.push(virTypedParameter {
                field: to_arr("global_period\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                value: period,
            });
        }
        if let Some(quota) = self.global_bw.quota {
            cparams.push(virTypedParameter {
                field: to_arr("global_quota\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_LLONG,
                value: quota as u64,
            });
        }
        if let Some(period) = self.iothread_bw.period {
            cparams.push(virTypedParameter {
                field: to_arr("iothread_period\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                value: period,
            });
        }
        if let Some(quota) = self.iothread_bw.quota {
            cparams.push(virTypedParameter {
                field: to_arr("iothread_quota\0"),
                typed: ::typedparam::VIR_TYPED_PARAM_LLONG,
                value: quota as u64,
            });
        }

        cparams
    }
}

/// Provides APIs for the management of domains.
///
/// See http://libvirt.org/html/libvirt-libvirt-domain.html
#[derive(Debug)]
pub struct Domain {
    ptr: Option<sys::virDomainPtr>,
}

impl Drop for Domain {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!(
                    "Unable to drop memory for Domain, code {}, message: {}",
                    e.code, e.message
                )
            }
        }
    }
}

fn to_arr(name: &str) -> [libc::c_char; 80] {
    let mut field: [libc::c_char; 80] = [0; 80];
    for (a, c) in field.iter_mut().zip(name.as_bytes()) {
        *a = *c as i8
    }
    field
}

impl Domain {
    pub fn new(ptr: sys::virDomainPtr) -> Domain {
        return Domain { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virDomainPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virDomainGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Extracts domain state.
    ///
    /// Each state can be accompanied with a reason (if known) which
    /// led to the state.
    pub fn get_state(&self) -> Result<(DomainState, i32), Error> {
        unsafe {
            let mut state: libc::c_int = -1;
            let mut reason: libc::c_int = -1;
            let ret = virDomainGetState(self.as_ptr(), &mut state, &mut reason, 0);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok((state as DomainState, reason as i32));
        }
    }

    /// Get the public name of the domain.
    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    /// Get the type of domain operating system.
    pub fn get_os_type(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetOSType(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    /// Get the hostname for that domain.
    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetHostname(self.as_ptr(), flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    /// Get the UUID for a domain as string.
    ///
    /// For more information about UUID see RFC4122.
    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virDomainGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    /// Get the hypervisor ID number for the domain
    pub fn get_id(&self) -> Option<u32> {
        unsafe {
            let ret = virDomainGetID(self.as_ptr());
            if ret as i32 == -1 {
                return None;
            }
            Some(ret)
        }
    }

    /// Provide an XML description of the domain. The description may
    /// be reused later to relaunch the domain with `create_xml()`.
    pub fn get_xml_desc(&self, flags: DomainCreateFlags) -> Result<String, Error> {
        unsafe {
            let xml = virDomainGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools. The domain will
    /// be paused only if restoring from managed state created from a
    /// paused domain.  For more control, see `create_with_flags()`.
    pub fn create(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools.
    pub fn create_with_flags(&self, flags: DomainCreateFlags) -> Result<u32, Error> {
        unsafe {
            let res = virDomainCreateWithFlags(self.as_ptr(), flags as libc::c_uint);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(res as u32);
        }
    }

    /// Extract information about a domain. Note that if the
    /// connection used to get the domain is limited only a partial
    /// set of the information can be extracted.
    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInfo::default();
            let res = virDomainGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(DomainInfo::from_ptr(pinfo));
        }
    }

    /// Launch a new guest domain, based on an XML description similar
    /// to the one returned by `get_xml_desc()`.
    ///
    /// This function may require privileged access to the hypervisor.
    ///
    /// The domain is not persistent, so its definition will disappear
    /// when it is destroyed, or if the host is restarted (see
    /// `define_xml()` to define persistent domains).
    pub fn create_xml(
        conn: &Connect,
        xml: &str,
        flags: DomainCreateFlags,
    ) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreateXML(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// `undefine()`. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXML(conn.as_ptr(), string_to_c_chars!(xml));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// `undefine()`. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    pub fn define_xml_flags(
        conn: &Connect,
        xml: &str,
        flags: DomainDefineFlags,
    ) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXMLFlags(
                conn.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virDomainDestroy(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    /// Reset a domain immediately without any guest OS shutdown.
    /// Reset emulates the power reset button on a machine, where all
    /// hardware sees the RST line set and reinitializes internal
    /// state.
    ///
    /// Note that there is a risk of data loss caused by reset without
    /// any guest OS shutdown.
    pub fn reset(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainReset(self.as_ptr(), 0);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy_flags(&self, flags: DomainDestroyFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDestroyFlags(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Shutdown a domain
    ///
    /// The domain object is still usable thereafter, but the domain
    /// OS is being stopped. Note that the guest OS may ignore the
    /// request. Additionally, the hypervisor may check and support
    /// the domain 'on_poweroff' XML setting resulting in a domain
    /// that reboots instead of shutting down. For guests that react
    /// to a shutdown request, the differences from `destroy()` are
    /// that the guests disk storage will be in a stable state rather
    /// than having the (virtual) power cord pulled, and this command
    /// returns as soon as the shutdown request is issued rather than
    /// blocking until the guest is no longer running.
    pub fn shutdown(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainShutdown(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Reboot a domain.
    ///
    /// The domain object is still usable thereafter.
    pub fn reboot(&self) -> Result<(), Error> {
        unsafe {
            if virDomainReboot(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    /// Suspend a domain.
    ///
    /// Suspends an active domain, the process is frozen without
    /// further access to CPU resources and I/O but the memory used by
    /// the domain at the hypervisor level will stay allocated. Use
    /// `resume` to reactivate the domain.  This function may
    /// require privileged access.  Moreover, suspend may not be
    /// supported if domain is in some special state like
    /// VIR_DOMAIN_PMSUSPENDED.
    pub fn suspend(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSuspend(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Resume a suspended domain.
    ///
    /// the process is restarted from the state where it was frozen by
    /// calling `suspend()`. This function may require privileged
    /// access Moreover, resume may not be supported if domain is in
    /// some special state like VIR_DOMAIN_PMSUSPENDED.
    pub fn resume(&self) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainResume(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    /// Determine if the domain is currently running.
    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virDomainUndefine(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    /// Free the domain object.
    ///
    /// The running instance is kept alive. The data structure is
    /// freed and should not be used thereafter.
    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virDomainFree(self.as_ptr()) == -1 {
                return Err(Error::new());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsUpdated(self.as_ptr());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut autostart: libc::c_int = 0;
            let ret = virDomainGetAutostart(self.as_ptr(), &mut autostart);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(autostart == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetAutostart(self.as_ptr(), autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMaxMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_max_memory(&self) -> Result<u64, Error> {
        unsafe {
            let ret = virDomainGetMaxMemory(self.as_ptr());
            if ret == 0 {
                return Err(Error::new());
            }
            return Ok(ret as u64);
        }
    }

    pub fn get_max_vcpus(&self) -> Result<u64, Error> {
        unsafe {
            let ret = virDomainGetMaxVcpus(self.as_ptr());
            if ret == 0 {
                return Err(Error::new());
            }
            return Ok(ret as u64);
        }
    }

    pub fn set_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_flags(
        &self,
        memory: u64,
        flags: DomainMemoryModFlags,
    ) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemoryFlags(
                self.as_ptr(),
                memory as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_stats_period(
        &self,
        period: i32,
        flags: DomainMemoryModFlags,
    ) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemoryStatsPeriod(
                self.as_ptr(),
                period as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetVcpus(self.as_ptr(), vcpus as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus_flags(&self, vcpus: u32, flags: DomainVcpuFlags) -> Result<bool, Error> {
        unsafe {
            let ret =
                virDomainSetVcpusFlags(self.as_ptr(), vcpus as libc::c_uint, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        unsafe {
            if virDomainRestore(conn.as_ptr(), string_to_c_chars!(path)) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn domain_restore_flags(
        conn: &Connect,
        path: &str,
        flags: DomainSaveRestoreFlags,
    ) -> Result<(), Error> {
        unsafe {
            if virDomainRestoreFlags(
                conn.as_ptr(),
                string_to_c_chars!(path),
                flags as libc::c_uint,
            ) == -1
            {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn get_vcpus_flags(&self, flags: DomainVcpuFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainGetVcpusFlags(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_set_max_speed(&self, bandwidth: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetMaxSpeed(
                self.as_ptr(),
                bandwidth as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_get_max_speed(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut bandwidth: libc::c_ulong = 0;
            let ret =
                virDomainMigrateGetMaxSpeed(self.as_ptr(), &mut bandwidth, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(bandwidth as u64);
        }
    }

    pub fn migrate_set_compression_cache(&self, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetCompressionCache(
                self.as_ptr(),
                size as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_get_compression_cache(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut size: libc::c_ulong = 0;
            let ret = virDomainMigrateGetCompressionCache(
                self.as_ptr(),
                &mut size,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(size as u64);
        }
    }

    pub fn migrate_set_max_downtime(&self, downtime: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetMaxDowntime(
                self.as_ptr(),
                downtime as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetTime(
                self.as_ptr(),
                seconds as libc::c_long,
                nseconds as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_time(&self, flags: u32) -> Result<(i64, i32), Error> {
        unsafe {
            let mut seconds: libc::c_long = 0;
            let mut nseconds: libc::c_int = 0;
            let ret = virDomainGetTime(
                self.as_ptr(),
                &mut seconds,
                &mut nseconds,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok((seconds as i64, nseconds as i32));
        }
    }

    pub fn get_block_info(&self, disk: &str, flags: u32) -> Result<BlockInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainBlockInfo::default();
            let ret = virDomainGetBlockInfo(
                self.as_ptr(),
                string_to_c_chars!(disk),
                pinfo,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(BlockInfo::from_ptr(pinfo));
        }
    }

    pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinVcpu(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr(),
                cpumap.len() as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinVcpuFlags(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr(),
                cpumap.len() as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinEmulator(
                self.as_ptr(),
                cpumap.as_ptr(),
                cpumap.len() as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn rename(&self, new_name: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainRename(
                self.as_ptr(),
                string_to_c_chars!(new_name),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_user_password(&self, user: &str, password: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetUserPassword(
                self.as_ptr(),
                string_to_c_chars!(user),
                string_to_c_chars!(password),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_block_threshold(&self, dev: &str, threshold: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetBlockThreshold(
                self.as_ptr(),
                string_to_c_chars!(dev),
                threshold as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenGraphics(
                self.as_ptr(),
                idx as libc::c_uint,
                fd as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret =
                virDomainOpenGraphicsFD(self.as_ptr(), idx as libc::c_uint, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_channel(&self, name: &str, stream: Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenChannel(
                self.as_ptr(),
                string_to_c_chars!(name),
                stream.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_console(&self, name: &str, stream: Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenConsole(
                self.as_ptr(),
                string_to_c_chars!(name),
                stream.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInterfaceStats::default();
            let ret = virDomainInterfaceStats(
                self.as_ptr(),
                string_to_c_chars!(path),
                pinfo,
                mem::size_of::<sys::virDomainInterfaceStats>() as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(InterfaceStats::from_ptr(pinfo));
        }
    }

    pub fn memory_stats(&self, nr_stats: u32, flags: u32) -> Result<MemoryStats, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainMemoryStats::default();
            let ret = virDomainMemoryStats(
                self.as_ptr(),
                pinfo,
                nr_stats as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(MemoryStats::from_ptr(pinfo));
        }
    }

    pub fn save_image_get_xml_desc(
        conn: &Connect,
        file: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let ptr = virDomainSaveImageGetXMLDesc(
                conn.as_ptr(),
                string_to_c_chars!(file),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(ptr));
        }
    }

    pub fn save_image_define_xml(
        conn: &Connect,
        file: &str,
        dxml: &str,
        flags: u32,
    ) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSaveImageDefineXML(
                conn.as_ptr(),
                string_to_c_chars!(file),
                string_to_c_chars!(dxml),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn attach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainAttachDevice(self.as_ptr(), string_to_c_chars!(xml));
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn attach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainAttachDeviceFlags(
                self.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn detach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDetachDevice(self.as_ptr(), string_to_c_chars!(xml));
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn detach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDetachDeviceFlags(
                self.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn update_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainUpdateDeviceFlags(
                self.as_ptr(),
                string_to_c_chars!(xml),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn managed_save(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainManagedSave(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn has_managed_save(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainHasManagedSaveImage(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn managed_save_remove(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainManagedSaveRemove(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn core_dump(&self, to: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret =
                virDomainCoreDump(self.as_ptr(), string_to_c_chars!(to), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn core_dump_with_format(&self, to: &str, format: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainCoreDumpWithFormat(
                self.as_ptr(),
                string_to_c_chars!(to),
                format as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_metadata(
        &self,
        kind: i32,
        metadata: &str,
        key: &str,
        uri: &str,
        flags: u32,
    ) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                string_to_c_chars!(metadata),
                string_to_c_chars!(key),
                string_to_c_chars!(uri),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_metadata(&self, kind: i32, uri: &str, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                string_to_c_chars!(uri),
                flags as libc::c_uint,
            );
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn block_resize(&self, disk: &str, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainBlockResize(
                self.as_ptr(),
                string_to_c_chars!(disk),
                size as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_memory_parameters(&self, flags: u32) -> Result<MemoryParameters, Error> {
        unsafe {
            let mut nparams: libc::c_int = 0;
            let ret = virDomainGetMemoryParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            let mut params: Vec<virTypedParameter> = vec![virTypedParameter::default(); 3];
            let ret = virDomainGetMemoryParameters(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(MemoryParameters::from_vec(params))
        }
    }

    pub fn set_memory_parameters(
        &self,
        params: MemoryParameters,
        flags: u32,
    ) -> Result<u32, Error> {
        unsafe {
            let mut cparams: Vec<virTypedParameter> = Vec::new();
            if params.hard_limit.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("hard_limit\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                    value: params.hard_limit.unwrap(),
                })
            }
            if params.soft_limit.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("soft_limit\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                    value: params.soft_limit.unwrap(),
                })
            }
            if params.min_guarantee.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("min_guarantee\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                    value: params.min_guarantee.unwrap(),
                })
            }
            if params.swap_hard_limit.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("swap_hard_limit\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                    value: params.swap_hard_limit.unwrap(),
                })
            }

            let ret = virDomainSetMemoryParameters(
                self.as_ptr(),
                &mut cparams[0],
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret as u32)
        }
    }

    pub fn migrate(
        &self,
        dconn: &Connect,
        flags: u32,
        uri: &str,
        bandwidth: u64,
    ) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrate(
                self.as_ptr(),
                dconn.as_ptr(),
                flags as libc::c_uint,
                string_to_c_chars!(""),
                string_to_c_chars!(uri),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate2(
        &self,
        dconn: &Connect,
        dxml: &str,
        flags: u32,
        uri: &str,
        bandwidth: u64,
    ) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrate2(
                self.as_ptr(),
                dconn.as_ptr(),
                string_to_c_chars!(dxml),
                flags as libc::c_uint,
                string_to_c_chars!(""),
                string_to_c_chars!(uri),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate_to_uri(&self, duri: &str, flags: u32, bandwidth: u64) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrateToURI(
                self.as_ptr(),
                string_to_c_chars!(duri),
                flags as libc::c_uint,
                string_to_c_chars!(""),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate_to_uri2(
        &self,
        dconn_uri: &str,
        mig_uri: &str,
        dxml: &str,
        flags: u32,
        bandwidth: u64,
    ) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrateToURI2(
                self.as_ptr(),
                string_to_c_chars!(dconn_uri),
                string_to_c_chars!(mig_uri),
                string_to_c_chars!(dxml),
                flags as libc::c_uint,
                string_to_c_chars!(""),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_numa_parameters(&self, flags: u32) -> Result<NUMAParameters, Error> {
        unsafe {
            let mut nparams: libc::c_int = 0;
            let ret = virDomainGetNumaParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            let mut params: Vec<virTypedParameter> =
                vec![virTypedParameter::default(); nparams as usize];
            let ret = virDomainGetNumaParameters(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(NUMAParameters::from_vec(params))
        }
    }

    pub fn set_numa_parameters(&self, params: NUMAParameters, flags: u32) -> Result<u32, Error> {
        unsafe {
            let mut cparams: Vec<virTypedParameter> = Vec::new();
            if params.node_set.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("numa_nodeset\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_STRING,
                    value: string_to_mut_c_chars!(params.node_set.unwrap()) as u64,
                })
            }
            if params.mode.is_some() {
                cparams.push(virTypedParameter {
                    field: to_arr("numa_mode\0"),
                    typed: ::typedparam::VIR_TYPED_PARAM_INT,
                    value: params.mode.unwrap() as u64,
                })
            }

            let ret = virDomainSetNumaParameters(
                self.as_ptr(),
                &mut cparams[0],
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret as u32)
        }
    }

    pub fn list_all_snapshots(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        unsafe {
            let mut snaps: *mut virDomainSnapshotPtr = ptr::null_mut();
            let size = virDomainListAllSnapshots(self.as_ptr(), &mut snaps, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
            }

            mem::forget(snaps);

            let mut array: Vec<DomainSnapshot> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainSnapshot::new(*snaps.offset(x)));
            }
            libc::free(snaps as *mut libc::c_void);

            return Ok(array);
        }
    }

    /// Get the cpu scheduler type for the domain
    pub fn get_scheduler_type(&self) -> Result<(String, i32), Error> {
        unsafe {
            let mut nparams: libc::c_int = -1;
            let sched_type = virDomainGetSchedulerType(self.as_ptr(), &mut nparams);
            if sched_type.is_null() {
                return Err(Error::new());
            }

            return Ok((c_chars_to_string!(sched_type), nparams));
        }
    }

    /// Get the scheduler parameters for the domain.
    pub fn get_scheduler_parameters(&self) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        unsafe {
            let mut params: Vec<virTypedParameter> = vec![virTypedParameter::default(); 9];
            let ret = virDomainGetSchedulerParameters(self.as_ptr(), &mut params[0], &mut nparams);
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(SchedulerInfo::from_vec(params, sched_type))
        }
    }

    /// Get the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the Domain Impact: CONFIG, LIVE or CURRENT.
    pub fn get_scheduler_parameters_flags(
        &self,
        flags: DomainModImpactFlags,
    ) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        unsafe {
            let mut params: Vec<virTypedParameter> = vec![virTypedParameter::default(); 9];
            let ret = virDomainGetSchedulerParametersFlags(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(SchedulerInfo::from_vec(params, sched_type))
        }
    }

    /// Set the scheduler parameters for the domain.
    pub fn set_scheduler_parameters(&self, sched_info: &SchedulerInfo) -> Result<i32, Error> {
        unsafe {
            let mut params = sched_info.to_vec();
            let ret = virDomainSetSchedulerParameters(
                self.as_ptr(),
                &mut params[0],
                params.len() as libc::c_int,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret)
        }
    }

    /// Set the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the Domain Impact: CONFIG, LIVE or CURRENT.
    pub fn set_scheduler_parameters_flags(
        &self,
        sched_info: &SchedulerInfo,
        flags: DomainModImpactFlags,
    ) -> Result<i32, Error> {
        unsafe {
            let mut params = sched_info.to_vec();
            let ret = virDomainSetSchedulerParametersFlags(
                self.as_ptr(),
                &mut params[0],
                params.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret)
        }
    }
}
