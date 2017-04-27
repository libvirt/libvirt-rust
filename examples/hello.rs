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


fn show_hypervisor_info(conn: Connect) -> Result<(), Error> {
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

    if let Err(e) = show_hypervisor_info(conn.clone()) {
        disconnect(conn);
        panic!("Failed to show hypervisor info: code {}, message: {}",
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
