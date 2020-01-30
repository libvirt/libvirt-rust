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

use virt::stream::Stream;
use virt::stream::VIR_STREAM_NONBLOCK;

#[test]
fn test_create_blocking() {
    let c = common::conn();
    let s = Stream::new(&c, 0).unwrap();
    drop(s);
    common::close(c);
}

#[test]
fn test_create_non_blocking() {
    let c = common::conn();
    let s = Stream::new(&c, VIR_STREAM_NONBLOCK).unwrap();
    drop(s);
    common::close(c);
}
