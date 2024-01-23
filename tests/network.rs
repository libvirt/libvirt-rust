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

mod common;

#[test]
fn test_create() {
    let c = common::conn();
    let n = common::build_network(&c, "wipes", false);
    assert_eq!(Ok(0), n.create());
    assert_eq!(Ok(String::from("libvirt-rs-test-wipes")), n.get_name());
    assert!(n.get_uuid().is_ok());
    assert!(!n.get_uuid_string().unwrap_or_default().is_empty());
    assert!(!n.get_xml_desc(0).unwrap_or_default().is_empty());
    common::clean_net(n);
    common::close(c);
}

#[test]
fn test_active() {
    let c = common::conn();
    let n = common::build_network(&c, "active", false);
    assert_eq!(Ok(false), n.is_active());
    assert_eq!(Ok(0), n.create());
    assert_eq!(Ok(true), n.is_active());
    common::clean_net(n);
    common::close(c);
}

#[test]
fn test_auto_start() {
    let c = common::conn();
    let n = common::build_network(&c, "autostart", false);
    assert_eq!(Ok(0), n.create());
    assert_eq!(Ok(false), n.get_autostart());
    assert_eq!(Ok(0), n.set_autostart(true));
    assert_eq!(Ok(true), n.get_autostart());
    common::clean_net(n);
    common::close(c);
}

#[test]
fn test_lookup_network_by_name() {
    let c = common::conn();
    let v = c.list_networks().unwrap_or_default();
    assert!(!v.is_empty(), "At least one network should exist");
    common::close(c);
}
