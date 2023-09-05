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

//! A Rust bindings for libvirt.
//!
//! Libvirt is a portable toolkit to interact with the virtualisation
//! capabilities of Linux, Solaris and other operating systems.
//!
//! The binding tries to be a fairly direct mapping of the underling C
//! API with some differences to respect Rust conventions. So for
//! example C functions related to a domain like: `virDomainCreate`
//! will be mapped in the binding like `dom.create()` or
//! `virDomainPinVcpu` as `dom.pin_vcpu`.
//!
//! The binding uses standard errors handling from Rust. Each method
//! (there are some exceptions) is returning a type `Option` or
//! `Result`.
//!
//! ```
//! use virt::connect::Connect;
//!
//! if let Ok(mut conn) = Connect::open("test://default") {
//!   assert_eq!(Ok(0), conn.close());
//! }
//! ```
//!
//! Most of the structs are automatically release their references by
//! implemementing `Drop` trait but for structs which are reference
//! counted at C level, it is still possible to explicitly release the
//! reference at Rust level. For instance if a Rust method returns a
//! *Domain, it is possible to call `free` on it when no longer
//! required.
//!
//! ```
//! use virt::connect::Connect;
//! use virt::domain::Domain;
//!
//! if let Ok(mut conn) = Connect::open("test://default") {
//!   if let Ok(mut dom) = Domain::lookup_by_name(&conn, "myguest") {
//!       assert_eq!(Ok(()), dom.free());   // Explicitly releases memory at Rust level.
//!       assert_eq!(Ok(0), conn.close());
//!   }
//! }
//! ```
//!
//! For each methods accepting or returning a virTypedParameter array
//! a new Rust struct has been defined where each attribute is
//! handling a type Option.
//!
//! ```
//! use virt::connect::Connect;
//! use virt::domain::Domain;
//!
//! if let Ok(mut conn) = Connect::open("test://default") {
//!   if let Ok(dom) = Domain::lookup_by_name(&conn, "myguest") {
//!     if let Ok(memp) = dom.get_memory_parameters(0) {
//!       if memp.hard_limit.is_some() {
//!         println!("hard limit: {}", memp.hard_limit.unwrap())
//!       }
//!     }
//!   }
//!   assert_eq!(Ok(0), conn.close());
//! }
//! ```

pub extern crate virt_sys as sys;

macro_rules! c_chars_to_string {
    ($x:expr) => {{
        let ret = ::std::ffi::CStr::from_ptr($x)
            .to_string_lossy()
            .into_owned();
        libc::free($x as *mut libc::c_void);
        ret
    }};

    ($x:expr, nofree) => {{
        ::std::ffi::CStr::from_ptr($x)
            .to_string_lossy()
            .into_owned()
    }};
}

macro_rules! string_to_mut_c_chars {
    ($x:expr) => {
        // Usage of this should ensure deallocation.
        ::std::ffi::CString::new($x).unwrap().into_raw() as *mut libc::c_char
    };
}

mod util;

pub mod connect;
pub mod domain;
pub mod domain_snapshot;
pub mod error;
pub mod interface;
pub mod network;
pub mod nodedev;
pub mod nwfilter;
pub mod secret;
pub mod storage_pool;
pub mod storage_vol;
pub mod stream;
