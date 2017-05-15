# This crate provides a Rust bindings to the libvirt C library

The binding tries to be a fairly direct mapping of the underling C API
with some differences to respect Rust conventions.

## Important considerations

Make sure to have `libvirt-dev` or `libvirt-devel` package (or the
development files otherwise somewhere in your include path).

The library is still under development and the API is not quite stable
yet.

The binding uses standard errors handling from Rust. Each method
(there are some exceptions) is returning a type `Option` or `Result`.

## Documentation

* https://docs.rs/crate/virt/
* http://libvirt.org/html/libvirt-libvirt.html

## Testing/Exercises

CI is executing tests automatically from libvirt 1.2.0 to 2.5.0. Using
Rust from stable, beta to nightly.

* https://travis-ci.org/sahid/libvirt-rs

For executing tests and other excerices.

```
cargo test --verbose --
```

Integration tests are using real connection for example
integration_qemu is using a qemu:///system connection. They are all
ignored by default.

```
cargo test --verbose -- --ignored
```

For executing examples

```
cargo run --example hello -- test:///default
```

## Contributing

Any bug fixes and other improvements are welcome at any time. It's
possible to look at what is missing by running command like:

```
$ python tools/api_tests.py virDomain
{'file': 'libvirt-domain', 'name': 'virDomainMigrateSetMaxSpeed', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainRef', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainGetMemoryParameters', 'module': 'libvirt-domain'}
...
```
