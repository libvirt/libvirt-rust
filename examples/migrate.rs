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

//! An example of migrating domain

use std::env;

use virt::connect::Connect;
use virt::domain::Domain;
use virt::sys;

fn main() {
    if env::args().len() < 4 {
        panic!(
            "Usage: {} <src uri> <dst uri> <domain name>",
            env::args().next().unwrap()
        );
    }

    let src_uri = env::args().nth(1);
    let dst_uri = env::args().nth(2);
    let dname = env::args().nth(3).unwrap();

    println!(
        "Attempting to migrate domain '{}' from '{:?}' to '{:?}'...",
        dname, src_uri, dst_uri
    );

    let mut conn = match Connect::open(src_uri.as_deref()) {
        Ok(c) => c,
        Err(e) => panic!("No connection to source hypervisor: {}", e),
    };

    if let Ok(dom) = Domain::lookup_by_name(&conn, &dname) {
        let flags = sys::VIR_MIGRATE_LIVE | sys::VIR_MIGRATE_PEER2PEER | sys::VIR_MIGRATE_TUNNELLED;
        if dom
            .migrate(&conn, flags, None, dst_uri.as_deref(), 0)
            .is_ok()
        {
            println!("Domain migrated");
        }
    }

    if let Err(e) = conn.close() {
        panic!("Failed to disconnect from hypervisor: {}", e);
    }
    println!("Disconnected from source hypervisor");
}
