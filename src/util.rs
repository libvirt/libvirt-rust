use libc::c_ulong;

#[cfg(all(target_pointer_width = "64", not(windows)))]
pub fn c_ulong_to_u64(val: c_ulong) -> u64 {
    val
}

#[cfg(not(all(target_pointer_width = "64", not(windows))))]
pub fn c_ulong_to_u64(val: c_ulong) -> u64 {
    val as u64
}

macro_rules! check_null {
    ($e:expr) => {{
        let ptr = $e;
        if ptr.is_null() {
            Err($crate::error::Error::last_error())
        } else {
            Ok(ptr)
        }
    }};
}

macro_rules! check_neg {
    ($e:expr) => {{
        let ret = $e;
        if ret == -1 {
            Err($crate::error::Error::last_error())
        } else {
            Ok(ret)
        }
    }};
}

macro_rules! check_zero {
    ($e:expr) => {{
        let ret = $e;
        if ret == 0 {
            Err($crate::error::Error::last_error())
        } else {
            Ok(ret)
        }
    }};
}

pub(crate) use check_neg;
pub(crate) use check_null;
pub(crate) use check_zero;
