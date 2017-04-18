This crate provides a Rust bindings to the libvirt C library

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
  match Connect.connect("test:///default") {
    Ok(conn) => {
      ...
      conn.close()
    }
    Err(e) => panic!("Not able to connect; code:{}, message:{}",
                     e.code, e.message)
  }
```

For structs which are reference counted at C level, it is necessary to
explicitly release the reference at Rust level. For instance is a Rust
method returns a &Domain, it is necessary to call `free` on it when no
longer required.

```
  match conn.lookup_domain_by_name("myguest") {
    Ok(dom) => {
      ...
      dom.free()
    }
    Err(e) => panic!("Something wrongs....")
  }
```

## Testing/Exercises

For executing tests and other excerices

```
# cargo test --verbose -- --nocapture
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