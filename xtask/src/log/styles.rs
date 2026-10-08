use owo_colors::Style;

/// Unstyled.
const UNSTYLED: Style = owo_colors::Style::new().remove_all_effects();

/// Bright white and bold for blocks.
const BLOCK: Style = owo_colors::Style::new().bright_white().bold();

/// Dimmed and italic for modules.
const MODULE: Style = owo_colors::Style::new().white().dimmed().italic();
