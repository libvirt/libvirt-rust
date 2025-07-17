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

use virt::connect::Connect;
use virt::storage_pool::StoragePool;

mod common;

#[test]
fn exercices() {
    match Connect::open(Some("test:///default")) {
        Ok(mut conn) => {
            let sp = conn.list_storage_pools().unwrap_or_default();
            assert!(!sp.is_empty(), "At least one storage_pool should exist");
            match StoragePool::lookup_by_name(&conn, &sp[0]) {
                Ok(storage_pool) => {
                    assert!(!storage_pool.get_name().unwrap_or_default().is_empty());
                    assert!(!storage_pool
                        .get_uuid_string()
                        .unwrap_or_default()
                        .is_empty());
                    assert!(storage_pool.get_uuid().is_ok());
                    assert!(!storage_pool.get_xml_desc(0).unwrap_or_default().is_empty());
                }
                Err(e) => panic!("{e}"),
            }
            assert_eq!(0, conn.close().unwrap_or(-1));
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_lookup_storage_pool_by_name() {
    let c = common::conn();
    let v = c.list_storage_pools().unwrap_or_default();
    assert!(!v.is_empty(), "At least one storage_pool should exist");
    match StoragePool::lookup_by_name(&c, &v[0]) {
        Ok(mut s) => s.free().unwrap_or(()),
        Err(e) => panic!("{e}"),
    }
    common::close(c);
}
