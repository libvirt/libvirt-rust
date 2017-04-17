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


fn conn() -> Connect {
    match Connect::open("test:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message),
        Ok(conn) =>
            conn
    }
}

#[test]
fn test_version() {
    let version = Connect::get_version().unwrap_or(0);
    assert!(version != 0, "Version was 0")
}

#[test]
fn test_connection() {
    match Connect::open("test:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message),
        Ok(conn) =>
            conn.close()
    }
}

#[test]
fn test_read_only_connection() {
    match Connect::open_read_only("test:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message),
        Ok(conn) =>
            conn.close()
    }
}

#[test]
#[should_panic]
fn test_connection_invalid() {
    match Connect::open("invalidtest:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message),
        Ok(conn) =>
            conn.close()
    }
}

#[test]
fn test_get_type() {
    let c = conn();
    assert_eq!("Test", c.get_type().unwrap_or(String::new()));
    c.close();
}

#[test]
fn test_get_uri() {
    let c = conn();
    assert_eq!("test:///default", c.get_uri().unwrap_or(String::new()));
    c.close();
}

#[test]
fn test_is_alive() {
    let c = conn();
    assert_eq!(true, c.is_alive().unwrap_or(false));
    c.close();
}

#[test]
fn test_is_encrypted() {
    let c = conn();
    assert_eq!(
        false, c.is_encrypted().unwrap_or(true),
        "Test driver should not be encrypted");
    c.close();
}

#[test]
fn test_is_secure() {
    let c = conn();
    assert_eq!(
        true, c.is_secure().unwrap_or(false),
        "Test driver should be secure");
    c.close();
}

#[test]
fn test_capabilities() {
    let c = conn();
    assert!(
        "" != c.get_capabilities().unwrap_or(String::new()),
        "Capabilities should not be empty");
    c.close();
}

#[test]
fn test_get_node_info() {
    let c = conn();
    match c.get_node_info() {
        Ok(info) => assert_eq!("i686", info.model),
        Err(_) => panic!("should have a node info")
    }
    c.close();
}

#[test]
fn test_hostname() {
    let c = conn();
    assert_eq!(
        "localhost.localdomain",
        c.get_hostname().unwrap_or(String::new()));
    c.close();
}

/*
#[test]
fn test_get_free_memory() {
    let c = conn();
    assert!(
        0 != c.get_free_memory().unwrap_or(0),
        "Version was 0");
    c.close();
}
*/

#[test]
fn test_lib_version() {
    let c = conn();
    assert!(
        0 != c.get_lib_version().unwrap_or(0),
        "Version was 0");
    c.close();
}

#[test]
fn test_list_domains() {
    let c = conn();
    assert!(
        0 < c.list_domains().unwrap_or(vec![]).len(),
        "At least one domain should exist");
    c.close();
}

#[test]
fn test_list_interfaces() {
    let c = conn();
    assert!(
        0 < c.list_interfaces().unwrap_or(vec![]).len(),
        "At least one interface should exist");
    c.close();
}

#[test]
fn test_list_networks() {
    let c = conn();
    assert!(
        0 < c.list_networks().unwrap_or(vec![]).len(),
        "At least one networks should exist");
    c.close();
}

#[test]
fn test_list_storage_pools() {
    let c = conn();
    assert!(
        0 < c.list_storage_pools().unwrap_or(vec![]).len(),
        "At least one storage pool should exist");
    c.close();
}


#[test]
fn test_list_all_domains() {
    let c = conn();
    let v = c.list_all_domains(0).unwrap_or(vec![]);
    assert!(0 < v.len(),
            "At least one domain should exist");
    assert_eq!("test", v[0].get_name().unwrap_or(String::new()));
    c.close();
}

#[test]
fn test_lookup_domain_by_id() {
    let c = conn();
    let v = c.list_domains().unwrap_or(vec![]);
    assert!(
        0 < v.len(),
        "At least one domain should exist");
    match c.domain_lookup_by_id(v[0]) {
        Ok(r) => r.free().unwrap_or(()),
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
    c.close();
}

#[test]
fn test_lookup_domain_by_name() {
    let c = conn();
    match c.domain_lookup_by_name("test") {
        Ok(r) => r.free().unwrap_or(()),
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
    c.close();
}

#[test]
fn test_lookup_network_by_name() {
    let c = conn();
    let v = c.list_networks().unwrap_or(vec![]);
    assert!(
        0 < v.len(),
        "At least one network should exist");
    match c.network_lookup_by_name(&v[0]) {
        Ok(r) => r.free().unwrap_or(()),
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
    c.close();
}

#[test]
fn test_lookup_interface_by_name() {
    let c = conn();
    let v = c.list_interfaces().unwrap_or(vec![]);
    assert!(
        0 < v.len(),
        "At least one interface should exist");
    match c.interface_lookup_by_name(&v[0]) {
        Ok(r) => r.free().unwrap_or(()),
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
    c.close();
}

#[test]
fn test_lookup_storage_pool_by_name() {
    let c = conn();
    let v = c.list_storage_pools().unwrap_or(vec![]);
    assert!(
        0 < v.len(),
        "At least one storage_pool should exist");
    match c.storage_pool_lookup_by_name(&v[0]) {
        Ok(r) => r.free().unwrap_or(()),
        Err(e) => panic!(
            "failed with code {}, message: {}", e.code, e.message)
    }
    c.close();
}

#[test]
fn test_get_cpu_models_names() {
    let c = conn();
    let mcpus = c.get_cpu_models_names("i686", 0).unwrap_or(vec![]);
    assert!(0 < mcpus.len(),
            "At least one cpu model should exist");
    c.close();
}

#[test]
fn test_get_max_vcpus() {
    let c = conn();
    let m = c.get_max_vcpus("").unwrap_or(0);
    assert!(0 < m, "At least one cpu should exist");
    c.close();
}
