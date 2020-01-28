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

//! This file contains trivial example code to suspend/resume domains
//!
//! Largely inspired by libvirt/examples/suspend.c

extern crate virt;

use std::{env, thread, time};

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;

fn suspend_and_resume(conn: &Connect, name: &str, sec: u64) -> Result<(), Error> {
    if let Ok(dom) = Domain::lookup_by_name(conn, name) {
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' suspended, info: {:?}", name, dom.get_info());
            thread::sleep(time::Duration::from_millis(sec * 1000));

            if dom.resume().is_ok() {
                println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
                return Ok(());
            }
        }
    }
    Err(Error::new())
}

fn fetch_domains(conn: &Connect) -> Result<(), Error> {
    let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE;
    if let Ok(doms) = conn.list_all_domains(flags) {
        println!("Running domains:");
        println!("----------------");
        for dom in doms {
            println!("{}", dom.get_name().unwrap_or(String::from("no-name")));
        }
        return Ok(());
    }
    Err(Error::new())
}

fn main() {
    let uri = match env::args().nth(1) {
        Some(u) => u,
        None => String::from(""),
    };
    let name = match env::args().nth(2) {
        Some(n) => n,
        None => String::from(""),
    };

    println!("Attempting to connect to hypervisor: '{}'", uri);
    let mut conn = match Connect::open(&uri) {
        Ok(c) => c,
        Err(e) => panic!(
            "No connection to hypervisor: code {}, message: {}",
            e.code, e.message
        ),
    };

    if name.len() == 0 {
        if let Err(e) = fetch_domains(&conn) {
            println!(
                "Failed to fetch domains. code {}, message: {}",
                e.code, e.message
            );
        }
    } else {
        if let Err(e) = suspend_and_resume(&conn, &name, 1) {
            println!(
                "Failed to suspend/resume. code {}, message: {}",
                e.code, e.message
            );
        }
    }

    if let Err(e) = conn.close() {
        panic!(
            "Failed to disconnect from hypervisor: code {}, message: {}",
            e.code, e.message
        );
    }
}
