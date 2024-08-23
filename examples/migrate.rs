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
use virt::domain::MigrateParameters;
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

    let mut dconn = match Connect::open(dst_uri.as_deref()) {
        Ok(c) => c,
        Err(e) => panic!("No connection to destination hypervisor: {}", e),
    };

    if let Ok(dom) = Domain::lookup_by_name(&conn, &dname) {
        let flags = sys::VIR_MIGRATE_LIVE;
        let migrate_parameters = MigrateParameters {
            dest_name: Some(dname.clone()),
            ..Default::default()
        };
        if let Ok(new_dom) = dom.migrate3(&dconn, migrate_parameters, flags) {
            if let Ok(job_stats) = new_dom.get_job_stats(sys::VIR_DOMAIN_JOB_STATS_COMPLETED) {
                println!(
                    "Migration completed in {}ms",
                    job_stats
                        .time_elapsed
                        .map(|time| time.to_string())
                        .unwrap_or("?".into())
                );
            }
        }
    }

    if let Err(e) = conn.close() {
        panic!("Failed to disconnect from source hypervisor: {}", e);
    }

    if let Err(e) = dconn.close() {
        panic!("Failed to disconnect from destination hypervisor: {}", e);
    }
    println!("Disconnected from source and destination hypervisors");
}
