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


pub fn conn() -> Connect {
    match Connect::open("test:///default") {
        Err(e) => {
            panic!("Build connection failed with code {}, message: {}",
                   e.code,
                   e.message)
        }
        Ok(conn) => conn,
    }
}

pub fn close(mut conn: Connect) {
    assert_eq!(Ok(0), conn.close(), "close(), expected 0")
}
