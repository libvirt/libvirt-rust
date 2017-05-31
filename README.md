# This crate provides a Rust bindings to the libvirt C library

The binding tries to be a fairly direct mapping of the underling C API
with some differences to respect Rust conventions.

## Important considerations

Make sure to have `libvirt-dev` or `libvirt-devel` package (or the
development files otherwise somewhere in your include path).

The binding does not implement all of what the C library is providing
but we do consider the current API quite stable.

The binding uses standard errors handling from Rust. Each method
(there are some exceptions) is returning a type `Option` or `Result`.

## Documentation

* http://libvirt.org/html/libvirt-libvirt.html
* https://docs.rs/crate/virt/

## Tests/Exercises

CI is executing tests automatically from libvirt 1.2.0 to 3.3.0. Using
Rust from stable, beta to nightly.

* https://travis-ci.org/sahid/libvirt-rs

### To execute locally tests and other excerices

`cargo test --verbose`

Integration tests are using real connection to libvirtd. For instance
integration_qemu.rs is using a qemu:///system connection. They are all
ignored by default.

`cargo test --verbose -- --ignored`

As for `libvirt-go` the integration test also requires that libvirtd
to listen for TCP connection on localhost with sasl auth. This can be
setup by editing `/etc/libvirt/libvirtd.conf` to set

```
  listen_tls=0
  listen_tcp=1
  auth_tcp=sasl
  listen_addr="127.0.0.1"
```

and then start libvirtd with the --listen flag (this can
be set in /etc/sysconfig/libvirtd to make it persistent).

Then create a sasl user

`saslpasswd2 -a libvirt user`

and enter "pass" as the password.

### To execute examples

```
# cargo run --example hello
# cargo run --example migrate -- qemu:///system tcp+qemu://192.168.0.1/system myguest

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
