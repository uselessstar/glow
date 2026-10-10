//! Raw Limine protocol objects used by the kernel.

pub use limine::memmap;

#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: limine::RequestsStartMarker = limine::RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP: limine::request::MemmapRequest = limine::request::MemmapRequest::new();

#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: limine::RequestsEndMarker = limine::RequestsEndMarker::new();
