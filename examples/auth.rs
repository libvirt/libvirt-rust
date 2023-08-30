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

//! A trivial example to ilustract how to setup a connection auth
//! hanlder.
//!
//! By default the program try to connect to
//! 'test+tcp://127.0.0.1/default'
//!
//! An example of libvirtd configuration for sasl:
//!
//! ```
//!  listen_tls=0
//!  listen_tcp=1
//!  auth_tcp=sasl
//!  listen_addr="127.0.0.1"
//! ```
//!
//! Then to create user nad password:
//!
//! ```
//! saslpasswd2 -a libvirt user
//! ```

use std::{env, io};

use virt::connect::{Connect, ConnectAuth, ConnectCredential};
use virt::sys;

fn main() {
    let uri = env::args().nth(1);

    fn callback(creds: &mut Vec<ConnectCredential>) {
        for cred in creds {
            let mut input = String::new();

            println!("{}:", cred.prompt);
            match cred.typed as u32 {
                sys::VIR_CRED_AUTHNAME => {
                    io::stdin().read_line(&mut input).expect("");
                    cred.result = Some(String::from(input.trim()));
                }
                sys::VIR_CRED_PASSPHRASE => {
                    io::stdin().read_line(&mut input).expect("");
                    cred.result = Some(String::from(input.trim()));
                }
                _ => {
                    panic!("Should not be here...");
                }
            }
        }
    }
    let mut auth = ConnectAuth::new(
        vec![sys::VIR_CRED_AUTHNAME, sys::VIR_CRED_PASSPHRASE],
        callback,
    );

    println!("Attempting to connect to hypervisor: '{:?}'...", uri);
    let mut conn = match Connect::open_auth(uri.as_deref(), &mut auth, 0) {
        Ok(c) => {
            println!("Connected");
            c
        }
        Err(e) => panic!("Not connected: {}", e),
    };
    if let Err(e) = conn.close() {
        panic!("Failed to disconnect from hypervisor: {}", e);
    }
}
