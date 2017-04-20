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
use virt::interface::Interface;

mod common;

#[test]
fn exercices() {
    match Connect::open("test:///default") {
        Ok(mut conn) => {
            let inters = conn.list_interfaces().unwrap_or(vec![]);
            let intid = &inters[0];
            match Interface::lookup_by_name(&conn, intid) {
                Ok(interface) => {
                    assert_eq!("eth1", interface.get_name().unwrap_or("n/a".to_string()));
                    assert_eq!(true, interface.is_active().unwrap_or(false));
                    assert_eq!("aa:bb:cc:dd:ee:ff",
                               interface.get_mac_string().unwrap_or("n/a".to_string()));
                    assert_eq!("<interface type='ethernet' name='eth1'>
  <start mode='onboot'/>
  <mtu size='1492'/>
  <protocol family='ipv4'>
    <ip address='192.168.0.5' prefix='24'/>
    <route gateway='192.168.0.1'/>
  </protocol>
  <mac address='aa:bb:cc:dd:ee:ff'/>
</interface>
",
                               interface.get_xml_desc(0).unwrap_or("n/a".to_string()));
                }
                Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
            }
            assert_eq!(0, conn.close().unwrap_or(-1));
        }
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
}

#[test]
fn defining() {
    match Connect::open("test:///default") {
        Ok(mut conn) => {
            let xml = "<interface type='ethernet' name='eth2'>
  <start mode='onboot'/>
  <mtu size='1492'/>
  <protocol family='ipv4'>
    <ip address='192.168.1.5' prefix='24'/>
    <route gateway='192.168.1.1'/>
  </protocol>
  <mac address='a1:bb:cc:dd:ee:ff'/>
</interface>";
            match Interface::define_xml(&conn, xml, 0) {
                Ok(interface) => {
                    assert_eq!(false, interface.is_active().unwrap_or(false));
                    interface.create(0).unwrap_or(());
                    assert_eq!(true, interface.is_active().unwrap_or(false));
                    assert_eq!("eth2", interface.get_name().unwrap_or("n/a".to_string()));
                    assert_eq!("a1:bb:cc:dd:ee:ff",
                               interface.get_mac_string().unwrap_or("n/a".to_string()));
                    let inters = conn.list_interfaces().unwrap_or(vec![]);
                    assert_eq!(2, inters.len());
                    interface.destroy().unwrap_or(());
                    interface.undefine().unwrap_or(());
                    let inters = conn.list_interfaces().unwrap_or(vec![]);
                    assert_eq!(1, inters.len());
                }
                Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
            }
            assert_eq!(0, conn.close().unwrap_or(-1));
        }
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
}

#[test]
fn test_lookup_interface_by_name() {
    let c = common::conn();
    let v = c.list_interfaces().unwrap_or(vec![]);
    assert!(0 < v.len(), "At least one interface should exist");
    match Interface::lookup_by_name(&c, &v[0]) {
        Ok(mut i) => i.free().unwrap_or(()),
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
    common::close(c);
}
