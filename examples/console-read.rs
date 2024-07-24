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
 * Ryosuke Yasuoka <ryasuoka@redhat.com>
 */

//! This file contains trivial example code to output console log. This file
//! connects to the running VM and waits for logs as a console client.
//! 1st arg is URI, 2nd arg is the name of VM, and 3rd arg is a console name.
//! If the console name is omitted, then the first console will be opened.
//!
//! Examples
//! ```
//! $ cargo run --example console-read -- 'qemu:///system' 'mytestvm' 'serial0'
//! ```
//!
//! Largely inspired by libvirt-python/examples/consolecallback.py

use std::env;
use virt::{
    connect::Connect,
    domain::Domain,
    stream::Stream,
    sys::{
        virEventRegisterDefaultImpl, virEventRunDefaultImpl, virStreamEventType,
        VIR_DOMAIN_CONSOLE_FORCE, VIR_STREAM_EVENT_READABLE, VIR_STREAM_NONBLOCK,
    },
};

fn read_callback(stream: &Stream, event_type: virStreamEventType) {
    if event_type == VIR_STREAM_EVENT_READABLE {
        let mut buf = vec![0; 1024];
        match stream.recv(buf.as_mut_slice()) {
            Ok(t) => {
                if let Ok(received_data) = std::str::from_utf8(&buf[..t]) {
                    print!("{}", received_data);
                } else {
                    eprint!("Invalid UTF-8 sequence received.");
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn event_register_default_impl() {
    unsafe { virEventRegisterDefaultImpl() };
}

fn event_run_default_impl() {
    unsafe { virEventRunDefaultImpl() };
}

fn main() {
    event_register_default_impl();

    let uri = env::args().nth(1);
    let name = env::args().nth(2).unwrap();
    let dev_name = env::args().nth(3);

    let conn = Connect::open(uri.as_deref()).unwrap();
    let dom = Domain::lookup_by_name(&conn, &name).unwrap();
    let mut st = Stream::new(&conn, VIR_STREAM_NONBLOCK).unwrap();
    dom.open_console(dev_name.as_deref(), &st, VIR_DOMAIN_CONSOLE_FORCE)
        .unwrap();

    st.event_add_callback(VIR_STREAM_EVENT_READABLE, move |st, event_type| {
        read_callback(st, event_type)
    })
    .unwrap();

    loop {
        event_run_default_impl();
    }
}
