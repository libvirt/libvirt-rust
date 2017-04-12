Rust bindings for libvirt.

Make sure to have `libvirt-dev` package (or the development files
otherwise somewhere in your include path)

## Sample

```
use virt::connect::Connect;

match Connect::open("test:///default") {
  Ok(conn) => {
    let doms = conn.list_domains().unwrap_or(vec![]);
    for domid in doms {
      match conn.domain_lookup_by_id(domid) {
        Ok(domain) => {
          println!("I'm managing '{}' with Rust and libvirt!",
                   domain.get_name().unwrap_or("no name".to_string()))
        }
        Err(e) => panic!(
          "failed to retrieve domain, code {}, message: {}", e.code, e.message)
      }
    }
    conn.close();
  },
  Err(e) => panic!(
    "failed with code {}, message: {}", e.code, e.message)
}
```

## Testing/Exercises

For executing tests and other excerices

```
# cargo test --verbose -- --nocapture
```