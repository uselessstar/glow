use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, bail};
use log::info;

const TARGET: &str = "x86_64-unknown-none";

pub fn run() -> anyhow::Result<()> {
    let root = project_root();
    let kernel_version = kernel_version(&root)?;
    let limine_efi = root.join("boot/BOOTX64.EFI");
    let firmware_code = root.join("firmware/OVMF_CODE.fd");
    let firmware_vars = root.join("firmware/OVMF_VARS.fd");

    for path in [&limine_efi, &firmware_code, &firmware_vars] {
        if !path.is_file() {
            bail!(
                "Required boot asset is missing: {}. Run `cargo xtask setup` first.",
                path.display()
            );
        }
    }

    info!("Building the kernel for {TARGET}...");
    let status = Command::new("cargo")
        .current_dir(root.join("crates/kernel"))
        .args([
            "build",
            "--package",
            "kernel",
            "--config",
            root.join("crates/kernel/.cargo/config.toml")
                .to_str()
                .unwrap(),
        ])
        .status()
        .context("Failed to start Cargo while building the kernel")?;
    if !status.success() {
        bail!("Kernel build failed with status {status}");
    }

    let esp_dir = root.join("esp");
    let efi_boot_dir = esp_dir.join("EFI/BOOT");
    let kernel_dir = esp_dir.join("boot");
    create_dir_all(&efi_boot_dir)?;
    create_dir_all(&kernel_dir)?;

    fs::copy(&limine_efi, efi_boot_dir.join("BOOTX64.EFI"))
        .context("Failed to copy Limine into the EFI system partition")?;
    fs::copy(
        root.join("assets/glowday.png"),
        esp_dir.join("boot/glowday.png"),
    )
    .context("Failed to copy Limine wallpaper")?;
    fs::write(
        esp_dir.join("limine.conf"),
        format!(
            r#"timeout: no
wallpaper: boot():/boot/glowday.png
interface_branding_color: 000000
interface_help_color: 000000
interface_help_color_bright: 000000
interface_resolution: 800x600
term_palette: 000000;000000;000000;000000;000000;000000;000000;000000
term_palette_bright: 000000;000000;000000;000000;000000;000000;000000;000000
term_background: ff000000
term_foreground: 000000
term_background_bright: ff000000
term_foreground_bright: 000000

/Glow OS v{kernel_version}
    comment: Glow OS — Kernel v{kernel_version}
    protocol: limine
    path: boot():/boot/glowkrnl
    if_fw_type: uefi

/EFI fallback
    comment: Default EFI loader
    comment: order-priority=10
    protocol: efi
    path: boot():/EFI/BOOT/BOOTX64.EFI
"#
        ),
    )
    .context("Failed to generate the Limine configuration")?;
    fs::copy(
        root.join("target").join(TARGET).join("debug/kernel"),
        kernel_dir.join("glowkrnl"),
    )
    .context("Failed to copy the built kernel into the EFI system partition")?;

    let qemu_vars = root.join("target/qemu/OVMF_VARS.fd");
    create_dir_all(qemu_vars.parent().expect("QEMU vars path has a parent"))?;
    fs::copy(&firmware_vars, &qemu_vars)
        .context("Failed to prepare writable OVMF variables for QEMU")?;

    info!("Starting QEMU with the generated ESP...");
    let status = Command::new("qemu-system-x86_64")
        .current_dir(&root)
        .args([
            "-machine",
            "q35,accel=kvm",
            "-cpu",
            "host",
            "-smp",
            "2",
            "-m",
            "2G",
            "-drive",
            &format!(
                "if=pflash,format=raw,readonly=on,file={}",
                firmware_code.display()
            ),
            "-drive",
            &format!("if=pflash,format=raw,file={}", qemu_vars.display()),
            "-device",
            "ich9-ahci,id=ahci",
            "-serial",
            "stdio",
            "-drive",
            "if=none,id=esp,format=raw,file=fat:rw:esp",
            "-device",
            "ide-hd,drive=esp,bus=ahci.0,bootindex=1",
        ])
        .status()
        .context("Failed to start qemu-system-x86_64; check that QEMU is installed and on PATH")?;
    if !status.success() {
        bail!("QEMU exited with status {status}");
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live inside a Cargo workspace")
        .to_path_buf()
}

fn kernel_version(root: &Path) -> anyhow::Result<String> {
    let manifest_path = root.join("crates/kernel/Cargo.toml");
    let output = Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .context("Failed to retrieve workspace metadata from Cargo")?;
    if !output.status.success() {
        bail!(
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .context("Cargo returned invalid JSON workspace metadata")?;
    metadata["packages"]
        .as_array()
        .context("Cargo metadata does not contain a packages array")?
        .iter()
        .find(|package| {
            package["name"] == "kernel"
                && package["manifest_path"]
                    .as_str()
                    .is_some_and(|path| Path::new(path) == manifest_path)
        })
        .and_then(|package| package["version"].as_str())
        .map(str::to_owned)
        .context("Cargo metadata does not contain the kernel package or its version")
}
