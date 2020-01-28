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

//! Integration tests using a real qemu:///system connection they are
//! all ignored by default.

extern crate virt;

mod common;

use virt::connect::{Connect, ConnectAuth, ConnectCredential};

#[test]
#[ignore]
fn test_create_domain_with_flags() {
    let c = common::qemu_conn();
    let d = common::build_qemu_domain(&c, "create", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(
        Ok((::virt::domain::VIR_DOMAIN_START_PAUSED, 1)),
        d.get_state()
    );
    assert_eq!(Ok(String::from("libvirt-rs-test-create")), d.get_name());
    common::clean(d);
    common::close(c);
}

#[test]
#[ignore]
fn test_create_storage_pool_and_vols() {
    let c = common::qemu_conn();
    let p = common::build_storage_pool(&c, "create", false);
    assert_eq!(Ok(0), p.create(0));
    assert_eq!(Ok(String::from("libvirt-rs-test-create")), p.get_name());
    let v = common::build_storage_vol(&p, "vol1", 8);
    assert_eq!(Ok(String::from("vol1")), v.get_name());
    assert_eq!(
        Ok(String::from("/var/lib/libvirt/images/vol1")),
        v.get_path()
    );
    assert_eq!(
        Ok(String::from("/var/lib/libvirt/images/vol1")),
        v.get_key()
    );
    if let Ok(info) = v.get_info() {
        assert_eq!(0, info.kind);
        assert_eq!(8192, info.allocation);
        assert_eq!(8192, info.capacity);
    } else {
        common::clean_vol(v);
        common::clean_pool(p);
        common::close(c);
        panic!("should not be here")
    }
    assert_eq!(Ok(0), v.resize(10240, 0));
    if let Ok(info) = v.get_info() {
        assert_eq!(0, info.kind);
        assert_eq!(8192, info.allocation);
        assert_eq!(10240, info.capacity);
    } else {
        common::clean_vol(v);
        common::clean_pool(p);
        common::close(c);
        panic!("should not be here")
    }
    if let Ok(info) = p.get_info() {
        assert_eq!(2, info.state);
        assert_eq!(0, info.capacity - (info.allocation + info.available));
    } else {
        common::clean_vol(v);
        common::clean_pool(p);
        common::close(c);
        panic!("should not be here")
    }
    common::clean_vol(v);
    common::clean_pool(p);
    common::close(c);
}

#[test]
#[ignore]
fn test_connection_with_auth() {
    fn callback(creds: &mut Vec<ConnectCredential>) {
        for cred in creds {
            match cred.typed {
                ::virt::connect::VIR_CRED_AUTHNAME => {
                    cred.result = Some(String::from("user"));
                }
                ::virt::connect::VIR_CRED_PASSPHRASE => {
                    cred.result = Some(String::from("pass"));
                }
                _ => {
                    panic!("Should not be here...");
                }
            }
        }
    };

    let mut auth = ConnectAuth::new(
        vec![
            ::virt::connect::VIR_CRED_AUTHNAME,
            ::virt::connect::VIR_CRED_PASSPHRASE,
        ],
        callback,
    );
    let c = Connect::open_auth("test+tcp://127.0.0.1/default", &mut auth, 0);
    assert_eq!(true, c.is_ok());
    common::close(c.unwrap());
}

#[test]
#[ignore]
fn test_connection_with_auth_wrong() {
    fn callback(creds: &mut Vec<ConnectCredential>) {
        for cred in creds {
            match cred.typed {
                ::virt::connect::VIR_CRED_AUTHNAME => {
                    cred.result = Some(String::from("user"));
                }
                ::virt::connect::VIR_CRED_PASSPHRASE => {
                    cred.result = Some(String::from("passwrong"));
                }
                _ => {
                    panic!("Should not be here...");
                }
            }
        }
    };

    let mut auth = ConnectAuth::new(
        vec![
            ::virt::connect::VIR_CRED_AUTHNAME,
            ::virt::connect::VIR_CRED_PASSPHRASE,
        ],
        callback,
    );
    let c = Connect::open_auth("test+tcp://127.0.0.1/default", &mut auth, 0);
    assert_eq!(false, c.is_ok());
}

#[test]
#[ignore]
fn test_reset() {
    let c = common::qemu_conn();
    let d = common::build_qemu_domain(&c, "reset", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((::virt::domain::VIR_DOMAIN_RUNNING, 1)), d.get_state());
    assert_eq!(Ok(0), d.reset());
    // TODO assert something showing reset has the intended side effect
    common::clean(d);
    common::close(c);
}
