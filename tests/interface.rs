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

mod common;

#[test]
fn test_create() {
    let c = common::conn();
    let n = common::build_interface(&c, "wipes");
    assert_eq!(Ok(0), n.create(0));
    assert_eq!(Ok(String::from("libvirt-rs-test-wipes")), n.get_name());
    assert!(!n.get_mac_string().unwrap_or_default().is_empty());
    assert!(!n.get_xml_desc(0).unwrap_or_default().is_empty());
    common::clean_iface(n);
    common::close(c);
}

#[test]
fn test_active() {
    let c = common::conn();
    let n = common::build_interface(&c, "active");
    assert_eq!(Ok(false), n.is_active());
    assert_eq!(Ok(0), n.create(0));
    assert_eq!(Ok(true), n.is_active());
    common::clean_iface(n);
    common::close(c);
}

#[test]
fn test_lookup_interface_by_name() {
    let c = common::conn();
    let v = c.list_interfaces().unwrap_or_default();
    assert!(!v.is_empty(), "At least one interface should exist");
    common::close(c);
}
