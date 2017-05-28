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

//! This file contains trivial example code to connect to the running
//! hypervisor and gather a few bits of information about domains.
//! Similar API's exist for storage pools, networks, and interfaces.
//!
//! Largely inspired by hellolibvirt.c

extern crate virt;

use std::env;

use virt::connect::Connect;
use virt::error::Error;


fn show_hypervisor_info(conn: &Connect) -> Result<(), Error> {
    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let major = hv_ver / 1000000;
            hv_ver %= 1000000;
            let minor = hv_ver / 1000;
            let release = hv_ver % 1000;
            println!("Hypervisor: '{}' version: {}.{}.{}",
                     hv_type,
                     major,
                     minor,
                     release);
            return Ok(());
        }
    }
    Err(Error::new())
}

fn show_domains(conn: &Connect) -> Result<(), Error> {
    let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE |
                virt::connect::VIR_CONNECT_LIST_DOMAINS_INACTIVE;

    if let Ok(num_active_domains) = conn.num_of_domains() {
        if let Ok(num_inactive_domains) = conn.num_of_defined_domains() {
            println!("There are {} active and {} inactive domains",
                     num_active_domains,
                     num_inactive_domains);
            /* Return a list of all active and inactive domains. Using this API
             * instead of virConnectListDomains() and virConnectListDefinedDomains()
             * is preferred since it "solves" an inherit race between separated API
             * calls if domains are started or stopped between calls */
            if let Ok(doms) = conn.list_all_domains(flags) {
                for dom in doms {
                    let id = dom.get_id().unwrap_or(0);
                    let name = dom.get_name().unwrap_or(String::from("no-name"));
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
                        println!("    Swap Hard Limit: {}",
                                 memtune.swap_hard_limit.unwrap_or(0));
                    }
                    if let Ok(numa) = dom.get_numa_parameters(0) {
                        println!("NUMA:");
                        println!("    Node Set: {}",
                                 numa.node_set.unwrap_or(String::from("")));
                        println!("    Mode: {}", numa.mode.unwrap_or(0));
                    }
                }
            }
            return Ok(());
        }
    }
    Err(Error::new())
}

fn main() {
    let uri = match env::args().nth(1) {
        Some(u) => u,
        None => String::from(""),
    };
    println!("Attempting to connect to hypervisor: '{}'", uri);

    let conn = match Connect::open(&uri) {
        Ok(c) => c,
        Err(e) => {
            panic!("No connection to hypervisor: code {}, message: {}",
                   e.code,
                   e.message)
        }
    };

    match conn.get_uri() {
        Ok(u) => println!("Connected to hypervisor at '{}'", u),
        Err(e) => {
            disconnect(conn);
            panic!("Failed to get URI for hypervisor connection: code {}, message: {}",
                   e.code,
                   e.message);
        }
    };

    if let Err(e) = show_hypervisor_info(&conn) {
        disconnect(conn);
        panic!("Failed to show hypervisor info: code {}, message: {}",
               e.code,
               e.message);
    }

    if let Err(e) = show_domains(&conn) {
        disconnect(conn);
        panic!("Failed to show domains info: code {}, message: {}",
               e.code,
               e.message);
    }

    fn disconnect(mut conn: Connect) {
        if let Err(e) = conn.close() {
            panic!("Failed to disconnect from hypervisor: code {}, message: {}",
                   e.code,
                   e.message);
        }
        println!("Disconnected from hypervisor");
    }
}
