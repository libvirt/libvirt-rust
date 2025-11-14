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

// The rustc is complaining about dead code because only used when
// ignored tests are executed.
#![allow(dead_code)]

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;
use virt::interface::Interface;
use virt::network::Network;
use virt::storage_pool::StoragePool;
use virt::storage_vol::StorageVol;

pub fn conn() -> Connect {
    Connect::open(Some("test:///default")).unwrap()
}

pub fn qemu_conn() -> Connect {
    Connect::open(Some("qemu:///system")).unwrap()
}

pub fn close(conn: Connect) {
    drop(conn);
}

pub fn clean_dom(dom: Domain) {
    let _ = dom.destroy();
    let _ = dom.undefine();
    drop(dom);
}

pub fn clean_iface(iface: Interface) {
    let _ = iface.destroy(0);
    let _ = iface.undefine();
    drop(iface);
}

pub fn clean_pool(pool: StoragePool) {
    let _ = pool.destroy();
    let _ = pool.undefine();
    drop(pool);
}

pub fn clean_net(net: Network) {
    let _ = net.destroy();
    let _ = net.undefine();
    drop(net);
}

pub fn clean_vol(vol: StorageVol) {
    let _ = vol.delete(0);
    drop(vol);
}

pub fn build_qemu_domain(conn: &Connect, name: &str, transient: bool) -> Domain {
    let name = format!("libvirt-rs-test-{name}");

    if let Ok(dom) = conn.lookup_domain_by_name(&name) {
        clean_dom(dom);
    }

    let xml = format!(
        "<domain type=\"qemu\">
		         <name>{name}</name>
                         <memory unit=\"KiB\">128</memory>
                         <features>
                           <acpi/>
                           <apic/>
                         </features>
                         <os>
                           <type>hvm</type>
                         </os>
                       </domain>"
    );

    let result: Result<Domain, Error> = if transient {
        conn.create_domain_xml(&xml, 0)
    } else {
        conn.define_domain_xml(&xml)
    };

    result.unwrap()
}

pub fn build_test_domain(conn: &Connect, name: &str, transient: bool) -> Domain {
    let name = format!("libvirt-rs-test-{name}");

    if let Ok(dom) = conn.lookup_domain_by_name(&name) {
        clean_dom(dom);
    }

    let xml = format!(
        "<domain type=\"test\">
		         <name>{name}</name>
                         <memory unit=\"KiB\">128</memory>
                         <features>
                           <acpi/>
                           <apic/>
                         </features>
                         <os>
                           <type>hvm</type>
                         </os>
                       </domain>"
    );

    let result: Result<Domain, Error> = if transient {
        conn.create_domain_xml(&xml, 0)
    } else {
        conn.define_domain_xml(&xml)
    };

    result.unwrap()
}

pub fn build_storage_pool(conn: &Connect, name: &str, transient: bool) -> StoragePool {
    let name = format!("libvirt-rs-test-{name}");

    if let Ok(pool) = StoragePool::lookup_by_name(conn, &name) {
        clean_pool(pool);
    }

    let xml = format!(
        "<pool type='dir'>
                          <name>{name}</name>
                            <target>
                              <path>/var/lib/libvirt/images</path>
                            </target>
                          </pool>"
    );

    let result: Result<StoragePool, Error> = if transient {
        StoragePool::create_xml(conn, &xml, 0)
    } else {
        StoragePool::define_xml(conn, &xml, 0)
    };

    result.unwrap()
}

pub fn build_storage_vol(pool: &StoragePool, name: &str, size: u64) -> StorageVol {
    if let Ok(vol) = StorageVol::lookup_by_name(pool, name) {
        return vol;
    }

    let xml = format!(
        "<volume type='file'>
                         <name>{name}</name>
                         <allocation unit='Kib'>{size}</allocation>
                         <capacity unit='Kib'>{size}</capacity>
                       </volume>"
    );
    StorageVol::create_xml(pool, &xml, 0).unwrap()
}

pub fn build_network(conn: &Connect, name: &str, transient: bool) -> Network {
    let name = format!("libvirt-rs-test-{name}");

    if let Ok(net) = Network::lookup_by_name(conn, &name) {
        clean_net(net);
    }

    let xml = format!(
        "<network>
                         <name>{name}</name>
                         <bridge name='testbr0'/>
                         <forward/>
                         <ip address='192.168.0.1' netmask='255.255.255.0'></ip>
                       </network>"
    );

    let result: Result<Network, Error> = if transient {
        Network::create_xml(conn, &xml)
    } else {
        Network::define_xml(conn, &xml)
    };

    result.unwrap()
}

pub fn build_interface(conn: &Connect, name: &str) -> Interface {
    let name = format!("libvirt-rs-test-{name}");

    if let Ok(iface) = conn.lookup_interface_by_name(&name) {
        clean_iface(iface);
    }

    let xml = format!(
        "<interface type='ethernet' name='{name}'>
                         <mac address='aa:bb:cc:dd:ee:ff'/>
                       </interface>"
    );

    conn.define_interface_xml(&xml, 0).unwrap()
}
