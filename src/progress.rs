//! Progress reporting utilities using indicatif.

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;

/// Creates a styled progress bar for a specific task
pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.green} {msg}")
            .expect("Valid progress bar template"),
    );
    pb.set_message(format!("{}", msg.cyan().bold()));
    pb.enable_steady_tick(Duration::from_millis(60));
    pb
}
