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
use virt::domain::VIR_DOMAIN_XML_SECURE;


#[test]
fn exercices() {
    match Connect::open("test:///default") {
        Ok(conn) => {
            let doms = conn.list_domains().unwrap_or(vec![]);
            assert_eq!(1, doms.len());

            let domid = doms[0];
            match conn.domain_lookup_by_id(domid) {
                Ok(domain) => {
                    assert_eq!("test", domain.get_name().unwrap_or("noname".to_string()));
                    assert_eq!("6695eb01-f6a4-8304-79aa-97f2502e193f",
                               domain.get_uuid_string().unwrap_or("nouid".to_string()));
                    assert_eq!(1, domain.get_id().unwrap_or(0));
                    assert_eq!("<domain type='test' id='1'>
  <name>test</name>
  <uuid>6695eb01-f6a4-8304-79aa-97f2502e193f</uuid>
  <memory unit='KiB'>8388608</memory>
  <currentMemory unit='KiB'>2097152</currentMemory>
  <vcpu placement='static'>2</vcpu>
  <os>
    <type arch='i686'>hvm</type>
    <boot dev='hd'/>
  </os>
  <clock offset='utc'/>
  <on_poweroff>destroy</on_poweroff>
  <on_reboot>restart</on_reboot>
  <on_crash>destroy</on_crash>
  <devices>
  </devices>
</domain>
", domain.get_xml_desc(VIR_DOMAIN_XML_SECURE).unwrap_or("noname".to_string()));
                }
                Err(e) => panic!(
                    "failed with code {}, message: {}", e.code, e.message)
            }
            conn.close();
        },
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
}
