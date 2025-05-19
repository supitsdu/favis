//! CLI argument definitions for favis

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "favis",
    about = "Generate web favicons and manifest for your PWA or website.",
    long_about = "\
favis is a CLI tool to generate favicon PNGs, ICOs, and web manifests from a single image source.

Overview:
  - Creates multiple PNG favicon images in industry-standard sizes
  - Generates a favicon.ico file with multiple sizes embedded
  - Optionally generates a web manifest file for PWAs
  - Can create HTML <link> tags from an existing manifest

Usage:
  > favis generate logo.svg
  > favis generate logo.svg --manifest
  > favis generate logo.svg --coverage extended
  > favis link ./public/manifest.webmanifest

Tips:
  - SVG sources are strongly recommended for best quality at all sizes
  - Use --output to specify where generated files should be saved
  - Run 'favis <SUBCOMMAND> --help' for detailed options for each command
",
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
    /// Only required sizes (minimal set, fastest)
    Required,
    /// Recommended sizes (good compatibility, default)
    Recommended,
    /// All sizes (maximum compatibility)
    Extended,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate favicon PNGs, ICO, and manifest from a source image
    #[command(
        about = "Generate icons and manifest from a source image",
        long_about = "\
Generate favicon PNGs, ICO, and optionally a web manifest from a single source image.

Output:
  - Multiple PNG favicon files in various sizes (e.g., favicon-32x32.png)
  - A single favicon.ico file with multiple sizes embedded
  - Optional manifest.webmanifest file for PWAs

Examples:
  > favis generate logo.svg
  > favis generate logo.svg --coverage extended --manifest --output ./public
  > favis generate logo.png --raster-ok
"
    )]
    Generate {
        /// Path to the source image file (SVG preferred)
        #[arg(
            help = "Source image file (SVG strongly recommended for quality)",
            value_name = "SOURCE"
        )]
        source: String,

        /// Icon size coverage: required, recommended, or extended
        #[arg(
            short,
            long,
            value_enum,
            default_value = "recommended",
            help = "Icon size coverage level (affects number of icons generated)",
            value_name = "COVERAGE"
        )]
        coverage: SizeLevel,

        /// Also generate a web manifest file
        #[arg(short, long, help = "Generate a manifest.webmanifest file for PWAs")]
        manifest: bool,

        /// Output directory for generated files
        #[arg(
            short,
            long,
            default_value = ".",
            help = "Directory where generated files will be saved",
            value_name = "DIR"
        )]
        output: String,

        /// Allow raster source images (PNG/JPG) despite quality concerns
        #[arg(
            long,
            help = "Allow raster sources (PNG/JPG) despite quality concerns at large sizes"
        )]
        raster_ok: bool,
    },

    /// Generate HTML <link> tags from a webmanifest file
    #[command(
        about = "Generate HTML <link> tags from a manifest.webmanifest",
        long_about = "\
Generate HTML <link> tags for favicon and app icons defined in a manifest.webmanifest file.

Purpose:
  - Creates the proper <link> tags needed in your HTML head section
  - Automatically sets correct 'rel' attributes based on icon purposes
  - Organizes tags by importance and size

Examples:
  > favis link ./public/manifest.webmanifest
  > favis link ./public/manifest.webmanifest --base /assets/icons --output ./public/favicon-links.html
  > favis link ./manifest.webmanifest --base https://cdn.example.com/icons
"
    )]
    Link {
        /// Path to the manifest.webmanifest file
        #[arg(
            help = "Path to existing manifest.webmanifest file to read",
            value_name = "MANIFEST"
        )]
        manifest: String,

        /// Base URL path to prefix for all icon links
        #[arg(
            long,
            help = "Base URL to prefix for all icon paths (e.g., /assets or https://cdn.example.com)",
            value_name = "URL"
        )]
        base: Option<String>,

        /// Output HTML file path (default: print to stdout)
        #[arg(
            short,
            long,
            help = "Save HTML to file instead of printing to stdout",
            value_name = "FILE"
        )]
        output: Option<String>,
    },
}
