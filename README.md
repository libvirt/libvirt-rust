Rust bindings for libvirt.

Make sure to have `libvirt-dev` package (or the development files
otherwise somewhere in your include path)

## Contributing

To look at what we still need to implement in each source file a there
is TODO note with a list of API missing;

```
$ git grep TODO:
```

In `tools/` a small script can be use to look at what is missing:

```
$ python tools/api_tests.py virDomain
{'file': 'libvirt-domain', 'name': 'virDomainMigrateSetMaxSpeed', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainRef', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainGetMemoryParameters', 'module': 'libvirt-domain'}
...
```

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