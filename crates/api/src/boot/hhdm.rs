//! Typed access to Limine's Higher Half Direct Map (HHDM) response.
//!
//! The offset describes the direct-map virtual address corresponding to
//! physical address zero. This module reports the offset; it does not create
//! mappings or verify that an arbitrary physical address is mapped.

/// The HHDM offset supplied by Limine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hhdm {
    offset: u64,
}

impl Hhdm {
    /// Returns the virtual-address offset of the higher-half direct map.
    ///
    /// For a physical address covered by the direct map, adding this offset
    /// gives its corresponding virtual address.
    pub const fn offset(&self) -> u64 {
        self.offset
    }
}

/// Errors that can occur while querying the HHDM response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HhdmError {
    /// The kernel's requested Limine base revision is unsupported.
    UnsupportedBaseRevision,
    /// Limine did not provide a response to the HHDM request.
    ResponseUnavailable,
}

/// Returns the HHDM offset provided by Limine.
///
/// The caller must ensure Limine has completed boot handoff before calling
/// this function.
///
/// # Errors
///
/// Returns [`HhdmError::UnsupportedBaseRevision`] if the requested base
/// revision is unsupported, or [`HhdmError::ResponseUnavailable`] if the
/// bootloader did not provide the requested response.
pub fn get() -> Result<Hhdm, HhdmError> {
    if !raw_api::limine::BASE_REVISION.is_supported() {
        return Err(HhdmError::UnsupportedBaseRevision);
    }

    let response = raw_api::limine::HHDM
        .response()
        .ok_or(HhdmError::ResponseUnavailable)?;

    Ok(Hhdm {
        offset: response.offset,
    })
}
