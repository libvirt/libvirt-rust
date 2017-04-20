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


#[test]
fn exercices() {
    match Connect::open("test:///default") {
        Ok(mut conn) => {
            let nets = conn.list_networks().unwrap_or(vec![]);
            match conn.network_lookup_by_name(&nets[0]) {
                Ok(network) => {
                    assert_eq!("default", network.get_name().unwrap_or("n/a".to_string()));
                    assert_eq!("dd8fe884-6c02-601e-7551-cca97df1c5df",
                               network.get_uuid_string().unwrap_or("n/a".to_string()));
                    assert_eq!("<network>
  <name>default</name>
  <uuid>dd8fe884-6c02-601e-7551-cca97df1c5df</uuid>
  <forward mode='nat'/>
  <bridge name='virbr0' stp='on' delay='0'/>
  <ip address='192.168.122.1' netmask='255.255.255.0'>
    <dhcp>
      <range start='192.168.122.2' end='192.168.122.254'/>
    </dhcp>
  </ip>
</network>
", network.get_xml_desc(0).unwrap_or("n/a".to_string()));
                }
                Err(e) => panic!(
                    "failed with code {}, message: {}", e.code, e.message)
            }
            assert_eq!(0, conn.close().unwrap_or(-1));
        },
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
}
