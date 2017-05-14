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

extern crate virt;

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;


pub fn conn() -> Connect {
    match Connect::open("test:///default") {
        Err(e) => {
            panic!("Build connection failed with code {}, message: {}",
                   e.code,
                   e.message)
        }
        Ok(conn) => conn,
    }
}

pub fn qemu_conn() -> Connect {
    match Connect::open("qemu:///system") {
        Err(e) => {
            panic!("Build connection failed with code {}, message: {}",
                   e.code,
                   e.message)
        }
        Ok(conn) => conn,
    }
}

pub fn close(mut conn: Connect) {
    assert_eq!(Ok(0), conn.close(), "close(), expected 0")
}

pub fn clean(mut dom: Domain) {
    dom.destroy();
    dom.undefine();
    dom.free();
}

pub fn build_domain(conn: &Connect, name: &str, transient: bool) -> Domain {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        clean(dom);
    }

    let xml = format!("<domain type=\"qemu\">
		         <name>{}</name>
                         <memory unit=\"KiB\">128</memory>
                         <features>
                           <acpi/>
                           <apic/>
                         </features>
                         <os>
                           <type>hvm</type>
                         </os>
                       </domain>", name);

    let result: Result<Domain, Error>;
    if transient {
        result = Domain::create_xml(&conn, &xml, 0);
    } else {
        result = Domain::define_xml(&conn, &xml);
    }

    match result {
        Ok(dom) => dom,
        Err(e) => {
            panic!("Build domain failed with code {}, message: {}",
                   e.code,
                   e.message)
        }
    }
}
