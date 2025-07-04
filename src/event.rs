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

use std::os::unix::io::RawFd;

use crate::error::Error;

// wrapper for callbacks
unsafe extern "C" fn event_callback<
    F: FnMut(libc::c_int, libc::c_int, sys::virEventHandleType, *mut libc::c_void),
>(
    watch: libc::c_int,
    fd: libc::c_int,
    events: libc::c_int,
    opaque: *mut libc::c_void,
) {
    let ecd = &mut *(opaque as *mut EventCallbackData<F>);
    (ecd.cb)(watch, fd, events as sys::virEventHandleType, ecd.opaque);
}

unsafe extern "C" fn event_free<
    F: FnMut(libc::c_int, libc::c_int, sys::virEventHandleType, *mut libc::c_void),
>(
    opaque: *mut libc::c_void,
) {
    let _ = Box::from_raw(opaque as *mut EventCallbackData<F>);
}

struct EventCallbackData<
    F: FnMut(libc::c_int, libc::c_int, sys::virEventHandleType, *mut libc::c_void),
> {
    cb: F,
    opaque: *mut libc::c_void,
}

pub struct EventHandleWatch(pub libc::c_int);

impl EventHandleWatch {
    pub fn as_raw(&self) -> libc::c_int {
        self.0
    }

    pub fn event_remove_handle(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virEventRemoveHandle(self.0) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn event_update_handle(&self, events: sys::virEventHandleType) {
        let events = events as libc::c_int;
        unsafe { sys::virEventUpdateHandle(self.0, events) };
    }
}

pub fn event_add_handle<
    F: 'static + FnMut(libc::c_int, libc::c_int, sys::virEventHandleType, *mut libc::c_void),
>(
    fd: RawFd,
    events: sys::virEventHandleType,
    cb: F,
    opaque: *mut libc::c_void,
) -> Result<EventHandleWatch, Error> {
    let event_callback_data: Box<EventCallbackData<F>> = Box::new(EventCallbackData { cb, opaque });

    let ret = unsafe {
        sys::virEventAddHandle(
            fd,
            events as libc::c_int,
            Some(event_callback::<F>),
            Box::into_raw(event_callback_data) as *mut libc::c_void,
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
unsafe extern "C" fn event_timeout_callback<F: FnMut(libc::c_int, *mut libc::c_void)>(
    timer: libc::c_int,
    opaque: *mut libc::c_void,
) {
    let ecd = &mut *(opaque as *mut EventTimeoutCallbackData<F>);
    (ecd.cb)(timer, ecd.opaque);
}

unsafe extern "C" fn event_timeout_free<F: FnMut(libc::c_int, *mut libc::c_void)>(
    opaque: *mut libc::c_void,
) {
    let _ = Box::from_raw(opaque as *mut EventTimeoutCallbackData<F>);
}

struct EventTimeoutCallbackData<F: FnMut(libc::c_int, *mut libc::c_void)> {
    cb: F,
    opaque: *mut libc::c_void,
}

pub struct EventTimeoutWatch(pub libc::c_int);

impl EventTimeoutWatch {
    pub fn as_raw(&self) -> libc::c_int {
        self.0
    }

    pub fn event_remove_timeout(&self) -> Result<(), Error> {
        let ret = unsafe { sys::virEventRemoveTimeout(self.0) };
        if ret == -1 {
            return Err(Error::last_error());
        }
        Ok(())
    }

    pub fn event_update_timeout(&self, timeout: libc::c_int) {
        unsafe { sys::virEventUpdateTimeout(self.0, timeout) };
    }
}

pub fn event_add_timeout<F: 'static + FnMut(libc::c_int, *mut libc::c_void)>(
    timeout: libc::c_int,
    cb: F,
    opaque: *mut libc::c_void,
) -> Result<EventTimeoutWatch, Error> {
    let event_timeout_callback_data: Box<EventTimeoutCallbackData<F>> =
        Box::new(EventTimeoutCallbackData { cb, opaque });

    let ret = unsafe {
        sys::virEventAddTimeout(
            timeout,
            Some(event_timeout_callback::<F>),
            Box::into_raw(event_timeout_callback_data) as *mut libc::c_void,
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
    let ret = unsafe { sys::virEventRegisterDefaultImpl() };
    if ret == -1 {
        return Err(Error::last_error());
    }
    Ok(())
}

pub fn event_run_default_impl() -> Result<(), Error> {
    let ret = unsafe { sys::virEventRunDefaultImpl() };
    if ret == -1 {
        return Err(Error::last_error());
    }
    Ok(())
}
