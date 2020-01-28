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
use virt::storage_pool::StoragePool;

mod common;

#[test]
fn exercices() {
    match Connect::open("test:///default") {
        Ok(mut conn) => {
            let sp = conn.list_storage_pools().unwrap_or(vec![]);
            match StoragePool::lookup_by_name(&conn, &sp[0]) {
                Ok(storage_pool) => {
                    assert!(0 != storage_pool.get_name().unwrap_or(String::new()).len());
                    assert!(
                        0 != storage_pool
                            .get_uuid_string()
                            .unwrap_or(String::new())
                            .len()
                    );
                    assert!(0 != storage_pool.get_xml_desc(0).unwrap_or(String::new()).len());
                }
                Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
            }
            assert_eq!(0, conn.close().unwrap_or(-1));
        }
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
}

#[test]
fn test_lookup_storage_pool_by_name() {
    let c = common::conn();
    let v = c.list_storage_pools().unwrap_or(vec![]);
    assert!(0 < v.len(), "At least one storage_pool should exist");
    match StoragePool::lookup_by_name(&c, &v[0]) {
        Ok(mut s) => s.free().unwrap_or(()),
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
    common::close(c);
}
