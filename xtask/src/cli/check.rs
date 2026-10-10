use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use log::info;

/// Checks whether the project is set up and its runtime dependencies are available.
pub fn check() -> anyhow::Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live inside a Cargo workspace")
        .to_path_buf();

    let required_files = [
        root.join("boot/BOOTX64.EFI"),
        root.join("firmware/OVMF_CODE.fd"),
        root.join("firmware/OVMF_VARS.fd"),
        root.join("assets/glowday.png"),
    ];
    let missing_files: Vec<_> = required_files
        .iter()
        .filter(|path| !path.is_file())
        .collect();
    if !missing_files.is_empty() {
        let missing = missing_files
            .iter()
            .map(|path| format!("  - {}", path.display()))
            .collect::<Vec<_>>()
            .join("\n");
        bail!(
            "Project setup is incomplete. Missing required files:\n{missing}\nRun `cargo xtask setup` to install the boot assets and firmware."
        );
    }
    info!("Required project assets are present.");

    info!("Checking for QEMU...");
    let status = Command::new("qemu-system-x86_64")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("QEMU is not installed or qemu-system-x86_64 is not on PATH")?;
    if !status.success() {
        bail!("QEMU version check failed with status {status}");
    }

    info!("Project check passed.");
    Ok(())
}
