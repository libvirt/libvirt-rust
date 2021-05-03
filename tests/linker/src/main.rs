use virt::connect::Connect;

fn main() {
    Connect::open("any_driver").ok();
}
