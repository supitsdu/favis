//! CLI argument definitions for wfig

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "wfig",
    about = "Generate web favicons and manifest for your PWA or website.",
    long_about = "wfig is a CLI tool to generate favicon PNGs, ICOs, and web manifests from a single image (PNG, JPG, SVG).",
    author,
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate icons and manifest from a source image
    Generate {
        /// Source image file (png, jpg, svg)
        #[arg(short, long, help = "Path to the source image file (PNG, JPG, or SVG)")]
        source: String,

        /// Size range for PNGs (e.g. "16-256")
        #[arg(short = 'r', long, default_value = "16-256", help = "Size range for PNGs, e.g. 16-256")]
        size_range: String,

        /// Only generate Apple Touch Icons (recommended for iOS)
        #[arg(long, help = "Only generate Apple Touch Icon sizes")]
        only_ati: bool,

        /// Also generate a webmanifest
        #[arg(short, long, help = "Also generate a web manifest file")]
        manifest: bool,

        /// Output directory for generated files
        #[arg(short, long, default_value = ".", help = "Output directory for generated files")]
        output: String,
    },
}
