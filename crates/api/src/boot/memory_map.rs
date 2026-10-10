//! Typed, read-only access to the physical memory map provided by Limine.
//!
//! The regions describe physical address ranges. This module does not create
//! virtual mappings or allocate memory.

use core::fmt;

/// A memory region type defined by the Limine boot protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    /// Memory available for general use after the relevant reservations are handled.
    Usable,
    /// Memory reserved by firmware or another system component.
    Reserved,
    /// ACPI tables that may be reclaimed after they have been copied or parsed.
    AcpiReclaimable,
    /// ACPI memory that must remain reserved.
    AcpiNvs,
    /// A memory range reported as faulty.
    BadMemory,
    /// Memory used by the bootloader that can be reclaimed after boot services are no longer needed.
    BootloaderReclaimable,
    /// Memory occupied by the kernel image or boot modules.
    KernelAndModules,
    /// Memory occupied by the framebuffer.
    Framebuffer,
    /// Memory reserved because it is mapped by the bootloader.
    MappedReserved,
    /// A region type unknown to this API, preserving its protocol value.
    Unknown(u64),
}

/// Converts a raw Limine memory-region type to its API representation.
impl From<u64> for RegionKind {
    fn from(kind: u64) -> Self {
        match kind {
            raw_api::limine::memmap::MEMMAP_USABLE => Self::Usable,
            raw_api::limine::memmap::MEMMAP_RESERVED => Self::Reserved,
            raw_api::limine::memmap::MEMMAP_ACPI_RECLAIMABLE => Self::AcpiReclaimable,
            raw_api::limine::memmap::MEMMAP_ACPI_NVS => Self::AcpiNvs,
            raw_api::limine::memmap::MEMMAP_BAD_MEMORY => Self::BadMemory,
            raw_api::limine::memmap::MEMMAP_BOOTLOADER_RECLAIMABLE => Self::BootloaderReclaimable,
            raw_api::limine::memmap::MEMMAP_EXECUTABLE_AND_MODULES => Self::KernelAndModules,
            raw_api::limine::memmap::MEMMAP_FRAMEBUFFER => Self::Framebuffer,
            raw_api::limine::memmap::MEMMAP_MAPPED_RESERVED => Self::MappedReserved,
            other => Self::Unknown(other),
        }
    }
}

impl fmt::Display for RegionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Usable => "usable",
            Self::Reserved => "reserved",
            Self::AcpiReclaimable => "ACPI reclaimable",
            Self::AcpiNvs => "ACPI NVS",
            Self::BadMemory => "bad memory",
            Self::BootloaderReclaimable => "bootloader reclaimable",
            Self::KernelAndModules => "kernel/modules",
            Self::Framebuffer => "framebuffer",
            Self::MappedReserved => "mapped reserved",
            Self::Unknown(kind) => return write!(f, "unknown ({kind})"),
        };
        f.write_str(name)
    }
}

/// One memory region with a typed Limine region kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    /// Starting physical address of the region.
    pub base: u64,
    /// Size of the region in bytes.
    pub length: u64,
    /// Semantic kind of the region.
    pub kind: RegionKind,
}

/// A read-only view of the physical memory map supplied by Limine at boot.
///
/// This view borrows the bootloader response for the lifetime of the kernel.
pub struct MemoryMap {
    entries: &'static [&'static raw_api::limine::memmap::Entry],
}

impl MemoryMap {
    /// Returns the number of memory regions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the memory map contains no regions.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates over memory regions using API-level types.
    pub fn entries(&self) -> impl Iterator<Item = Region> + '_ {
        self.entries.iter().map(|entry| Region {
            base: entry.base,
            length: entry.length,
            kind: entry.type_.into(),
        })
    }
}

/// Errors that can occur while querying the boot memory map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMapError {
    /// The kernel's requested Limine base revision is unsupported.
    UnsupportedBaseRevision,
    /// Limine did not provide a response to the memory-map request.
    ResponseUnavailable,
}

/// Returns the memory map provided by Limine.
///
/// The caller must ensure Limine has completed boot handoff before calling
/// this function.
///
/// # Errors
///
/// Returns [`MemoryMapError::UnsupportedBaseRevision`] if the requested base
/// revision is unsupported, or [`MemoryMapError::ResponseUnavailable`] if the
/// bootloader did not provide the requested response.
pub fn get() -> Result<MemoryMap, MemoryMapError> {
    if !raw_api::limine::BASE_REVISION.is_supported() {
        return Err(MemoryMapError::UnsupportedBaseRevision);
    }

    let response = raw_api::limine::MEMORY_MAP
        .response()
        .ok_or(MemoryMapError::ResponseUnavailable)?;

    Ok(MemoryMap {
        entries: response.entries(),
    })
}
