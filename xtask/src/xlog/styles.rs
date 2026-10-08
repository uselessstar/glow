use owo_colors::Style;

/// Bright white and bold for blocks.
pub(super) const BLOCK: Style = owo_colors::Style::new().bright_white().bold();

/// Dimmed and italic for modules.
pub(super) const MODULE: Style = owo_colors::Style::new().white().dimmed().italic();

/// Dimmed for trace messages.
pub(super) const TRACE: Style = owo_colors::Style::new().dimmed();

/// Dimmed for debug messages.
pub(super) const DEBUG: Style = TRACE;

/// Green and bold for info messages.
pub(super) const INFO: Style = owo_colors::Style::new().green().bold();

/// Yellow and bold for warning messages.
pub(super) const WARN: Style = owo_colors::Style::new().yellow().bold();

/// Red and bold for error messages.
pub(super) const ERROR: Style = owo_colors::Style::new().red().bold();
