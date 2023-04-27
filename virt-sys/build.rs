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

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut config = pkg_config::Config::new();

    config
        .atleast_version(LIBVIRT_VERSION)
        .probe("libvirt")?;

    if cfg!(feature = "qemu") {
        config
            .atleast_version(LIBVIRT_VERSION)
            .probe("libvirt-qemu")?;
    }

    let bindgen_in_dir = PathBuf::from("bindgen");
    let bindgen_in_file = bindgen_in_dir.join("bindings.rs");
    let bindgen_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let bindgen_out_file = bindgen_out_dir.join("bindings.rs");

    if cfg!(feature = "bindgen_regenerate") {
        let mut bindings = bindgen::builder()
            .header("wrapper.h")
            .allowlist_var("^(VIR_|vir).*")
            .allowlist_type("^vir.*")
            .allowlist_function("^vir.*")
            // this is only false on esoteric platforms which libvirt does not support
            .size_t_is_usize(true)
            .generate_comments(false)
            .prepend_enum_name(false)
            .ctypes_prefix("::libc");

        if cfg!(feature = "qemu") {
            bindings = bindings
                .clang_arg("-DBINDGEN_USE_QEMU");
        }

        bindings
            .generate()
            .map_err(|_| String::from("could not generate bindings"))?
            .write_to_file(&bindgen_in_file)?;
    }

    fs::copy(bindgen_in_file, bindgen_out_file)?;

    Ok(())
}
