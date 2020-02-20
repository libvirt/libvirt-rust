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

## Documentation

* https://libvirt.org/html/index.html
* https://docs.rs/crate/virt/

## Tests/Exercises

CI is executing tests automatically from libvirt 2.5.0 to 5.5.0. Using
Rust from stable, beta to nightly.

* https://travis-ci.org/libvirt/libvirt-rust

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

Similar to `libvirt-go`, the integration tests also require that
libvirtd listens on localhost with sasl auth. This can be setup by
editing `/etc/libvirt/libvirtd.conf` as follows:

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

Bug fixes and other improvements to the libvirt-rs library are welcome
at any time. The preferred submission method is to use git send-email
to submit patches to the libvir-list@redhat.com mailing list. eg. to
send a single patch

```
   git send-email --to libvir-list@redhat.com --cc sahid.ferdjaoui@libremel.fr \
       --subject-prefix "rust PATCH" --smtp-server=$HOSTNAME -1
```

Or to send all patches on the current branch, against master

```
   git send-email --to libvir-list@redhat.com --cc sahid.ferdjaoui@libremel.fr \
       --subject-prefix "rust PATCH" --smtp-server=$HOSTNAME --no-chain-reply-to \
       --cover-letter --annotate master..
```

It is also possible to use git-publish.

Note the master GIT repository is at

* https://libvirt.org/git/?p=libvirt-rust.git;a=summary

The following automatic read-only mirrors are available as a
convenience to allow contributors to "fork" the repository:

* https://gitlab.com/libvirt/libvirt-rust
* https://github.com/libvirt/libvirt-rust

While you can send pull-requests to these mirrors, they will be
re-submitted via email to the mailing list for review before being
merged, unless they are trivial/obvious bug fixes.

The list of missing methods can be displayed with:

```
$ python tools/api_tests.py virDomain
{'file': 'libvirt-domain', 'name': 'virDomainMigrateSetMaxSpeed', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainRef', 'module': 'libvirt-domain'}
{'file': 'libvirt-domain', 'name': 'virDomainGetMemoryParameters', 'module': 'libvirt-domain'}
...
```
