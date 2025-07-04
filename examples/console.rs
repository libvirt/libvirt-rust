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

//! This file contains trivial example code to read/write via console. This file
//! connects to the running VM and works as a console client.
//! 1st arg is URI, 2nd arg is the name of VM, and 3rd arg is a console name.
//! If the console name is omitted, then the first console will be opened.
//!
//! Examples
//! ```
//! $ cargo run --example console -- 'qemu:///system' 'mytestvm' 'serial0'
//! ```
//!
//! Largely inspired by libvirt-python/examples/consolecallback.py

use std::{
    env,
    io::{self, Read, Write},
    os::unix::io::AsRawFd,
    sync::{atomic::AtomicBool, Arc},
};
use termios::{Termios, ECHO, ICANON, ISIG, TCSANOW};
use virt::{
    connect::Connect,
    domain::Domain,
    event::{event_add_handle, event_register_default_impl, event_run_default_impl},
    stream::Stream,
    sys::{
        virStreamEventType, VIR_DOMAIN_CONSOLE_FORCE, VIR_EVENT_HANDLE_READABLE,
        VIR_STREAM_EVENT_READABLE, VIR_STREAM_NONBLOCK,
    },
};

pub struct Console {
    pub st: Stream,
    pub cond: Arc<AtomicBool>,
}

impl Console {
    pub fn new(st: Stream) -> Self {
        Console {
            st,
            cond: Arc::new(AtomicBool::new(true)),
        }
    }
}

fn read_callback(stream: &Stream, event_type: virStreamEventType) {
    if event_type == VIR_STREAM_EVENT_READABLE {
        let mut buf = vec![0; 1024];
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        match stream.recv(buf.as_mut_slice()) {
            Ok(_) => {
                stdout.write_all(&buf).unwrap();
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

fn stdin_callback(
    _watch: libc::c_int,
    _fd: libc::c_int,
    events: libc::c_int,
    console_ptr: *mut libc::c_void,
) {
    if events == VIR_EVENT_HANDLE_READABLE as libc::c_int {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let con = unsafe { &mut *(console_ptr as *mut Console) };

        let mut buf = [0; 1];
        if stdin.read(&mut buf).is_ok() {
            // 29 == Ctrl-]
            if buf[0] == 29 {
                con.cond.store(false, std::sync::atomic::Ordering::SeqCst);
            }
            let _ = con.st.send(&buf);
        }
    }
}

#[allow(dead_code)]
fn timer_task(_timer: libc::c_int, _opaque: *mut libc::c_void) {
    println!("timer!");
}

fn set_raw_mode() -> Termios {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    let mut termios = Termios::from_fd(fd).unwrap();
    let orig_termios = termios;

    termios.c_lflag &= !(ICANON | ECHO | ISIG);
    termios::tcsetattr(fd, TCSANOW, &termios).unwrap();

    orig_termios
}

fn reset_mode(orig_termios: Termios) {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    termios::tcsetattr(fd, TCSANOW, &orig_termios).unwrap();
    println!();
}

fn main() {
    println!("Try to connect via console");
    println!("Escape character is ^] (Ctrl + ])");

    let uri = env::args().nth(1);
    let name = env::args().nth(2).unwrap();
    let dev_name = env::args().nth(3);

    let _ = event_register_default_impl();
    let conn = Connect::open(uri.as_deref()).unwrap();
    let dom = Domain::lookup_by_name(&conn, &name).unwrap();
    let st = Stream::new(&conn, VIR_STREAM_NONBLOCK).unwrap();
    dom.open_console(dev_name.as_deref(), &st, VIR_DOMAIN_CONSOLE_FORCE)
        .unwrap();

    let mut console = Console::new(st);

    console
        .st
        .event_add_callback(VIR_STREAM_EVENT_READABLE, |st, event_type| {
            read_callback(st, event_type)
        })
        .unwrap();

    let console_ptr = &mut console as *mut Console as *mut libc::c_void;

    let _ehw = event_add_handle(
        0,
        VIR_EVENT_HANDLE_READABLE,
        |watch, fd, events, opaque| stdin_callback(watch, fd, events as libc::c_int, opaque),
        console_ptr,
    )
    .unwrap();

    //let ret = ehw.event_remove_handle();
    //ehw.event_update_handle(virt::sys::VIR_EVENT_HANDLE_READABLE);

    //let etw = virt::event::event_add_timeout(3000, |timer, opaque| timer_task(timer, opaque), std::ptr::null_mut()).unwrap();

    //let _ret = etw.event_remove_timeout();
    //etw.event_update_timeout(1000);

    let orig_termios = set_raw_mode();

    while console.cond.load(std::sync::atomic::Ordering::SeqCst) {
        let _ = event_run_default_impl();
    }

    reset_mode(orig_termios);
}
