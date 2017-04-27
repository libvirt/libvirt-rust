# This crate provides a Rust bindings to the libvirt C library

The binding tries to be a fairly direct mapping of the underling C API
with some differences to respect Rust conventions.

## Important considerations

Make sure to have `libvirt-dev` package (or the development files
otherwise somewhere in your include path).

The library is still under development and the API is not quite
stable.

The binding uses standard errors handling from Rust. Each method
(there are some exceptions) is returning a type `Result`.

```
  match Connect.open("test:///default") {
    Ok(conn) => {
      ...
      assert_eq!(0, conn.close().unwrap_or(-1)) // Verify that all objects has
                                                // been freen before we close
						// the connection.
    }
    Err(e) => panic!("Not able to connect; code:{}, message:{}",
                     e.code, e.message)
  }
```

Most of the structs are automatically release their references by
implemementing `Drop` trait. It provides a way to run some code when a
value goes out of scope. But for structs which are reference counted
at C level, it is still possible to explicitly release the reference
at Rust level. For instance if a Rust method returns a *Domain, it is
possible to call `free` on it when no longer required.


```
  match conn.lookup_domain_by_name("myguest") {
    Ok(mut dom) => {
      ...
      dom.free() // Explicitly releases memory at Rust level.
    }
    Err(e) => panic!("Something wrongs....")
  }
```

## Testing/Exercises

For executing tests and other excerices

```
cargo test --verbose -- --nocapture
```

For executing examples

```
cargo run --example hello -- test:///default
```


## Contributing

To look at what we need to implement, in each source file there is
TODO note with a list of API missing;

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