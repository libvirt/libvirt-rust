#[cfg(all(target_pointer_width = "64", not(windows)))]
pub fn c_ulong_to_u64(val: ::libc::c_ulong) -> u64 {
    val
}

#[cfg(not(all(target_pointer_width = "64", not(windows))))]
pub fn c_ulong_to_u64(val: ::libc::c_ulong) -> u64 {
    val as u64
}
