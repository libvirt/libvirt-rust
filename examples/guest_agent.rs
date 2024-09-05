use std::env;
use virt::connect::Connect;
use virt::domain::Domain;
use virt::sys;

fn main() {
    let uri = env::args().nth(1);
    let name = env::args().nth(2).expect("Domain name requried");

    let conn = Connect::open(uri.as_deref()).unwrap();

    let domain = Domain::lookup_by_name(&conn, &name).unwrap();
    let result = domain.qemu_agent_command(
        "{\"execute\": \"guest-info\"}",
        sys::VIR_DOMAIN_QEMU_AGENT_COMMAND_BLOCK,
        0,
    );
    match result {
        Ok(r) => println!("Result: {}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}
