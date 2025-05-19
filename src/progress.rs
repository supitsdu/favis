//! Progress reporting utilities using indicatif.

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use owo_colors::OwoColorize;

/// Creates a styled progress bar for a specific task
pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}", msg.cyan().bold()));
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
