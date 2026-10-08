use owo_colors::Style;

/// Unstyled.
const UNSTYLED: Style = owo_colors::Style::new().remove_all_effects();

/// Bright white and bold for blocks.
const BLOCK: Style = owo_colors::Style::new().bright_white().bold();

/// Dimmed and italic for modules.
const MODULE: Style = owo_colors::Style::new().white().dimmed().italic();

/// Dimmed for trace messages.
const TRACE: Style = owo_colors::Style::new().dimmed();

/// Dimmed for debug messages.
const DEBUG: Style = TRACE;

/// Green and bold for info messages.
const INFO: Style = owo_colors::Style::new().green().bold();

/// Yellow and bold for warning messages.
const WARN: Style = owo_colors::Style::new().yellow().bold();

/// Red and bold for error messages.
const ERROR: Style = owo_colors::Style::new().red().bold();
