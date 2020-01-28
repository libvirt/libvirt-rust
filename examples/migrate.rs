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

//! An example of migrating domain

extern crate virt;

use std::env;

use virt::connect::Connect;
use virt::domain::Domain;

fn main() {
    if env::args().len() < 4 {
        panic!(
            "Usage: {} <src uri> <dst uri> <domain name>",
            env::args().nth(0).unwrap()
        );
    }

    let src_uri = env::args().nth(1).unwrap();
    let dst_uri = env::args().nth(2).unwrap();
    let dname = env::args().nth(3).unwrap();

    println!(
        "Attempting to migrate domain '{}' from '{}' to '{}'...",
        dname, src_uri, dst_uri
    );

    let mut conn = match Connect::open(&src_uri) {
        Ok(c) => c,
        Err(e) => panic!(
            "No connection to source hypervisor: code {}, message: {}",
            e.code, e.message
        ),
    };

    if let Ok(dom) = Domain::lookup_by_name(&conn, &dname) {
        let flags = ::virt::domain::VIR_MIGRATE_LIVE
            | ::virt::domain::VIR_MIGRATE_PEER2PEER
            | ::virt::domain::VIR_MIGRATE_TUNNELLED;
        if let Ok(_) = dom.migrate(&conn, flags, &dst_uri, 0) {
            println!("Domain migrated");
        }
    }

    if let Err(e) = conn.close() {
        panic!(
            "Failed to disconnect from hypervisor: code {}, message: {}",
            e.code, e.message
        );
    }
    println!("Disconnected from source hypervisor");
}
