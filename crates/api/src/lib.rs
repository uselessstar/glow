#![no_std]
#![deny(missing_docs)]

//! Safe, high-level interfaces used by the kernel.
//!
//! This crate wraps low-level hardware and boot-protocol details exposed by
//! [`raw_api`] with typed operations and explicit error handling.

/// Information and services provided by the bootloader during startup.
pub mod boot;
/// Logging and panic-reporting facilities.
pub mod log;
/// Synchronized, formatted serial output.
pub mod serial;
