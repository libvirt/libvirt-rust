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

use std::ffi::CStr;
use std::ffi::CString;
use std::{mem, ptr, str};

use crate::connect::Connect;
use crate::domain_snapshot::DomainSnapshot;
use crate::error::Error;
use crate::stream::Stream;
use crate::util::c_ulong_to_u64;

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
            cpu_time: (*ptr).cpuTime as u64,
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
            capacity: (*ptr).capacity as u64,
            allocation: (*ptr).capacity as u64,
            physical: (*ptr).capacity as u64,
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
    pub fn from_vec(vec: Vec<sys::virTypedParameter>) -> MemoryParameters {
        unsafe {
            let mut ret = MemoryParameters::default();
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "hard_limit" => ret.hard_limit = Some(param.value.ul),
                    "soft_limit" => ret.soft_limit = Some(param.value.ul),
                    "min_guarantee" => ret.min_guarantee = Some(param.value.ul),
                    "swap_hard_limit" => ret.swap_hard_limit = Some(param.value.ul),
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
    pub mode: Option<sys::virDomainNumatuneMemMode>,
}

impl NUMAParameters {
    pub fn from_vec(vec: Vec<sys::virTypedParameter>) -> NUMAParameters {
        unsafe {
            let mut ret = NUMAParameters::default();
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "numa_nodeset" => ret.node_set = Some(c_chars_to_string!(param.value.s)),
                    "numa_mode" => ret.mode = Some(param.value.ui),
                    unknow => panic!("Field not implemented for NUMAParameters, {:?}", unknow),
                }
            }
            ret
        }
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
            val: (*ptr).val as u64,
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
    pub fn from_vec(vec: Vec<sys::virTypedParameter>, scheduler_type: String) -> SchedulerInfo {
        unsafe {
            let mut ret = SchedulerInfo {
                scheduler_type,
                ..Default::default()
            };
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "cpu_shares" => ret.cpu_shares = Some(param.value.ul),
                    "vcpu_period" => ret.vcpu_bw.period = Some(param.value.ul),
                    "vcpu_quota" => ret.vcpu_bw.quota = Some(param.value.l),
                    "emulator_period" => ret.emulator_bw.period = Some(param.value.ul),
                    "emulator_quota" => ret.emulator_bw.quota = Some(param.value.l),
                    "global_period" => ret.global_bw.period = Some(param.value.ul),
                    "global_quota" => ret.global_bw.quota = Some(param.value.l),
                    "iothread_period" => ret.iothread_bw.period = Some(param.value.ul),
                    "iothread_quota" => ret.iothread_bw.quota = Some(param.value.l),
                    unknow => panic!("Field not implemented for SchedulerInfo, {:?}", unknow),
                }
            }
            ret
        }
    }

    pub fn to_vec(&self) -> Vec<sys::virTypedParameter> {
        let mut cparams: Vec<sys::virTypedParameter> = Vec::new();

        if let Some(shares) = self.cpu_shares {
            cparams.push(sys::virTypedParameter {
                field: to_arr("cpu_shares\0"),
                type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                value: sys::_virTypedParameterValue { ul: shares },
            });
        }
        if let Some(period) = self.vcpu_bw.period {
            cparams.push(sys::virTypedParameter {
                field: to_arr("vcpu_period\0"),
                type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                value: sys::_virTypedParameterValue { ul: period },
            });
        }
        if let Some(quota) = self.vcpu_bw.quota {
            cparams.push(sys::virTypedParameter {
                field: to_arr("vcpu_quota\0"),
                type_: sys::VIR_TYPED_PARAM_LLONG as libc::c_int,
                value: sys::_virTypedParameterValue { l: quota },
            });
        }
        if let Some(period) = self.emulator_bw.period {
            cparams.push(sys::virTypedParameter {
                field: to_arr("emulator_period\0"),
                type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                value: sys::_virTypedParameterValue { ul: period },
            });
        }
        if let Some(quota) = self.emulator_bw.quota {
            cparams.push(sys::virTypedParameter {
                field: to_arr("emulator_quota\0"),
                type_: sys::VIR_TYPED_PARAM_LLONG as libc::c_int,
                value: sys::_virTypedParameterValue { l: quota },
            });
        }
        if let Some(period) = self.global_bw.period {
            cparams.push(sys::virTypedParameter {
                field: to_arr("global_period\0"),
                type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                value: sys::_virTypedParameterValue { ul: period },
            });
        }
        if let Some(quota) = self.global_bw.quota {
            cparams.push(sys::virTypedParameter {
                field: to_arr("global_quota\0"),
                type_: sys::VIR_TYPED_PARAM_LLONG as libc::c_int,
                value: sys::_virTypedParameterValue { l: quota },
            });
        }
        if let Some(period) = self.iothread_bw.period {
            cparams.push(sys::virTypedParameter {
                field: to_arr("iothread_period\0"),
                type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                value: sys::_virTypedParameterValue { ul: period },
            });
        }
        if let Some(quota) = self.iothread_bw.quota {
            cparams.push(sys::virTypedParameter {
                field: to_arr("iothread_quota\0"),
                type_: sys::VIR_TYPED_PARAM_LLONG as libc::c_int,
                value: sys::_virTypedParameterValue { l: quota },
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
                panic!("Unable to drop memory for Domain: {}", e)
            }
        }
    }
}

fn to_arr(name: &str) -> [libc::c_char; 80] {
    let mut field: [libc::c_char; 80] = [0; 80];
    for (a, c) in field.iter_mut().zip(name.as_bytes()) {
        *a = *c as libc::c_char
    }
    field
}

impl Domain {
    pub fn new(ptr: sys::virDomainPtr) -> Domain {
        Domain { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virDomainPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virDomainGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Connect::new(ptr))
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let id_buf = CString::new(id).unwrap();
            let ptr = sys::virDomainLookupByName(conn.as_ptr(), id_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let uuid_buf = CString::new(uuid).unwrap();
            let ptr = sys::virDomainLookupByUUIDString(conn.as_ptr(), uuid_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    /// Extracts domain state.
    ///
    /// Each state can be accompanied with a reason (if known) which
    /// led to the state.
    pub fn get_state(&self) -> Result<(sys::virDomainState, i32), Error> {
        unsafe {
            let mut state: libc::c_int = -1;
            let mut reason: libc::c_int = -1;
            let ret = sys::virDomainGetState(self.as_ptr(), &mut state, &mut reason, 0);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok((state as sys::virDomainState, reason as i32))
        }
    }

    /// Get the public name of the domain.
    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n, nofree))
        }
    }

    /// Get the type of domain operating system.
    pub fn get_os_type(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetOSType(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    /// Get the hostname for that domain.
    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetHostname(self.as_ptr(), flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    /// Get the UUID for a domain as string.
    ///
    /// For more information about UUID see RFC4122.
    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virDomainGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(uuid.as_ptr(), nofree))
        }
    }

    /// Get the hypervisor ID number for the domain
    pub fn get_id(&self) -> Option<u32> {
        unsafe {
            let ret = sys::virDomainGetID(self.as_ptr());
            if ret as i32 == -1 {
                return None;
            }
            Some(ret)
        }
    }

    /// Provide an XML description of the domain. The description may
    /// be reused later to relaunch the domain with `create_xml()`.
    pub fn get_xml_desc(&self, flags: sys::virDomainCreateFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virDomainGetXMLDesc(self.as_ptr(), flags);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(xml))
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools. The domain will
    /// be paused only if restoring from managed state created from a
    /// paused domain.  For more control, see `create_with_flags()`.
    pub fn create(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools.
    pub fn create_with_flags(&self, flags: sys::virDomainCreateFlags) -> Result<u32, Error> {
        unsafe {
            let res = sys::virDomainCreateWithFlags(self.as_ptr(), flags as libc::c_uint);
            if res == -1 {
                return Err(Error::last_error());
            }
            Ok(res as u32)
        }
    }

    /// Extract information about a domain. Note that if the
    /// connection used to get the domain is limited only a partial
    /// set of the information can be extracted.
    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let res = sys::virDomainGetInfo(self.as_ptr(), pinfo.as_mut_ptr());
            if res == -1 {
                return Err(Error::last_error());
            }
            Ok(DomainInfo::from_ptr(&mut pinfo.assume_init()))
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
        flags: sys::virDomainCreateFlags,
    ) -> Result<Domain, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr =
                sys::virDomainCreateXML(conn.as_ptr(), xml_buf.as_ptr(), flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
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
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virDomainDefineXML(conn.as_ptr(), xml_buf.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
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
        flags: sys::virDomainDefineFlags,
    ) -> Result<Domain, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ptr = sys::virDomainDefineXMLFlags(
                conn.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainDestroy(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
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
            let ret = sys::virDomainReset(self.as_ptr(), 0);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy_flags(&self, flags: sys::virDomainDestroyFlagsValues) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainDestroyFlags(self.as_ptr(), flags);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
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
            let ret = sys::virDomainShutdown(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Reboot a domain.
    ///
    /// The domain object is still usable thereafter.
    pub fn reboot(&self, flags: sys::virDomainRebootFlagValues) -> Result<(), Error> {
        unsafe {
            if sys::virDomainReboot(self.as_ptr(), flags) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
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
            let ret = sys::virDomainSuspend(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
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
            let ret = sys::virDomainResume(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    /// Determine if the domain is currently running.
    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine_flags(&self, flags: sys::virDomainUndefineFlagsValues) -> Result<(), Error> {
        unsafe {
            if sys::virDomainUndefineFlags(self.as_ptr(), flags) == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    /// Free the domain object.
    ///
    /// The running instance is kept alive. The data structure is
    /// freed and should not be used thereafter.
    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            Ok(())
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainIsUpdated(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut autostart: libc::c_int = 0;
            let ret = sys::virDomainGetAutostart(self.as_ptr(), &mut autostart);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(autostart == 1)
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetAutostart(self.as_ptr(), autostart as libc::c_int);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetMaxMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn get_max_memory(&self) -> Result<u64, Error> {
        unsafe {
            let ret = sys::virDomainGetMaxMemory(self.as_ptr());
            if ret == 0 {
                return Err(Error::last_error());
            }
            Ok(c_ulong_to_u64(ret))
        }
    }

    pub fn get_max_vcpus(&self) -> Result<u64, Error> {
        unsafe {
            let ret = sys::virDomainGetMaxVcpus(self.as_ptr());
            if ret == 0 {
                return Err(Error::last_error());
            }
            Ok(ret as u64)
        }
    }

    pub fn set_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn set_memory_flags(
        &self,
        memory: u64,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetMemoryFlags(
                self.as_ptr(),
                memory as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn set_memory_stats_period(
        &self,
        period: i32,
        flags: sys::virDomainMemoryModFlags,
    ) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetMemoryStatsPeriod(
                self.as_ptr(),
                period as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetVcpus(self.as_ptr(), vcpus as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn set_vcpus_flags(
        &self,
        vcpus: u32,
        flags: sys::virDomainVcpuFlags,
    ) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetVcpusFlags(
                self.as_ptr(),
                vcpus as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        unsafe {
            let path_buf = CString::new(path).unwrap();
            let ret = sys::virDomainRestore(conn.as_ptr(), path_buf.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    // TODO: expose dxml parameter
    pub fn domain_restore_flags(
        conn: &Connect,
        path: &str,
        flags: sys::virDomainSaveRestoreFlags,
    ) -> Result<(), Error> {
        unsafe {
            let path_buf = CString::new(path).unwrap();
            let ret =
                sys::virDomainRestoreFlags(conn.as_ptr(), path_buf.as_ptr(), ptr::null(), flags);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn get_vcpus_flags(&self, flags: sys::virDomainVcpuFlags) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainGetVcpusFlags(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn migrate_set_max_speed(&self, bandwidth: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainMigrateSetMaxSpeed(
                self.as_ptr(),
                bandwidth as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn migrate_get_max_speed(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut bandwidth: libc::c_ulong = 0;
            let ret = sys::virDomainMigrateGetMaxSpeed(
                self.as_ptr(),
                &mut bandwidth,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(c_ulong_to_u64(bandwidth))
        }
    }

    pub fn migrate_set_compression_cache(&self, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainMigrateSetCompressionCache(
                self.as_ptr(),
                size as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn migrate_get_compression_cache(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut size: libc::c_ulong = 0;
            let ret = sys::virDomainMigrateGetCompressionCache(
                self.as_ptr(),
                &mut size,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(size as u64)
        }
    }

    pub fn migrate_set_max_downtime(&self, downtime: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainMigrateSetMaxDowntime(
                self.as_ptr(),
                downtime as libc::c_ulong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainSetTime(
                self.as_ptr(),
                seconds as libc::c_long,
                nseconds as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn get_time(&self, flags: u32) -> Result<(i64, i32), Error> {
        unsafe {
            let mut seconds: libc::c_long = 0;
            let mut nseconds: libc::c_uint = 0;
            let ret = sys::virDomainGetTime(
                self.as_ptr(),
                &mut seconds,
                &mut nseconds,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok((seconds as i64, nseconds as i32))
        }
    }

    pub fn get_block_info(&self, disk: &str, flags: u32) -> Result<BlockInfo, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let disk_buf = CString::new(disk).unwrap();
            let ret = sys::virDomainGetBlockInfo(
                self.as_ptr(),
                disk_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(BlockInfo::from_ptr(&mut pinfo.assume_init()))
        }
    }

    pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainPinVcpu(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainPinVcpuFlags(
                self.as_ptr(),
                vcpu as libc::c_uint,
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainPinEmulator(
                self.as_ptr(),
                cpumap.as_ptr() as *mut _,
                cpumap.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn rename(&self, new_name: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let new_name_buf = CString::new(new_name).unwrap();
            let ret =
                sys::virDomainRename(self.as_ptr(), new_name_buf.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn set_user_password(&self, user: &str, password: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let user_buf = CString::new(user).unwrap();
            let password_buf = CString::new(password).unwrap();
            let ret = sys::virDomainSetUserPassword(
                self.as_ptr(),
                user_buf.as_ptr(),
                password_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn set_block_threshold(&self, dev: &str, threshold: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let dev_buf = CString::new(dev).unwrap();
            let ret = sys::virDomainSetBlockThreshold(
                self.as_ptr(),
                dev_buf.as_ptr(),
                threshold as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainOpenGraphics(
                self.as_ptr(),
                idx as libc::c_uint,
                fd as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainOpenGraphicsFD(
                self.as_ptr(),
                idx as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn open_channel(&self, name: &str, stream: &Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let name_buf = CString::new(name).unwrap();
            let ret = sys::virDomainOpenChannel(
                self.as_ptr(),
                name_buf.as_ptr(),
                stream.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn open_console(&self, name: &str, stream: &Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let name_buf = CString::new(name).unwrap();
            let ret = sys::virDomainOpenConsole(
                self.as_ptr(),
                name_buf.as_ptr(),
                stream.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn interface_addresses(
        &self,
        source: sys::virDomainInterfaceAddressesSource,
        flags: u32,
    ) -> Result<Vec<Interface>, Error> {
        unsafe {
            let mut addresses: *mut sys::virDomainInterfacePtr = ptr::null_mut();
            let size =
                sys::virDomainInterfaceAddresses(self.as_ptr(), &mut addresses, source, flags);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<Interface> = Vec::new();
            for x in 0..size as isize {
                array.push(Interface::from_ptr(*addresses.offset(x)));
            }
            libc::free(addresses as *mut libc::c_void);

            Ok(array)
        }
    }

    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        unsafe {
            let mut pinfo = mem::MaybeUninit::uninit();
            let path_buf = CString::new(path).unwrap();
            let ret = sys::virDomainInterfaceStats(
                self.as_ptr(),
                path_buf.as_ptr(),
                pinfo.as_mut_ptr(),
                mem::size_of::<sys::virDomainInterfaceStatsStruct>(),
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(InterfaceStats::from_ptr(&mut pinfo.assume_init()))
        }
    }

    pub fn memory_stats(&self, flags: u32) -> Result<Vec<MemoryStat>, Error> {
        unsafe {
            let mut pinfo: Vec<sys::virDomainMemoryStatStruct> =
                Vec::with_capacity(sys::VIR_DOMAIN_MEMORY_STAT_NR as usize);
            let ret = sys::virDomainMemoryStats(
                self.as_ptr(),
                pinfo.as_mut_ptr(),
                sys::VIR_DOMAIN_MEMORY_STAT_NR,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            // low-level operation that is confirmed by return from
            // libvirt.
            pinfo.set_len(ret as usize);

            let mut stats: Vec<MemoryStat> = Vec::with_capacity(ret as usize);
            for x in pinfo.iter().take(ret as usize) {
                stats.push(MemoryStat::from_ptr(x));
            }
            Ok(stats)
        }
    }

    pub fn save_image_get_xml_desc(
        conn: &Connect,
        file: &str,
        flags: u32,
    ) -> Result<String, Error> {
        unsafe {
            let file_buf = CString::new(file).unwrap();
            let ptr = sys::virDomainSaveImageGetXMLDesc(
                conn.as_ptr(),
                file_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(ptr))
        }
    }

    pub fn save_image_define_xml(
        conn: &Connect,
        file: &str,
        dxml: &str,
        flags: u32,
    ) -> Result<u32, Error> {
        unsafe {
            let file_buf = CString::new(file).unwrap();
            let dxml_buf = CString::new(dxml).unwrap();
            let ret = sys::virDomainSaveImageDefineXML(
                conn.as_ptr(),
                file_buf.as_ptr(),
                dxml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn attach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virDomainAttachDevice(self.as_ptr(), xml_buf.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn attach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virDomainAttachDeviceFlags(
                self.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn detach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virDomainDetachDevice(self.as_ptr(), xml_buf.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn detach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virDomainDetachDeviceFlags(
                self.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn update_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let xml_buf = CString::new(xml).unwrap();
            let ret = sys::virDomainUpdateDeviceFlags(
                self.as_ptr(),
                xml_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn managed_save(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainManagedSave(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn has_managed_save(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainHasManagedSaveImage(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret == 1)
        }
    }

    pub fn managed_save_remove(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainManagedSaveRemove(self.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn core_dump(&self, to: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let to_buf = CString::new(to).unwrap();
            let ret = sys::virDomainCoreDump(self.as_ptr(), to_buf.as_ptr(), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn core_dump_with_format(&self, to: &str, format: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let to_buf = CString::new(to).unwrap();
            let ret = sys::virDomainCoreDumpWithFormat(
                self.as_ptr(),
                to_buf.as_ptr(),
                format as libc::c_uint,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
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
            let metadata_buf = CString::new(metadata).unwrap();
            let key_buf = CString::new(key).unwrap();
            let uri_buf = CString::new(uri).unwrap();
            let ret = sys::virDomainSetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                metadata_buf.as_ptr(),
                key_buf.as_ptr(),
                uri_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn get_metadata(&self, kind: i32, uri: &str, flags: u32) -> Result<String, Error> {
        unsafe {
            let uri_buf = CString::new(uri).unwrap();
            let n = sys::virDomainGetMetadata(
                self.as_ptr(),
                kind as libc::c_int,
                uri_buf.as_ptr(),
                flags as libc::c_uint,
            );
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    pub fn block_resize(&self, disk: &str, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let disk_buf = CString::new(disk).unwrap();
            let ret = sys::virDomainBlockResize(
                self.as_ptr(),
                disk_buf.as_ptr(),
                size as libc::c_ulonglong,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn get_memory_parameters(&self, flags: u32) -> Result<MemoryParameters, Error> {
        unsafe {
            let mut nparams: libc::c_int = 0;
            let ret = sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
            let ret = sys::virDomainGetMemoryParameters(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            params.set_len(nparams as usize);
            Ok(MemoryParameters::from_vec(params))
        }
    }

    pub fn set_memory_parameters(
        &self,
        params: MemoryParameters,
        flags: u32,
    ) -> Result<u32, Error> {
        unsafe {
            let mut cparams: Vec<sys::virTypedParameter> = Vec::new();
            if params.hard_limit.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("hard_limit\0"),
                    type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        ul: params.hard_limit.unwrap(),
                    },
                })
            }
            if params.soft_limit.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("soft_limit\0"),
                    type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        ul: params.soft_limit.unwrap(),
                    },
                })
            }
            if params.min_guarantee.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("min_guarantee\0"),
                    type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        ul: params.min_guarantee.unwrap(),
                    },
                })
            }
            if params.swap_hard_limit.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("swap_hard_limit\0"),
                    type_: sys::VIR_TYPED_PARAM_ULLONG as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        ul: params.swap_hard_limit.unwrap(),
                    },
                })
            }

            let ret = sys::virDomainSetMemoryParameters(
                self.as_ptr(),
                &mut cparams[0],
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
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
            let dname_buf = CString::new("").unwrap();
            let uri_buf = CString::new(uri).unwrap();
            let ptr = sys::virDomainMigrate(
                self.as_ptr(),
                dconn.as_ptr(),
                flags as libc::c_ulong,
                dname_buf.as_ptr(),
                uri_buf.as_ptr(),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
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
            let dxml_buf = CString::new(dxml).unwrap();
            let dname_buf = CString::new("").unwrap();
            let uri_buf = CString::new(uri).unwrap();
            let ptr = sys::virDomainMigrate2(
                self.as_ptr(),
                dconn.as_ptr(),
                dxml_buf.as_ptr(),
                flags as libc::c_ulong,
                dname_buf.as_ptr(),
                uri_buf.as_ptr(),
                bandwidth as libc::c_ulong,
            );
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            Ok(Domain::new(ptr))
        }
    }

    pub fn migrate_to_uri(&self, duri: &str, flags: u32, bandwidth: u64) -> Result<(), Error> {
        unsafe {
            let duri_buf = CString::new(duri).unwrap();
            let dname_buf = CString::new("").unwrap();
            let ret = sys::virDomainMigrateToURI(
                self.as_ptr(),
                duri_buf.as_ptr(),
                flags as libc::c_ulong,
                dname_buf.as_ptr(),
                bandwidth as libc::c_ulong,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn migrate_to_uri2(
        &self,
        dconn_uri: &str,
        mig_uri: &str,
        dxml: &str,
        flags: u32,
        bandwidth: u64,
    ) -> Result<(), Error> {
        unsafe {
            let dconn_uri_buf = CString::new(dconn_uri).unwrap();
            let mig_uri_buf = CString::new(mig_uri).unwrap();
            let dxml_buf = CString::new(dxml).unwrap();
            let dname_buf = CString::new("").unwrap();
            let ret = sys::virDomainMigrateToURI2(
                self.as_ptr(),
                dconn_uri_buf.as_ptr(),
                mig_uri_buf.as_ptr(),
                dxml_buf.as_ptr(),
                flags as libc::c_ulong,
                dname_buf.as_ptr(),
                bandwidth as libc::c_ulong,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(())
        }
    }

    pub fn get_numa_parameters(&self, flags: u32) -> Result<NUMAParameters, Error> {
        unsafe {
            let mut nparams: libc::c_int = 0;
            let ret = sys::virDomainGetNumaParameters(
                self.as_ptr(),
                ptr::null_mut(),
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
            let ret = sys::virDomainGetNumaParameters(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            params.set_len(nparams as usize);
            Ok(NUMAParameters::from_vec(params))
        }
    }

    pub fn set_numa_parameters(&self, params: NUMAParameters, flags: u32) -> Result<u32, Error> {
        unsafe {
            let mut cparams: Vec<sys::virTypedParameter> = Vec::new();
            if params.node_set.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("numa_nodeset\0"),
                    type_: sys::VIR_TYPED_PARAM_STRING as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        s: string_to_mut_c_chars!(params.node_set.unwrap()),
                    },
                })
            }
            if params.mode.is_some() {
                cparams.push(sys::virTypedParameter {
                    field: to_arr("numa_mode\0"),
                    type_: sys::VIR_TYPED_PARAM_INT as libc::c_int,
                    value: sys::_virTypedParameterValue {
                        ui: params.mode.unwrap(),
                    },
                })
            }

            let ret = sys::virDomainSetNumaParameters(
                self.as_ptr(),
                &mut cparams[0],
                cparams.len() as libc::c_int,
                flags as libc::c_uint,
            );
            typed_params_release_c_chars!(cparams);
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret as u32)
        }
    }

    pub fn list_all_snapshots(&self, flags: u32) -> Result<Vec<DomainSnapshot>, Error> {
        unsafe {
            let mut snaps: *mut sys::virDomainSnapshotPtr = ptr::null_mut();
            let size =
                sys::virDomainListAllSnapshots(self.as_ptr(), &mut snaps, flags as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<DomainSnapshot> = Vec::new();
            for x in 0..size as isize {
                array.push(DomainSnapshot::new(*snaps.offset(x)));
            }
            libc::free(snaps as *mut libc::c_void);

            Ok(array)
        }
    }

    /// Get the cpu scheduler type for the domain
    pub fn get_scheduler_type(&self) -> Result<(String, i32), Error> {
        unsafe {
            let mut nparams: libc::c_int = -1;
            let sched_type = sys::virDomainGetSchedulerType(self.as_ptr(), &mut nparams);
            if sched_type.is_null() {
                return Err(Error::last_error());
            }

            Ok((c_chars_to_string!(sched_type), nparams))
        }
    }

    /// Get the scheduler parameters for the domain.
    pub fn get_scheduler_parameters(&self) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        unsafe {
            let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
            let ret =
                sys::virDomainGetSchedulerParameters(self.as_ptr(), &mut params[0], &mut nparams);
            if ret == -1 {
                return Err(Error::last_error());
            }
            params.set_len(nparams as usize);
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
        flags: sys::virDomainModificationImpact,
    ) -> Result<SchedulerInfo, Error> {
        let (sched_type, mut nparams) = self.get_scheduler_type()?;
        unsafe {
            let mut params: Vec<sys::virTypedParameter> = Vec::with_capacity(nparams as usize);
            let ret = sys::virDomainGetSchedulerParametersFlags(
                self.as_ptr(),
                &mut params[0],
                &mut nparams,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            params.set_len(nparams as usize);
            Ok(SchedulerInfo::from_vec(params, sched_type))
        }
    }

    /// Set the scheduler parameters for the domain.
    pub fn set_scheduler_parameters(&self, sched_info: &SchedulerInfo) -> Result<i32, Error> {
        unsafe {
            let mut params = sched_info.to_vec();
            let ret = sys::virDomainSetSchedulerParameters(
                self.as_ptr(),
                &mut params[0],
                params.len() as libc::c_int,
            );
            if ret == -1 {
                return Err(Error::last_error());
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
        flags: sys::virDomainModificationImpact,
    ) -> Result<i32, Error> {
        unsafe {
            let mut params = sched_info.to_vec();
            let ret = sys::virDomainSetSchedulerParametersFlags(
                self.as_ptr(),
                &mut params[0],
                params.len() as libc::c_int,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(ret)
        }
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
        unsafe {
            if sys::virDomainSendKey(
                self.as_ptr(),
                codeset as libc::c_uint,
                holdtime as libc::c_uint,
                keycodes as *mut libc::c_uint,
                nkeycodes as libc::c_int,
                flags as libc::c_uint,
            ) == -1
            {
                return Err(Error::last_error());
            }
            Ok(())
        }
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
        unsafe {
            let n = sys::virDomainScreenshot(
                self.as_ptr(),
                stream.as_ptr(),
                screen as libc::c_uint,
                flags as libc::c_uint,
            );
            if n.is_null() {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(n))
        }
    }

    /// Send an arbitrary monitor command cmd to domain through the QEMU monitor.
    ///
    /// * `cmd` - the QEMU monitor command string
    /// * `flags` - bitwise-or of supported virDomainQemuMonitorCommandFlags
    #[cfg(feature = "qemu")]
    pub fn qemu_monitor_command(&self, cmd: &str, flags: u32) -> Result<String, Error> {
        unsafe {
            let mut result: *mut libc::c_char = std::ptr::null_mut();
            let cmd_buf = CString::new(cmd).unwrap();
            let ret = sys::virDomainQemuMonitorCommand(
                self.as_ptr(),
                cmd_buf.as_ptr(),
                &mut result,
                flags as libc::c_uint,
            );
            if ret == -1 {
                return Err(Error::last_error());
            }
            Ok(c_chars_to_string!(result))
        }
    }
}
