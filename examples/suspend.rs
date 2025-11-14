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

//! This file contains trivial example code to suspend/resume domains
//!
//! Largely inspired by libvirt/examples/suspend.c

use std::{env, thread, time};

use virt::connect::Connect;
use virt::error::{clear_error_callback, Error};
use virt::sys;

fn suspend_and_resume(conn: &Connect, name: &str, sec: u64) -> Result<(), Error> {
    if let Ok(dom) = conn.lookup_domain_by_name(name) {
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' suspended, info: {:?}", name, dom.get_info());
            thread::sleep(time::Duration::from_millis(sec * 1000));

            if dom.resume().is_ok() {
                println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
                return Ok(());
            }
        }
    }
    Err(Error::last_error())
}

fn fetch_domains(conn: &Connect) -> Result<(), Error> {
    let flags = sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE;
    if let Ok(doms) = conn.list_all_domains(flags) {
        println!("Running domains:");
        println!("----------------");
        for dom in doms {
            println!(
                "{}",
                dom.get_name().unwrap_or_else(|_| String::from("no-name"))
            );
        }
        return Ok(());
    }
    Err(Error::last_error())
}

fn main() {
    clear_error_callback();

    let uri = env::args().nth(1);
    let name = env::args().nth(2).unwrap_or_default();

    println!("Attempting to connect to hypervisor: '{uri:?}'");
    let conn = match Connect::open(uri.as_deref()) {
        Ok(c) => c,
        Err(e) => panic!("No connection to hypervisor: {e}"),
    };

    if name.is_empty() {
        if let Err(e) = fetch_domains(&conn) {
            println!("Failed to fetch domains: {e}");
        }
    } else if let Err(e) = suspend_and_resume(&conn, &name, 1) {
        println!("Failed to suspend/resume: {e}");
    }

    drop(conn);
}
