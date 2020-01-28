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

// The rustc is complaining about dead code because only used when
// ignored tests are executed.
#![allow(dead_code)]

extern crate virt;

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;
use virt::interface::Interface;
use virt::network::Network;
use virt::storage_pool::StoragePool;
use virt::storage_vol::StorageVol;

pub fn conn() -> Connect {
    match Connect::open("test:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message
        ),
        Ok(conn) => conn,
    }
}

pub fn qemu_conn() -> Connect {
    match Connect::open("qemu:///system") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message
        ),
        Ok(conn) => conn,
    }
}

pub fn close(mut conn: Connect) {
    assert_eq!(Ok(0), conn.close(), "close(), expected 0")
}

pub fn clean(mut dom: Domain) {
    if let Err(_) = dom.destroy() {}
    if let Err(_) = dom.undefine() {}
    assert_eq!(Ok(()), dom.free())
}

pub fn clean_iface(mut iface: Interface) {
    if let Err(_) = iface.destroy() {}
    if let Err(_) = iface.undefine() {}
    assert_eq!(Ok(()), iface.free())
}

pub fn clean_pool(mut pool: StoragePool) {
    if let Err(_) = pool.destroy() {}
    if let Err(_) = pool.undefine() {}
    assert_eq!(Ok(()), pool.free())
}

pub fn clean_net(mut net: Network) {
    if let Err(_) = net.destroy() {}
    if let Err(_) = net.undefine() {}
    assert_eq!(Ok(()), net.free())
}

pub fn clean_vol(mut vol: StorageVol) {
    if let Err(_) = vol.delete(0) {}
    assert_eq!(Ok(()), vol.free())
}

pub fn build_qemu_domain(conn: &Connect, name: &str, transient: bool) -> Domain {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        clean(dom);
    }

    let xml = format!(
        "<domain type=\"qemu\">
		         <name>{}</name>
                         <memory unit=\"KiB\">128</memory>
                         <features>
                           <acpi/>
                           <apic/>
                         </features>
                         <os>
                           <type>hvm</type>
                         </os>
                       </domain>",
        name
    );

    let result: Result<Domain, Error>;
    if transient {
        result = Domain::create_xml(&conn, &xml, 0);
    } else {
        result = Domain::define_xml(&conn, &xml);
    }

    match result {
        Ok(dom) => dom,
        Err(e) => panic!(
            "Build domain failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}

pub fn build_test_domain(conn: &Connect, name: &str, transient: bool) -> Domain {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        clean(dom);
    }

    let xml = format!(
        "<domain type=\"test\">
		         <name>{}</name>
                         <memory unit=\"KiB\">128</memory>
                         <features>
                           <acpi/>
                           <apic/>
                         </features>
                         <os>
                           <type>hvm</type>
                         </os>
                       </domain>",
        name
    );

    let result: Result<Domain, Error>;
    if transient {
        result = Domain::create_xml(&conn, &xml, 0);
    } else {
        result = Domain::define_xml(&conn, &xml);
    }

    match result {
        Ok(dom) => dom,
        Err(e) => panic!(
            "Build domain failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}

pub fn build_storage_pool(conn: &Connect, name: &str, transient: bool) -> StoragePool {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(pool) = StoragePool::lookup_by_name(&conn, &name) {
        clean_pool(pool);
    }

    let xml = format!(
        "<pool type='dir'>
                          <name>{}</name>
                            <target>
                              <path>/var/lib/libvirt/images</path>
                            </target>
                          </pool>",
        name
    );

    let result: Result<StoragePool, Error>;
    if transient {
        result = StoragePool::create_xml(&conn, &xml, 0);
    } else {
        result = StoragePool::define_xml(&conn, &xml, 0);
    }

    match result {
        Ok(pool) => pool,
        Err(e) => panic!(
            "Build storage pool failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}

pub fn build_storage_vol(pool: &StoragePool, name: &str, size: u64) -> StorageVol {
    if let Ok(vol) = StorageVol::lookup_by_name(&pool, name) {
        return vol;
    }

    let xml = format!(
        "<volume type='file'>
                         <name>{}</name>
                         <allocation unit='Kib'>{}</allocation>
                         <capacity unit='Kib'>{}</capacity>
                       </volume>",
        name, size, size
    );
    match StorageVol::create_xml(&pool, &xml, 0) {
        Ok(vol) => vol,
        Err(e) => panic!(
            "Build vol failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}

pub fn build_network(conn: &Connect, name: &str, transient: bool) -> Network {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(net) = Network::lookup_by_name(&conn, &name) {
        clean_net(net);
    }

    let xml = format!(
        "<network>
                         <name>{}</name>
                         <bridge name='testbr0'/>
                         <forward/>
                         <ip address='192.168.0.1' netmask='255.255.255.0'></ip>
                       </network>",
        name
    );

    let result: Result<Network, Error>;
    if transient {
        result = Network::create_xml(&conn, &xml, 0);
    } else {
        result = Network::define_xml(&conn, &xml);
    }

    match result {
        Ok(net) => net,
        Err(e) => panic!(
            "Build storage pool failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}

pub fn build_interface(conn: &Connect, name: &str) -> Interface {
    let name = format!("libvirt-rs-test-{}", name);

    if let Ok(iface) = Interface::lookup_by_name(&conn, &name) {
        clean_iface(iface);
    }

    let xml = format!(
        "<interface type='ethernet' name='{}'>
                         <mac address='aa:bb:cc:dd:ee:ff'/>
                       </interface>",
        name
    );

    let result = Interface::define_xml(&conn, &xml, 0);
    match result {
        Ok(iface) => iface,
        Err(e) => panic!(
            "Build storage pool failed with code {}, message: {}",
            e.code, e.message
        ),
    }
}
