use std::{env, path::PathBuf};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("none") {
        return;
    }

    let linker_script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("linker.ld");

    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!(
        "cargo:rustc-link-arg-bin=kernel=-T{}",
        linker_script.display()
    );
    println!("cargo:rustc-link-arg-bin=kernel=-zmax-page-size=0x1000");
}
