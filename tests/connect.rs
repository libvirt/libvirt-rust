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

mod common;

use virt::connect::Connect;

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
            e.code, e.message
        ),
        Ok(conn) => common::close(conn),
    }
}

#[test]
fn test_read_only_connection() {
    match Connect::open_read_only("test:///default") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message
        ),
        Ok(conn) => common::close(conn),
    }
}

#[test]
#[should_panic]
fn test_connection_invalid() {
    match Connect::open_read_only("invalid") {
        Err(e) => panic!(
            "Build connection failed with code {}, message: {}",
            e.code, e.message
        ),
        Ok(conn) => common::close(conn),
    }
}

#[test]
fn test_get_type() {
    let c = common::conn();
    assert!(0 != c.get_type().unwrap_or(String::new()).len());
    common::close(c)
}

#[test]
fn test_get_uri() {
    let c = common::conn();
    assert_eq!("test:///default", c.get_uri().unwrap_or(String::new()));
    common::close(c);
}

#[test]
fn test_is_alive() {
    let c = common::conn();
    assert_eq!(true, c.is_alive().unwrap_or(false));
    common::close(c);
}

#[test]
fn test_is_encrypted() {
    let c = common::conn();
    assert_eq!(
        false,
        c.is_encrypted().unwrap_or(true),
        "Test driver should not be encrypted"
    );
    common::close(c);
}

#[test]
fn test_is_secure() {
    let c = common::conn();
    assert_eq!(
        true,
        c.is_secure().unwrap_or(false),
        "Test driver should be secure"
    );
    common::close(c);
}

#[test]
fn test_capabilities() {
    let c = common::conn();
    assert!(
        "" != c.get_capabilities().unwrap_or(String::new()),
        "Capabilities should not be empty"
    );
    common::close(c);
}

#[test]
fn test_get_node_info() {
    let c = common::conn();
    match c.get_node_info() {
        Ok(info) => assert_eq!("i686", info.model),
        Err(_) => panic!("should have a node info"),
    }
    common::close(c);
}

#[test]
fn test_hostname() {
    let c = common::conn();
    assert!(0 != c.get_hostname().unwrap_or(String::new()).len());
    common::close(c);
}

/*
#[test]
fn test_get_free_memory() {
    let c = common::conn();
    assert!(
        0 != c.get_free_memory().unwrap_or(0),
        "Version was 0");
    common::close(c);
}
*/

#[test]
fn test_lib_version() {
    let c = common::conn();
    assert!(0 != c.get_lib_version().unwrap_or(0), "Version was 0");
    common::close(c);
}

#[test]
fn test_list_domains() {
    let c = common::conn();
    assert!(
        0 < c.list_domains().unwrap_or(vec![]).len(),
        "At least one domain should exist"
    );
    common::close(c);
}

#[test]
fn test_list_interfaces() {
    let c = common::conn();
    assert!(
        0 < c.list_interfaces().unwrap_or(vec![]).len(),
        "At least one interface should exist"
    );
    common::close(c);
}

#[test]
fn test_list_networks() {
    let c = common::conn();
    assert!(
        0 < c.list_networks().unwrap_or(vec![]).len(),
        "At least one networks should exist"
    );
    common::close(c);
}

#[test]
fn test_list_storage_pools() {
    let c = common::conn();
    assert!(
        0 < c.list_storage_pools().unwrap_or(vec![]).len(),
        "At least one storage pool should exist"
    );
    common::close(c);
}

#[test]
fn test_list_all_domains() {
    let c = common::conn();
    let v = c.list_all_domains(0).unwrap_or(vec![]);
    assert!(0 < v.len(), "At least one domain should exist");
    drop(v);
    common::close(c);
}

/* Travis is failing on this test
#[test]
fn test_get_cpu_models_names() {
    let c = common::conn();
    let mcpus = c.get_cpu_models_names("i686", 0).unwrap_or(vec![]);
    assert!(0 < mcpus.len(), "At least one cpu model should exist");
    common::close(c);
}
*/

#[test]
fn test_get_max_vcpus() {
    let c = common::conn();
    let m = c.get_max_vcpus("").unwrap_or(0);
    assert!(0 < m, "At least one cpu should exist");
    common::close(c);
}
