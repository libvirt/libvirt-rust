use std::process::Command;
use std::path::PathBuf;

fn cargo_build(directory: &str) -> Result<(), String> {
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(directory)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        let std_err = std::string::String::from_utf8(output.stderr).unwrap();
        return Err(std_err)
    }
    Ok(())
}

fn link(directory: &str) -> Result<(), String> {
    let output = Command::new("gcc")
        .arg("-lsymbols")
        .arg("-lvirt")
        .arg("-lm")
        .arg("-ldl")
        .arg("-lpthread")
        .arg("-L")
        .arg(directory)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        let std_err = std::string::String::from_utf8(output.stderr).unwrap();
        return Err(std_err)
    }
    Ok(())
}

/// Tests that libvirt-rust only references valid symbols by using the GNU linker
#[test]
fn test_valid_symbols() {
    let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_build(d.join("tests/symbols/").to_str().unwrap()).unwrap();
    assert!(d.join("tests/symbols/target/debug/libsymbols.a").exists());
    link(d.join("tests/symbols/target/debug/").to_str().unwrap()).unwrap();
}
