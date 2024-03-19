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
use std::{mem, ptr, str};

use uuid::Uuid;

use crate::connect::Connect;
use crate::domain_snapshot::DomainSnapshot;
use crate::error::Error;
use crate::stream::Stream;
use crate::typedparams::{from_params, to_params};
use crate::util::c_ulong_to_u64;
use crate::{param_field_in, param_field_out};

#[derive(Clone, Debug)]
pub struct DomainInfo {
    /// The running state, one of virDomainState.
    pub state: sys::virDomainState,
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
            state: (*ptr).state as sys::virDomainState,
            max_mem: c_ulong_to_u64((*ptr).maxMem),
            memory: c_ulong_to_u64((*ptr).memory),
            nr_virt_cpu: (*ptr).nrVirtCpu as u32,
            cpu_time: (*ptr).cpuTime,
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
    ptr: Option<sys::virDomainPtr>,
}

unsafe impl Send for Domain {}
unsafe impl Sync for Domain {}

impl Drop for Domain {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Domain: {}", e)
            }
        }
    }
}

impl Clone for Domain {
    /// Creates a copy of a domain.
    ///
    /// Increments the internal reference counter on the given
    /// domain. For each call to this method, there shall be a
    /// corresponding call to [`free()`].
    ///
    /// [`free()`]: Domain::free
    fn clone(&self) -> Self {
        self.add_ref().unwrap()
    }
}

impl Domain {
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid.
    pub unsafe fn from_ptr(ptr: sys::virDomainPtr) -> Domain {
        Domain { ptr: Some(ptr) }
    }

    fn add_ref(&self) -> Result<Domain, Error> {
        unsafe {
            if sys::virDomainRef(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
        }

        Ok(unsafe { Domain::from_ptr(self.as_ptr()) })
    }

    pub fn as_ptr(&self) -> sys::virDomainPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        let ptr = unsafe { sys::virDomainGetConnect(self.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Connect::from_ptr(ptr) })
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        let ptr = unsafe { sys::virDomainLookupByID(conn.as_ptr(), id as libc::c_int) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        let id_buf = CString::new(id).unwrap();
        let ptr = unsafe { sys::virDomainLookupByName(conn.as_ptr(), id_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid(conn: &Connect, uuid: Uuid) -> Result<Domain, Error> {
        let ptr = unsafe { sys::virDomainLookupByUUID(conn.as_ptr(), uuid.as_bytes().as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        let uuid_buf = CString::new(uuid).unwrap();
        let ptr = unsafe { sys::virDomainLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Extracts domain state.
    ///
    /// Each state can be accompanied with a reason (if known) which
    /// led to the state.
    pub fn get_state(&self) -> Result<(sys::virDomainState, i32), Error> {
        let mut state: libc::c_int = -1;
        let mut reason: libc::c_int = -1;
        let ret = unsafe { sys::virDomainGetState(self.as_ptr(), &mut state, &mut reason, 0) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok((state as sys::virDomainState, reason))
    }

    /// Get the public name of the domain.
    pub fn get_name(&self) -> Result<String, Error> {
        let n = unsafe { sys::virDomainGetName(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n, nofree) })
    }

    /// Get the type of domain operating system.
    pub fn get_os_type(&self) -> Result<String, Error> {
        let n = unsafe { sys::virDomainGetOSType(self.as_ptr()) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Get the hostname for that domain.
    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        let n = unsafe { sys::virDomainGetHostname(self.as_ptr(), flags as libc::c_uint) };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    pub fn get_uuid(&self) -> Result<Uuid, Error> {
        let mut uuid: [libc::c_uchar; sys::VIR_UUID_BUFLEN as usize] =
            [0; sys::VIR_UUID_BUFLEN as usize];
        let ret = unsafe { sys::virDomainGetUUID(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(Uuid::from_bytes(uuid))
    }

    /// Get the UUID for a domain as string.
    ///
    /// For more information about UUID see RFC4122.
    pub fn get_uuid_string(&self) -> Result<String, Error> {
        let mut uuid: [libc::c_char; sys::VIR_UUID_STRING_BUFLEN as usize] =
            [0; sys::VIR_UUID_STRING_BUFLEN as usize];
        let ret = unsafe { sys::virDomainGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(uuid.as_ptr(), nofree) })
    }

    /// Get the hypervisor ID number for the domain
    pub fn get_id(&self) -> Option<u32> {
        let ret = unsafe { sys::virDomainGetID(self.as_ptr()) };
        if ret as i32 == -1 {
            return None;
        }
        Some(ret)
    }

    /// Provide an XML description of the domain. The description may
    /// be reused later to relaunch the domain with [`create_xml()`].
    ///
    /// [`create_xml()`]: Domain::create_xml
    pub fn get_xml_desc(&self, flags: sys::virDomainCreateFlags) -> Result<String, Error> {
        let xml = unsafe { sys::virDomainGetXMLDesc(self.as_ptr(), flags) };
        if xml.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(xml) })
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools. The domain will
    /// be paused only if restoring from managed state created from a
    /// paused domain.For more control, see [`create_with_flags()`].
    ///
    /// [`create_with_flags()`]: Domain::create_with_flags
    pub fn create(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainCreate(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools.
    pub fn create_with_flags(&self, flags: sys::virDomainCreateFlags) -> Result<u32, Error> {
        let res = unsafe { sys::virDomainCreateWithFlags(self.as_ptr(), flags as libc::c_uint) };
        if res == -1 {
            return Err(Error::last_error());
        }
        Ok(res as u32)
    }

    /// Extract information about a domain. Note that if the
    /// connection used to get the domain is limited only a partial
    /// set of the information can be extracted.
    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let res = unsafe { sys::virDomainGetInfo(self.as_ptr(), pinfo.as_mut_ptr()) };
        if res == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { DomainInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    /// Launch a new guest domain, based on an XML description similar
    /// to the one returned by [`get_xml_desc()`].
    ///
    /// This function may require privileged access to the hypervisor.
    ///
    /// The domain is not persistent, so its definition will disappear
    /// when it is destroyed, or if the host is restarted (see
    /// [`define_xml()`] to define persistent domains).
    ///
    /// [`get_xml_desc()`]: Domain::get_xml_desc
    /// [`define_xml()`]: Domain::define_xml
    pub fn create_xml(
        conn: &Connect,
        xml: &str,
        flags: sys::virDomainCreateFlags,
    ) -> Result<Domain, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virDomainCreateXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// [`undefine()`]. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    ///
    /// [`undefine()`]: Domain::undefine
    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Domain, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe { sys::virDomainDefineXML(conn.as_ptr(), xml_buf.as_ptr()) };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// [`undefine()`]. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    ///
    /// [`undefine()`]: Domain::undefine
    pub fn define_xml_flags(
        conn: &Connect,
        xml: &str,
        flags: sys::virDomainDefineFlags,
    ) -> Result<Domain, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ptr = unsafe {
            sys::virDomainDefineXMLFlags(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainDestroy(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    /// Reset a domain immediately without any guest OS shutdown.
    /// Reset emulates the power reset button on a machine, where all
    /// hardware sees the RST line set and reinitializes internal
    /// state.
    ///
    /// Note that there is a risk of data loss caused by reset without
    /// any guest OS shutdown.
    pub fn reset(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainReset(self.as_ptr(), 0) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy_flags(&self, flags: sys::virDomainDestroyFlagsValues) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainDestroyFlags(self.as_ptr(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
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
    pub fn shutdown(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainShutdown(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
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
    pub fn shutdown_flags(&self, flags: sys::virDomainShutdownFlagValues) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainShutdownFlags(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Reboot a domain.
    ///
    /// The domain object is still usable thereafter.
    pub fn reboot(&self, flags: sys::virDomainRebootFlagValues) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainReboot(self.as_ptr(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
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
    /// [`VIR_DOMAIN_PMSUSPENDED`].
    ///
    /// [`VIR_DOMAIN_PMSUSPENDED`]: sys::VIR_DOMAIN_PMSUSPENDED
    pub fn suspend(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainSuspend(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Resume a suspended domain.
    ///
    /// the process is restarted from the state where it was frozen by
    /// calling [`suspend()`]. This function may require privileged
    /// access Moreover, resume may not be supported if domain is in
    /// some special state like ['VIR_DOMAIN_PMSUSPENDED'].
    ///
    /// [`suspend()`]: Domain::suspend
    /// [`VIR_DOMAIN_PMSUSPENDED`]: sys::VIR_DOMAIN_PMSUSPENDED
    pub fn resume(&self) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainResume(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    /// Determine if the domain is currently running.
    pub fn is_active(&self) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainIsActive(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainUndefine(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine_flags(&self, flags: sys::virDomainUndefineFlagsValues) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainUndefineFlags(self.as_ptr(), flags) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    /// Free the domain object.
    ///
    /// The running instance is kept alive. The data structure is
    /// freed and should not be used thereafter.
    pub fn free(&mut self) -> Result<(), Error> {
        let ret = unsafe { sys::virDomainFree(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        self.ptr = None;
        Ok(())
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainIsUpdated(self.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        let mut autostart: libc::c_int = 0;
        let ret = unsafe { sys::virDomainGetAutostart(self.as_ptr(), &mut autostart) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(autostart == 1)
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainSetAutostart(self.as_ptr(), autostart as libc::c_int) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainSetMaxMemory(self.as_ptr(), memory as libc::c_ulong) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn get_max_memory(&self) -> Result<u64, Error> {
        let ret = unsafe { sys::virDomainGetMaxMemory(self.as_ptr()) };
        if ret == 0 {
            return Err(Error::last_error());
        }
        Ok(c_ulong_to_u64(ret))
    }

    pub fn get_max_vcpus(&self) -> Result<u64, Error> {
        let ret = unsafe { sys::virDomainGetMaxVcpus(self.as_ptr()) };
        if ret == 0 {
            return Err(Error::last_error());
        }
        Ok(ret as u64)
    }

    pub fn set_memory(&self, memory: u64) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainSetMemory(self.as_ptr(), memory as libc::c_ulong) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn set_memory_flags(
        &self,
        memory: u64,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<bool, Error> {
        let ret = unsafe {
            sys::virDomainSetMemoryFlags(
                self.as_ptr(),
                memory as libc::c_ulong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn set_memory_stats_period(
        &self,
        period: i32,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<bool, Error> {
        let ret = unsafe {
            sys::virDomainSetMemoryStatsPeriod(
                self.as_ptr(),
                period as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<bool, Error> {
        let ret = unsafe { sys::virDomainSetVcpus(self.as_ptr(), vcpus as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn set_vcpus_flags(
        &self,
        vcpus: u32,
        flags: sys::virDomainVcpuFlags,
    ) -> Result<bool, Error> {
        let ret = unsafe {
            sys::virDomainSetVcpusFlags(self.as_ptr(), vcpus as libc::c_uint, flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        let path_buf = CString::new(path).unwrap();
        let ret = unsafe { sys::virDomainRestore(conn.as_ptr(), path_buf.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn domain_restore_flags(
        conn: &Connect,
        path: &str,
        dxml: Option<&str>,
        flags: sys::virDomainSaveRestoreFlags,
    ) -> Result<(), Error> {
        let path_buf = CString::new(path).unwrap();
        let dxml_buf = some_string_to_cstring!(dxml);
        let ret = unsafe {
            sys::virDomainRestoreFlags(
                conn.as_ptr(),
                path_buf.as_ptr(),
                some_cstring_to_c_chars!(dxml_buf),
                flags,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn get_vcpus_flags(&self, flags: sys::virDomainVcpuFlags) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainGetVcpusFlags(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn migrate_set_max_speed(&self, bandwidth: u64, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainMigrateSetMaxSpeed(
                self.as_ptr(),
                bandwidth as libc::c_ulong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn migrate_get_max_speed(&self, flags: u32) -> Result<u64, Error> {
        let mut bandwidth: libc::c_ulong = 0;
        let ret = unsafe {
            sys::virDomainMigrateGetMaxSpeed(self.as_ptr(), &mut bandwidth, flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(c_ulong_to_u64(bandwidth))
    }

    pub fn migrate_set_compression_cache(&self, size: u64, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainMigrateSetCompressionCache(
                self.as_ptr(),
                size as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn migrate_get_compression_cache(&self, flags: u32) -> Result<u64, Error> {
        let mut size: libc::c_ulonglong = 0;
        let ret = unsafe {
            sys::virDomainMigrateGetCompressionCache(
                self.as_ptr(),
                &mut size,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(size)
    }

    pub fn migrate_set_max_downtime(&self, downtime: u64, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainMigrateSetMaxDowntime(
                self.as_ptr(),
                downtime as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainSetTime(
                self.as_ptr(),
                seconds as libc::c_longlong,
                nseconds as libc::c_uint,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn get_time(&self, flags: u32) -> Result<(i64, i32), Error> {
        let mut seconds: libc::c_longlong = 0;
        let mut nseconds: libc::c_uint = 0;
        let ret = unsafe {
            sys::virDomainGetTime(
                self.as_ptr(),
                &mut seconds,
                &mut nseconds,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok((seconds, nseconds as i32))
    }

    pub fn get_block_info(&self, disk: &str, flags: u32) -> Result<BlockInfo, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let disk_buf = CString::new(disk).unwrap();
        let ret = unsafe {
            sys::virDomainGetBlockInfo(
                self.as_ptr(),
                disk_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { BlockInfo::from_ptr(&mut pinfo.assume_init()) })
    }

    pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainPinVcpu(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainPinVcpuFlags(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainPinEmulator(
                self.as_ptr(),
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn rename(&self, new_name: &str, flags: u32) -> Result<u32, Error> {
        let new_name_buf = CString::new(new_name).unwrap();
        let ret = unsafe {
            sys::virDomainRename(self.as_ptr(), new_name_buf.as_ptr(), flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn set_user_password(&self, user: &str, password: &str, flags: u32) -> Result<u32, Error> {
        let user_buf = CString::new(user).unwrap();
        let password_buf = CString::new(password).unwrap();
        let ret = unsafe {
            sys::virDomainSetUserPassword(
                self.as_ptr(),
                user_buf.as_ptr(),
                password_buf.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn set_block_threshold(&self, dev: &str, threshold: u64, flags: u32) -> Result<u32, Error> {
        let dev_buf = CString::new(dev).unwrap();
        let ret = unsafe {
            sys::virDomainSetBlockThreshold(
                self.as_ptr(),
                dev_buf.as_ptr(),
                threshold as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainOpenGraphics(
                self.as_ptr(),
                idx as libc::c_uint,
                fd as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Result<u32, Error> {
        let ret = unsafe {
            sys::virDomainOpenGraphicsFD(self.as_ptr(), idx as libc::c_uint, flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn open_channel(
        &self,
        name: Option<&str>,
        stream: &Stream,
        flags: u32,
    ) -> Result<u32, Error> {
        let name_buf = some_string_to_cstring!(name);
        let ret = unsafe {
            sys::virDomainOpenChannel(
                self.as_ptr(),
                some_cstring_to_c_chars!(name_buf),
                stream.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn open_console(
        &self,
        name: Option<&str>,
        stream: &Stream,
        flags: u32,
    ) -> Result<u32, Error> {
        let name_buf = some_string_to_cstring!(name);
        let ret = unsafe {
            sys::virDomainOpenConsole(
                self.as_ptr(),
                some_cstring_to_c_chars!(name_buf),
                stream.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn interface_addresses(
        &self,
        source: sys::virDomainInterfaceAddressesSource,
        flags: u32,
    ) -> Result<Vec<Interface>, Error> {
        let mut addresses: *mut sys::virDomainInterfacePtr = ptr::null_mut();
        let size = unsafe {
            sys::virDomainInterfaceAddresses(self.as_ptr(), &mut addresses, source, flags)
        };
        if size == -1 {
            return Err(Error::last_error());
        }

        let mut array: Vec<Interface> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { Interface::from_ptr(*addresses.offset(x)) });
        }
        unsafe { libc::free(addresses as *mut libc::c_void) };

        Ok(array)
    }

    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        let mut pinfo = mem::MaybeUninit::uninit();
        let path_buf = CString::new(path).unwrap();
        let ret = unsafe {
            sys::virDomainInterfaceStats(
                self.as_ptr(),
                path_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                mem::size_of::<sys::virDomainInterfaceStatsStruct>(),
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { InterfaceStats::from_ptr(&mut pinfo.assume_init()) })
    }

    pub fn memory_stats(&self, flags: u32) -> Result<Vec<MemoryStat>, Error> {
        let mut pinfo: Vec<sys::virDomainMemoryStatStruct> =
            Vec::with_capacity(sys::VIR_DOMAIN_MEMORY_STAT_NR as usize);
        let ret = unsafe {
            sys::virDomainMemoryStats(
                self.as_ptr(),
                pinfo.as_mut_ptr(),
                sys::VIR_DOMAIN_MEMORY_STAT_NR,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        // low-level operation that is confirmed by return from
        // libvirt.
        unsafe { pinfo.set_len(ret as usize) };

        let mut stats: Vec<MemoryStat> = Vec::with_capacity(ret as usize);
        for x in pinfo.iter().take(ret as usize) {
            stats.push(unsafe { MemoryStat::from_ptr(x) });
        }
        Ok(stats)
    }

    /// Get progress statistics about a background job running on this domain.
    /// This method will return an error if the domain isn't active
    pub fn get_job_stats(&self, flags: sys::virDomainGetJobStatsFlags) -> Result<JobStats, Error> {
        let mut r#type: libc::c_int = 0;

        // We allow libvirt to allocate the params structure for us. libvirt will populate
        // nparams with the number of typed params returned.
        let mut nparams: libc::c_int = 0;
        let mut params: sys::virTypedParameterPtr = ptr::null_mut();

        let ret = unsafe {
            sys::virDomainGetJobStats(
                self.as_ptr(),
                &mut r#type,
                &mut params,
                &mut nparams,
                flags as libc::c_uint,
            )
        };

        if ret == -1 {
            return Err(Error::last_error());
        }

        let res: Vec<sys::virTypedParameter> =
            unsafe { Vec::from_raw_parts(params, nparams as usize, nparams as usize) };

        Ok((r#type, res).into())
    }

    /// Get progress information about a background job running on this domain.
    /// NOTE: Only a subset of the fields in JobStats are populated by this method. If you want to
    /// populate more fields then you should use [`Self::get_job_stats`].
    pub fn get_job_info(&self) -> Result<JobStats, Error> {
        unsafe {
            let mut job_info = mem::MaybeUninit::uninit();
            let ret = sys::virDomainGetJobInfo(self.as_ptr(), job_info.as_mut_ptr());

            if ret == -1 {
                return Err(Error::last_error());
            }

            let ptr: sys::virDomainJobInfoPtr = &mut job_info.assume_init();

            Ok(JobStats {
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
            })
        }
    }

    pub fn save_image_get_xml_desc(
        conn: &Connect,
        file: &str,
        flags: u32,
    ) -> Result<String, Error> {
        let file_buf = CString::new(file).unwrap();
        let ptr = unsafe {
            sys::virDomainSaveImageGetXMLDesc(
                conn.as_ptr(),
                file_buf.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(ptr) })
    }

    pub fn save_image_define_xml(
        conn: &Connect,
        file: &str,
        dxml: &str,
        flags: u32,
    ) -> Result<u32, Error> {
        let file_buf = CString::new(file).unwrap();
        let dxml_buf = CString::new(dxml).unwrap();
        let ret = unsafe {
            sys::virDomainSaveImageDefineXML(
                conn.as_ptr(),
                file_buf.as_ptr(),
                dxml_buf.as_ptr(),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn attach_device(&self, xml: &str) -> Result<u32, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe { sys::virDomainAttachDevice(self.as_ptr(), xml_buf.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn attach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe {
            sys::virDomainAttachDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn detach_device(&self, xml: &str) -> Result<u32, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe { sys::virDomainDetachDevice(self.as_ptr(), xml_buf.as_ptr()) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn detach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe {
            sys::virDomainDetachDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn update_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        let xml_buf = CString::new(xml).unwrap();
        let ret = unsafe {
            sys::virDomainUpdateDeviceFlags(self.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn managed_save(&self, flags: u32) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainManagedSave(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn has_managed_save(&self, flags: u32) -> Result<bool, Error> {
        let ret =
            unsafe { sys::virDomainHasManagedSaveImage(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret == 1)
    }

    pub fn managed_save_remove(&self, flags: u32) -> Result<u32, Error> {
        let ret = unsafe { sys::virDomainManagedSaveRemove(self.as_ptr(), flags as libc::c_uint) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn core_dump(&self, to: &str, flags: u32) -> Result<u32, Error> {
        let to_buf = CString::new(to).unwrap();
        let ret = unsafe {
            sys::virDomainCoreDump(self.as_ptr(), to_buf.as_ptr(), flags as libc::c_uint)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn core_dump_with_format(&self, to: &str, format: u32, flags: u32) -> Result<u32, Error> {
        let to_buf = CString::new(to).unwrap();
        let ret = unsafe {
            sys::virDomainCoreDumpWithFormat(
                self.as_ptr(),
                to_buf.as_ptr(),
                format as libc::c_uint,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn set_metadata(
        &self,
        kind: i32,
        metadata: Option<&str>,
        key: Option<&str>,
        uri: Option<&str>,
        flags: u32,
    ) -> Result<u32, Error> {
        let metadata_buf = some_string_to_cstring!(metadata);
        let key_buf = some_string_to_cstring!(key);
        let uri_buf = some_string_to_cstring!(uri);
        let ret = unsafe {
            sys::virDomainSetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                some_cstring_to_c_chars!(metadata_buf),
                some_cstring_to_c_chars!(key_buf),
                some_cstring_to_c_chars!(uri_buf),
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn get_metadata(&self, kind: i32, uri: Option<&str>, flags: u32) -> Result<String, Error> {
        let uri_buf = some_string_to_cstring!(uri);
        let n = unsafe {
            sys::virDomainGetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                some_cstring_to_c_chars!(uri_buf),
                flags as libc::c_uint,
            )
        };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    pub fn block_resize(&self, disk: &str, size: u64, flags: u32) -> Result<u32, Error> {
        let disk_buf = CString::new(disk).unwrap();
        let ret = unsafe {
            sys::virDomainBlockResize(
                self.as_ptr(),
                disk_buf.as_ptr(),
                size as libc::c_ulonglong,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn get_memory_parameters(&self, flags: u32) -> Result<MemoryParameters, Error> {
        let mut nparams: libc::c_int = 0;
        let ret = unsafe {
            sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let ret = unsafe {
            sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        unsafe { params.set_len(nparams as usize) };
        Ok(MemoryParameters::from_vec(params))
    }

    pub fn set_memory_parameters(
        &self,
        params: MemoryParameters,
        flags: u32,
    ) -> Result<u32, Error> {
        let mut cparams = params.to_vec();

        let ret = unsafe {
            sys::virDomainSetMemoryParameters(
                self.as_ptr(),
                cparams.as_mut_ptr(),
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

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
        let ptr = unsafe {
            sys::virDomainMigrate(
                self.as_ptr(),
                dconn.as_ptr(),
                flags as libc::c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                some_cstring_to_c_chars!(uri_buf),
                bandwidth as libc::c_ulong,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

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
        let ptr = unsafe {
            sys::virDomainMigrate2(
                self.as_ptr(),
                dconn.as_ptr(),
                some_cstring_to_c_chars!(dxml_buf),
                flags as libc::c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                some_cstring_to_c_chars!(uri_buf),
                bandwidth as libc::c_ulong,
            )
        };
        if ptr.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { Domain::from_ptr(ptr) })
    }

    pub fn migrate_to_uri(
        &self,
        duri: &str,
        flags: u32,
        dname: Option<&str>,
        bandwidth: u64,
    ) -> Result<(), Error> {
        let duri_buf = CString::new(duri).unwrap();
        let dname_buf = some_string_to_cstring!(dname);
        let ret = unsafe {
            sys::virDomainMigrateToURI(
                self.as_ptr(),
                duri_buf.as_ptr(),
                flags as libc::c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                bandwidth as libc::c_ulong,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

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
        let ret = unsafe {
            sys::virDomainMigrateToURI2(
                self.as_ptr(),
                some_cstring_to_c_chars!(dconn_uri_buf),
                some_cstring_to_c_chars!(mig_uri_buf),
                some_cstring_to_c_chars!(dxml_buf),
                flags as libc::c_ulong,
                some_cstring_to_c_chars!(dname_buf),
                bandwidth as libc::c_ulong,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn get_numa_parameters(&self, flags: u32) -> Result<NUMAParameters, Error> {
        let mut nparams: libc::c_int = 0;
        let ret = unsafe {
            sys::virDomainGetNumaParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let ret = unsafe {
            sys::virDomainGetNumaParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        unsafe { params.set_len(nparams as usize) };
        let nparams = NUMAParameters::from_vec(params.clone());
        unsafe { typed_params_release_c_chars!(params) };

        Ok(nparams)
    }

    pub fn set_numa_parameters(&self, params: NUMAParameters, flags: u32) -> Result<u32, Error> {
        let mut cparams = params.to_vec();
        let ret = unsafe {
            sys::virDomainSetNumaParameters(
                self.as_ptr(),
                cparams.as_mut_ptr(),
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            )
        };
        unsafe { typed_params_release_c_chars!(cparams) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret as u32)
    }

    pub fn list_all_snapshots(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
        let size = unsafe {
            sys::virDomainListAllSnapshots(self.as_ptr(), &mut snaps, flags as libc::c_uint)
        };
        if size == -1 {
            return Err(Error::last_error());
        }

        let mut array: Vec<DomainSnapshot> = Vec::new();
        for x in 0..size as isize {
            array.push(unsafe { DomainSnapshot::from_ptr(*snaps.offset(x)) });
        }
        unsafe { libc::free(snaps as *mut libc::c_void) };

        Ok(array)
    }

    /// Get the cpu scheduler type for the domain
    pub fn get_scheduler_type(&self) -> Result<(String, i32), Error> {
        let mut nparams: libc::c_int = -1;
        let sched_type = unsafe { sys::virDomainGetSchedulerType(self.as_ptr(), &mut nparams) };
        if sched_type.is_null() {
            return Err(Error::last_error());
        }

        Ok((unsafe { c_chars_to_string!(sched_type) }, nparams))
    }

    /// Get the scheduler parameters for the domain.
    pub fn get_scheduler_parameters(&self) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let ret = unsafe {
            sys::virDomainGetSchedulerParameters(self.as_ptr(), params.as_mut_ptr(), &mut nparams)
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        unsafe { params.set_len(nparams as usize) };
        Ok(SchedulerInfo::from_vec(params, sched_type))
    }

    /// Get the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the domain modification [`Impact`]: [`VIR_DOMAIN_AFFECT_CURRENT`],
    /// [`VIR_DOMAIN_AFFECT_LIVE`] or [`VIR_DOMAIN_AFFECT_CONFIG`].
    ///
    /// [`Impact`]: sys::virDomainModificationImpact
    /// [`VIR_DOMAIN_AFFECT_CURRENT`]: sys::VIR_DOMAIN_AFFECT_CURRENT
    /// [`VIR_DOMAIN_AFFECT_LIVE`]: sys::VIR_DOMAIN_AFFECT_LIVE
    /// [`VIR_DOMAIN_AFFECT_CONFIG`]: sys::VIR_DOMAIN_AFFECT_CONFIG
    pub fn get_scheduler_parameters_flags(
        &self,
        flags: sys::virDomainModificationImpact,
    ) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
        let ret = unsafe {
            sys::virDomainGetSchedulerParametersFlags(
                self.as_ptr(),
                params.as_mut_ptr(),
                &mut nparams,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        unsafe { params.set_len(nparams as usize) };
        Ok(SchedulerInfo::from_vec(params, sched_type))
    }

    /// Set the scheduler parameters for the domain.
    pub fn set_scheduler_parameters(&self, sched_info: &SchedulerInfo) -> Result<i32, Error> {
        let mut params = sched_info.to_vec();
        let ret = unsafe {
            sys::virDomainSetSchedulerParameters(
                self.as_ptr(),
                params.as_mut_ptr(),
                params.len() as libc::c_int,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret)
    }

    /// Set the scheduler parameters for the domain for the configuration
    /// as specified by the flags.
    /// # Arguments
    ///
    /// * `flags` - Specifies the domain modification [`Impact`]: [`VIR_DOMAIN_AFFECT_CURRENT`],
    /// [`VIR_DOMAIN_AFFECT_LIVE`] or [`VIR_DOMAIN_AFFECT_CONFIG`].
    ///
    /// [`Impact`]: sys::virDomainModificationImpact
    /// [`VIR_DOMAIN_AFFECT_CURRENT`]: sys::VIR_DOMAIN_AFFECT_CURRENT
    /// [`VIR_DOMAIN_AFFECT_LIVE`]: sys::VIR_DOMAIN_AFFECT_LIVE
    /// [`VIR_DOMAIN_AFFECT_CONFIG`]: sys::VIR_DOMAIN_AFFECT_CONFIG
    pub fn set_scheduler_parameters_flags(
        &self,
        sched_info: &SchedulerInfo,
        flags: sys::virDomainModificationImpact,
    ) -> Result<i32, Error> {
        let mut params = sched_info.to_vec();
        let ret = unsafe {
            sys::virDomainSetSchedulerParametersFlags(
                self.as_ptr(),
                params.as_mut_ptr(),
                params.len() as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(ret)
    }

    /// Send key(s) to the guest.
    /// # Arguments
    ///
    /// * `codeset` - Specifies the code set of keycodes.
    /// * `holdtime` - Specifies the duration (in milliseconds) that the keys will be held.
    /// * `keycodes` - Specifies the array of keycodes.
    /// * `nkeycodes` - Specifies the number of keycodes.
    /// * `flags` - Extra flags; not used yet, so callers should always pass 0..
    pub fn send_key(
        &self,
        codeset: sys::virKeycodeSet,
        holdtime: u32,
        keycodes: *mut u32,
        nkeycodes: i32,
        flags: u32,
    ) -> Result<(), Error> {
        let ret = unsafe {
            sys::virDomainSendKey(
                self.as_ptr(),
                codeset as libc::c_uint,
                holdtime as libc::c_uint,
                keycodes as *mut libc::c_uint,
                nkeycodes as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
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
    pub fn screenshot(&self, stream: &Stream, screen: u32, flags: u32) -> Result<String, Error> {
        let n = unsafe {
            sys::virDomainScreenshot(
                self.as_ptr(),
                stream.as_ptr(),
                screen as libc::c_uint,
                flags as libc::c_uint,
            )
        };
        if n.is_null() {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(n) })
    }

    /// Send an arbitrary monitor command cmd to domain through the QEMU monitor.
    ///
    /// * `cmd` - the QEMU monitor command string
    /// * `flags` - bitwise-or of supported [`virDomainQemuMonitorCommandFlags`]
    ///
    /// [`virDomainQemuMonitorCommandFlags`]: sys::virDomainQemuMonitorCommandFlags
    #[cfg(feature = "qemu")]
    pub fn qemu_monitor_command(&self, cmd: &str, flags: u32) -> Result<String, Error> {
        let mut result: *mut libc::c_char = std::ptr::null_mut();
        let cmd_buf = CString::new(cmd).unwrap();
        let ret = unsafe {
            sys::virDomainQemuMonitorCommand(
                self.as_ptr(),
                cmd_buf.as_ptr(),
                &mut result,
                flags as libc::c_uint,
            )
        };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(unsafe { c_chars_to_string!(result) })
    }
}
