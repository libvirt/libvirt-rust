# This crate provides a Rust bindings to the libvirt C library

The bindings try to be a direct mapping of the underling C API
with some differences to match Rust conventions.

## Important considerations

Make sure the `libvirt-dev` or `libvirt-devel` package is installed
(or that the development files are in your include path).

The bindings do not implement all of what the C library is providing
but we do consider the current API quite stable.

The bindings use standard errors handling from Rust. Each method
(there are some exceptions) returns a type `Option` or `Result`.

## Optional features

* `qemu` allows using `libvirt-qemu` functions, such as `qemu_monitor_command`.

* `bindgen_regenerate` uses the `bindgen` crate to generate a
  Rust-compatible representation of the C API. The output for a
  recent version of libvirt is already included in the repository, so
  only maintainers should ever need to use this feature.

## Documentation

* https://libvirt.org/html/index.html
* https://docs.rs/crate/virt/

### To execute locally tests and other excerices

`cargo fmt -v -- --check`

The code is formatted using `rustfmt`, you should ensure that the
check is passing before to submit your patch(es). It may be required
to execute `rustup component add rustfmt` in your environment.

`cargo test --verbose`

Integration tests use a real connection to libvirtd. For instance
integration_qemu.rs uses a qemu:///system connection. They are all
ignored by default.

`cargo test --verbose -- --ignored`

Similar to
[libvirt-go-module](https://gitlab.com/libvirt/libvirt-go-module),
the integration tests also require that libvirtd listens on localhost
with sasl auth. This can be setup by editing
`/etc/libvirt/libvirtd.conf` as follows:

```
  listen_tls=0
  listen_tcp=1
  auth_tcp=sasl
  listen_addr="127.0.0.1"
```

and starting libvirtd with the --listen flag (this can
be set in /etc/sysconfig/libvirtd to make it persistent).

Then create a sasl user

`saslpasswd2 -a libvirt user`

and enter "pass" as the password.

### To run examples

```
# cargo run --example hello
# cargo run --example migrate -- qemu:///system tcp+qemu://192.168.0.1/system myguest

```

## Contributing

The libvirt project aims to add support for new APIs to libvirt-rs as
soon as they are added to the main libvirt C library. If you are
submitting changes to the libvirt C library API, please submit a
libvirt-rs change at the same time.

For more information, see the [CONTRIBUTING](CONTRIBUTING.md) file.

The list of missing methods can be displayed with:

```
$ python tools/api_tests.py virDomain
{'file': 'libvirt-domain', 'name': 'virDomainMigrateSetMaxSpeed', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainRef', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainGetMemoryParameters', 'module': 'libvirt-domain'}
...
```
