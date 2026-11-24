//! Terminal capability helpers used by the interactive TUI.
use std::io;

/// Returns whether the TUI should send terminal graphics capability queries.
/// Set MOVIEBOX_NO_IMAGE=1/true to disable image probing on terminals that
/// do not support graphics query sequences (including many mobile terminals).
pub fn should_query_images() -> bool {
    !std::env::var("MOVIEBOX_NO_IMAGE")
        .ok()
        .is_some_and(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
}

/// Update the terminal window title without interrupting the TUI.
pub fn set_window_title(title: &str) -> io::Result<()> {
    crossterm::execute!(io::stdout(), crossterm::terminal::SetTitle(title))
}
