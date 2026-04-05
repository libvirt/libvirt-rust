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

use std::os::raw::{c_int, c_void};
use std::os::unix::io::RawFd;

use crate::error::Error;
use crate::util::check_neg;

// wrapper for callbacks
unsafe extern "C" fn event_callback<
    F: FnMut(c_int, c_int, sys::virEventHandleType, *mut c_void),
>(
    watch: c_int,
    fd: c_int,
    events: c_int,
    opaque: *mut c_void,
) {
    let ecd = &mut *(opaque as *mut EventCallbackData<F>);
    (ecd.cb)(watch, fd, events as sys::virEventHandleType, ecd.opaque);
}

unsafe extern "C" fn event_free<F: FnMut(c_int, c_int, sys::virEventHandleType, *mut c_void)>(
    opaque: *mut c_void,
) {
    let _ = Box::from_raw(opaque as *mut EventCallbackData<F>);
}

struct EventCallbackData<F: FnMut(c_int, c_int, sys::virEventHandleType, *mut c_void)> {
    cb: F,
    opaque: *mut c_void,
}

pub struct EventHandleWatch(pub c_int);

impl EventHandleWatch {
    pub fn as_raw(&self) -> c_int {
        self.0
    }

    /// Remove an event file handle
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventRemoveHandle>
    pub fn event_remove_handle(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virEventRemoveHandle(self.0) })?;
        Ok(())
    }

    /// Update an event file handle
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventUpdateHandle>
    pub fn event_update_handle(&self, events: sys::virEventHandleType) {
        let events = events as c_int;
        unsafe { sys::virEventUpdateHandle(self.0, events) };
    }
}

/// Add an event file handle
///
/// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventAddHandle>
pub fn event_add_handle<F: 'static + FnMut(c_int, c_int, sys::virEventHandleType, *mut c_void)>(
    fd: RawFd,
    events: sys::virEventHandleType,
    cb: F,
    opaque: *mut c_void,
) -> Result<EventHandleWatch, Error> {
    let event_callback_data: Box<EventCallbackData<F>> = Box::new(EventCallbackData { cb, opaque });

    let ret = unsafe {
        sys::virEventAddHandle(
            fd,
            events as c_int,
            Some(event_callback::<F>),
            Box::into_raw(event_callback_data) as *mut c_void,
            Some(event_free::<F>),
        )
    };
    if ret == -1 {
        unsafe {
            let _ = Box::from_raw(opaque as *mut EventCallbackData<F>);
        }
        return Err(Error::last_error());
    }
    Ok(EventHandleWatch(ret))
}

// wrapper for callbacks
unsafe extern "C" fn event_timeout_callback<F: FnMut(c_int, *mut c_void)>(
    timer: c_int,
    opaque: *mut c_void,
) {
    let ecd = &mut *(opaque as *mut EventTimeoutCallbackData<F>);
    (ecd.cb)(timer, ecd.opaque);
}

unsafe extern "C" fn event_timeout_free<F: FnMut(c_int, *mut c_void)>(opaque: *mut c_void) {
    let _ = Box::from_raw(opaque as *mut EventTimeoutCallbackData<F>);
}

struct EventTimeoutCallbackData<F: FnMut(c_int, *mut c_void)> {
    cb: F,
    opaque: *mut c_void,
}

pub struct EventTimeoutWatch(pub c_int);

impl EventTimeoutWatch {
    pub fn as_raw(&self) -> c_int {
        self.0
    }

    /// Remove an event timer
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventRemoveTimeout>
    pub fn event_remove_timeout(&self) -> Result<(), Error> {
        let _ = check_neg!(unsafe { sys::virEventRemoveTimeout(self.0) })?;
        Ok(())
    }

    /// Update an event timer
    ///
    /// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventUpdateTimeout>
    pub fn event_update_timeout(&self, timeout: c_int) {
        unsafe { sys::virEventUpdateTimeout(self.0, timeout) };
    }
}

/// Add an event timer
///
/// See <https://libvirt.org/html/libvirt-libvirt-event.html#virEventAddTimeout>
pub fn event_add_timeout<F: 'static + FnMut(c_int, *mut c_void)>(
    timeout: c_int,
    cb: F,
    opaque: *mut c_void,
) -> Result<EventTimeoutWatch, Error> {
    let event_timeout_callback_data: Box<EventTimeoutCallbackData<F>> =
        Box::new(EventTimeoutCallbackData { cb, opaque });

    let ret = unsafe {
        sys::virEventAddTimeout(
            timeout,
            Some(event_timeout_callback::<F>),
            Box::into_raw(event_timeout_callback_data) as *mut c_void,
            Some(event_timeout_free::<F>),
        )
    };
    if ret == -1 {
        unsafe {
            let _ = Box::from_raw(opaque as *mut EventTimeoutCallbackData<F>);
        }
        return Err(Error::last_error());
    }
    Ok(EventTimeoutWatch(ret))
}

pub fn event_register_default_impl() -> Result<(), Error> {
    let _ = check_neg!(unsafe { sys::virEventRegisterDefaultImpl() })?;
    Ok(())
}

pub fn event_run_default_impl() -> Result<(), Error> {
    let _ = check_neg!(unsafe { sys::virEventRunDefaultImpl() })?;
    Ok(())
}
