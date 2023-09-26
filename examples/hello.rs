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

//! This file contains trivial example code to connect to the running
//! hypervisor and gather a few bits of information about domains.
//! Similar API's exist for storage pools, networks, and interfaces.
//!
//! Largely inspired by hellolibvirt.c

use std::env;

use virt::connect::Connect;
use virt::error::Error;
use virt::sys;

fn show_hypervisor_info(conn: &Connect) -> Result<(), Error> {
    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let major = hv_ver / 1000000;
            hv_ver %= 1000000;
            let minor = hv_ver / 1000;
            let release = hv_ver % 1000;
            println!(
                "Hypervisor: '{}' version: {}.{}.{}",
                hv_type, major, minor, release
            );
            return Ok(());
        }
    }
    Err(Error::last_error())
}

fn show_domains(conn: &Connect) -> Result<(), Error> {
    let flags = sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE;

    if let Ok(num_active_domains) = conn.num_of_domains() {
        if let Ok(num_inactive_domains) = conn.num_of_defined_domains() {
            println!(
                "There are {} active and {} inactive domains",
                num_active_domains, num_inactive_domains
            );
            /* Return a list of all active and inactive domains. Using this API
             * instead of virConnectListDomains() and virConnectListDefinedDomains()
             * is preferred since it "solves" an inherit race between separated API
             * calls if domains are started or stopped between calls */
            if let Ok(doms) = conn.list_all_domains(flags) {
                for dom in doms {
                    let id = dom.get_id().unwrap_or(0);
                    let name = dom.get_name().unwrap_or_else(|_| String::from("no-name"));
                    let active = dom.is_active().unwrap_or(false);
                    println!("ID: {}, Name: {}, Active: {}", id, name, active);
                    if let Ok(dinfo) = dom.get_info() {
                        println!("Domain info:");
                        println!("    State: {}", dinfo.state);
                        println!("    Max Memory: {}", dinfo.max_mem);
                        println!("    Memory: {}", dinfo.memory);
                        println!("    CPUs: {}", dinfo.nr_virt_cpu);
                        println!("    CPU Time: {}", dinfo.cpu_time);
                    }
                    if let Ok(memtune) = dom.get_memory_parameters(0) {
                        println!("Memory tune:");
                        println!("    Hard Limit: {}", memtune.hard_limit.unwrap_or(0));
                        println!("    Soft Limit: {}", memtune.soft_limit.unwrap_or(0));
                        println!("    Min Guarantee: {}", memtune.min_guarantee.unwrap_or(0));
                        println!(
                            "    Swap Hard Limit: {}",
                            memtune.swap_hard_limit.unwrap_or(0)
                        );
                    }
                    if let Ok(numa) = dom.get_numa_parameters(0) {
                        println!("NUMA:");
                        println!("    Node Set: {}", numa.node_set.unwrap_or_default());
                        println!("    Mode: {}", numa.mode.unwrap_or(0));
                    }

                    if let Ok((sched_type, nparams)) = dom.get_scheduler_type() {
                        println!("SchedType: {}, nparams: {}", sched_type, nparams);
                    }

                    if let Ok(sched_info) = dom.get_scheduler_parameters() {
                        println!("Schedule Information:");
                        println!("\tScheduler\t: {}", sched_info.scheduler_type);
                        if let Some(shares) = sched_info.cpu_shares {
                            println!("\tcpu_shares\t: {}", shares);
                        }
                        if let Some(period) = sched_info.vcpu_bw.period {
                            println!("\tvcpu_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.vcpu_bw.quota {
                            println!("\tvcpu_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.emulator_bw.period {
                            println!("\temulator_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.emulator_bw.quota {
                            println!("\temulator_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.global_bw.period {
                            println!("\tglobal_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.global_bw.quota {
                            println!("\tglobal_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.global_bw.period {
                            println!("\tiothread_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.global_bw.quota {
                            println!("\tiothread_quota\t: {}", quota);
                        }
                    }
                }
            }
            return Ok(());
        }
    }
    Err(Error::last_error())
}

fn main() {
    let uri = env::args().nth(1);
    println!("Attempting to connect to hypervisor: '{:?}'", uri);

    let conn = match Connect::open(uri.as_deref()) {
        Ok(c) => c,
        Err(e) => panic!("No connection to hypervisor: {}", e),
    };

    match conn.get_uri() {
        Ok(u) => println!("Connected to hypervisor at '{}'", u),
        Err(e) => {
            disconnect(conn);
            panic!("Failed to get URI for hypervisor connection: {}", e);
        }
    };

    if let Err(e) = show_hypervisor_info(&conn) {
        disconnect(conn);
        panic!("Failed to show hypervisor info: {}", e);
    }

    if let Err(e) = show_domains(&conn) {
        disconnect(conn);
        panic!("Failed to show domains info: {}", e);
    }

    fn disconnect(mut conn: Connect) {
        if let Err(e) = conn.close() {
            panic!("Failed to disconnect from hypervisor: {}", e);
        }
        println!("Disconnected from hypervisor");
    }
}
