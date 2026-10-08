//! # Log
//! Logging utilities for the xtask CLI.

use owo_colors::Style;

/// Simple logger that prints messages to stdout or stderr.
struct XTaskLogger;

/// Unstyled.
const UNSTYLED: Style = owo_colors::Style::new().remove_all_effects();
