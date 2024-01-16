use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::{env, process};

const LIBVIRT_VERSION: &str = "6.0.0";

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(feature = "bindgen_regenerate")]
fn bindgen_regenerate(bindgen_out_file: &PathBuf) -> Result<(), Box<dyn Error>> {

    // We want to make sure that the generated bindings.rs file includes all libvirt APIs,
    // including the ones that are QEMU-specific
    if !cfg!(feature = "qemu") {
        return Err("qemu must be enabled along with bindgen_regenerate".into())
    }

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_var("^(VIR_|vir).*")
        .allowlist_type("^vir.*")
        .allowlist_function("^vir.*")
        // this is only false on esoteric platforms which libvirt does not support
        .size_t_is_usize(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .generate_cstr(true)
        .ctypes_prefix("::libc");

    bindings
        .generate()
        .map_err(|_| String::from("could not generate bindings"))?
        .write_to_file(bindgen_out_file)?;

    Ok(())
}

#[cfg(not(feature = "bindgen_regenerate"))]
fn bindgen_regenerate(_: &PathBuf) -> Result<(), Box<dyn Error>> {

    // We haven't been asked to regenerate bindings.rs, so nothing to do here
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut config = pkg_config::Config::new();

    // Normally we would make the calls to probe() fatal by not ignoring their return value, but we
    // want to be able to build the documentation for the library even when the libvirt header
    // files are not present. This is necessary so that docs.rs can build and publish the API
    // documentation for libvirt-rust. If any of these calls fail, then we'll still get an error
    // when attempting to link against libvirt (e.g. when building the test suite).
    let _ = config
        .atleast_version(LIBVIRT_VERSION)
        .probe("libvirt");

    if cfg!(feature = "qemu") {
        let _ = config
            .atleast_version(LIBVIRT_VERSION)
            .probe("libvirt-qemu");
    }

    let bindgen_in_dir = PathBuf::from("bindgen");
    let bindgen_in_file = bindgen_in_dir.join("bindings.rs");
    let bindgen_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let bindgen_out_file = bindgen_out_dir.join("bindings.rs");

    bindgen_regenerate(&bindgen_in_file)?;

    fs::copy(bindgen_in_file, bindgen_out_file)?;

    Ok(())
}
