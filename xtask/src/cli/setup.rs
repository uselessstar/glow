use std::{
    fs::{self, OpenOptions, create_dir_all},
    io::copy,
    path::{Path, PathBuf},
};

use log::info;

pub fn setup() -> anyhow::Result<()> {
    info!("Setting up the project...");

    let root = project_root();
    let boot_dir = root.join("boot");
    create_dir_all(&boot_dir)?;

    if boot_dir.join("BOOTX64.EFI").exists() {
        info!("Limine is already installed at {}", boot_dir.display());
    } else {
        let temp_dir = temp_dir();
        create_dir_all(&temp_dir)?;
        download_limine(&temp_dir)?;
        info!("Limine bootloader downloaded successfully.");

        info!("Extracting Limine bootloader into the boot directory...");
        extract_limine(&temp_dir, &boot_dir)?;
    }

    install_ovmf(&root.join("firmware"))
}

const TEMP_DIR: &str = "/tmp/xtask";

/// The url to download the Limine bootloader from. This is used in the setup command to download and install Limine if it is not already installed.
const LIMINE_URL: &str =
    "https://github.com/Limine-Bootloader/Limine/releases/download/v12.9.3/limine-binary.tar.gz";

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live inside a Cargo workspace")
        .to_path_buf()
}

fn temp_dir() -> PathBuf {
    PathBuf::from(TEMP_DIR)
}

fn download_limine(temp_dir: &Path) -> anyhow::Result<()> {
    let archive_path = temp_dir.join("limine-binary.tar.gz");
    info!(
        "Downloading Limine bootloader from {LIMINE_URL} to {}",
        archive_path.display()
    );

    let response = ureq::get(LIMINE_URL).call()?;
    if !response.status().is_success() {
        anyhow::bail!("Limine download failed with status {}", response.status());
    }

    let mut reader = response.into_body();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&archive_path)?;
    copy(&mut reader.as_reader(), &mut file)?;
    Ok(())
}

fn extract_limine(temp_dir: &Path, boot_dir: &Path) -> anyhow::Result<()> {
    let archive_path = temp_dir.join("limine-binary.tar.gz");
    let extracted_path = temp_dir.join("limine-binary");

    if extracted_path.exists() {
        fs::remove_dir_all(&extracted_path)?;
    }

    info!(
        "Extracting Limine bootloader from {} to {}",
        archive_path.display(),
        temp_dir.display()
    );

    let tar_gz = std::fs::File::open(&archive_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(temp_dir)?;

    let source_efi = extracted_path.join("BOOTX64.EFI");
    if !source_efi.exists() {
        anyhow::bail!(
            "The extracted Limine archive did not contain BOOTX64.EFI at {}",
            source_efi.display()
        );
    }

    fs::copy(&source_efi, boot_dir.join("BOOTX64.EFI"))?;
    fs::remove_file(archive_path)?;
    fs::remove_dir_all(extracted_path)?;
    Ok(())
}

const OVMF_PATHS: &[(&str, &str)] = &[
    (
        "/usr/share/edk2/x64/OVMF_CODE.4m.fd",
        "/usr/share/edk2/x64/OVMF_VARS.4m.fd",
    ),
    (
        "/usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd",
        "/usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd",
    ),
    (
        "/usr/share/edk2/ovmf/OVMF_CODE.fd",
        "/usr/share/edk2/ovmf/OVMF_VARS.fd",
    ),
    (
        "/usr/share/OVMF/OVMF_CODE_4M.fd",
        "/usr/share/OVMF/OVMF_VARS_4M.fd",
    ),
    (
        "/usr/share/OVMF/OVMF_CODE.fd",
        "/usr/share/OVMF/OVMF_VARS.fd",
    ),
];

fn install_ovmf(firmware_dir: &Path) -> anyhow::Result<()> {
    let code_destination = firmware_dir.join("OVMF_CODE.fd");
    let vars_destination = firmware_dir.join("OVMF_VARS.fd");
    if code_destination.exists() && vars_destination.exists() {
        info!("OVMF is already installed at {}", firmware_dir.display());
        return Ok(());
    }

    let (code_source, vars_source) = OVMF_PATHS
        .iter()
        .map(|(code, vars)| (Path::new(code), Path::new(vars)))
        .find(|(code, vars)| code.is_file() && vars.is_file())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Could not find a matching x86_64 OVMF CODE/VARS pair. Install the OVMF package for your system; checked: {}",
                OVMF_PATHS
                    .iter()
                    .map(|(code, _)| *code)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    create_dir_all(firmware_dir)?;
    info!(
        "Copying OVMF CODE from {} to {}",
        code_source.display(),
        code_destination.display()
    );
    fs::copy(code_source, &code_destination)?;

    info!(
        "Copying OVMF VARS template from {} to {}",
        vars_source.display(),
        vars_destination.display()
    );
    fs::copy(vars_source, &vars_destination)?;
    Ok(())
}
