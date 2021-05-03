use virt::connect::Connect;

/// Implement a C main function, with at least 1 reference to a libvirt function
/// It doesn't need to succeed at runtime; only compile successfully
#[no_mangle]
extern "C" fn main() -> u32 {
    Connect::open("any_driver").ok();
    0
}
