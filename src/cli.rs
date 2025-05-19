//! CLI argument definitions for wfig

use clap::{Parser, Subcommand, ValueEnum};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum SizeLevel {
    /// Only required sizes (minimal set)
    Required,
    /// Recommended sizes (good compatibility)
    Recommended,
    /// All sizes (maximum compatibility)
    Extended,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate icons and manifest from a source image
    Generate {
        /// Source image file (SVG preferred)
        #[arg(help = "Path to the source image file (SVG preferred)")]
        source: String,

        /// Icon size coverage level
        #[arg(short, long, value_enum, default_value = "recommended", help = "Icon size coverage: required, recommended, or extended")]
        coverage: SizeLevel,

        /// Also generate a webmanifest
        #[arg(short, long, help = "Also generate a web manifest file")]
        manifest: bool,

        /// Output directory for generated files
        #[arg(short, long, default_value = ".", help = "Output directory for generated files")]
        output: String,
        
        /// Allow raster source images (PNG/JPG) despite quality concerns
        #[arg(long, help = "Allow raster source images (PNG/JPG) despite quality concerns")]
        raster_ok: bool,
    },
    
    /// Generate HTML link tags from a webmanifest file
    Link {
        /// Path to the manifest.webmanifest file
        #[arg(help = "Path to the manifest.webmanifest file")]
        manifest: String,
        
        /// Base URL path to prefix for all icon links
        #[arg(long, help = "Base URL path to prefix for all icon links")]
        base: Option<String>,
        
        /// Output HTML file path
        #[arg(short, long, help = "Output HTML file path")]
        output: Option<String>,
    },
}
