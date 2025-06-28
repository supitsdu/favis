//! CLI argument definitions for favis

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "favis",
    about = "favis turns one image into all the favicons and web assets you need — fast, free, and open-source.",
    long_about = "\
favis turns one image into all the favicons and web assets you need — fast, free, and open-source.

What it does:
  - Generates standard-sized PNG favicons automatically
  - Bundles them into a multi-resolution favicon.ico file
  - Can generate a web manifest for PWAs
  - Can also create HTML <link> tags from an existing manifest

How to use it:
  > favis generate logo.svg
  > favis generate logo.svg --manifest
  > favis generate logo.svg --coverage extended
  > favis link ./public/manifest.webmanifest

Tips:
  - Got an SVG? Perfect! It's the best source for clean, scalable icons
  - Use --output to choose where files are saved
  - Run 'favis <SUBCOMMAND> --help' for more options
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
        about = "Turn your image into all the favicons you need — easy and fast!",
        long_about = "\
Turn your image into all the favicons and icons you need — easy and fast!

What it creates:
  - PNG favicons in all the standard sizes (e.g. favicon-32x32.png)
  - A favicon.ico file with multiple sizes baked in
  - An optional manifest.webmanifest file for PWAs

How to use it:
  > favis generate logo.svg
  > favis generate logo.svg --coverage extended --manifest --output ./public
  > favis generate logo.png --raster-ok

Helpful tips:
  - SVGs are ideal — they scale cleanly at any size
  - Use --coverage extended to generate icons for every use case
  - PNGs are fine too — just pass --raster-ok and you're good to go!
"
    )]
    Generate {
        /// Path to the source image file (SVG preferred)
        #[arg(
            help = "Source image file — SVG recommended for best quality",
            value_name = "SOURCE"
        )]
        source: String,

        /// Icon size coverage: required, recommended, or extended
        #[arg(
            short,
            long,
            value_enum,
            default_value = "recommended",
            help = "Choose how many icon sizes to generate",
            value_name = "COVERAGE"
        )]
        coverage: SizeLevel,

        /// Also generate a web manifest file
        #[arg(short, long, help = "Include a manifest.webmanifest file for PWAs")]
        manifest: bool,

        /// Output directory for generated files
        #[arg(
            short,
            long,
            default_value = ".",
            help = "Where to save the generated files (default: current dir)",
            value_name = "DIR"
        )]
        output: String,

        /// Allow raster source images (PNG/JPG) despite quality concerns
        #[arg(
            long,
            help = "Allow raster images like PNG/JPG (lower quality at large sizes)"
        )]
        raster_ok: bool,
    },

    /// Generate HTML <link> tags from a webmanifest file
    #[command(
        about = "Generate HTML <link> tags from your manifest.webmanifest file",
        long_about = "\
Need the right HTML <link> tags for your favicon setup? We've got you.

What this command does:
  - Reads your manifest.webmanifest file
  - Generates <link> tags with proper rel and size attributes
  - Sorts them by importance and size
  - Lets you add custom URL prefixes (CDNs, asset paths, etc.)

How to use it:
  > favis link ./public/manifest.webmanifest
  > favis link ./public/manifest.webmanifest --base /assets/icons --output ./public/favicon-links.html
  > favis link ./manifest.webmanifest --base https://cdn.example.com/icons

Pro tips:
  - By default, output goes to the terminal — perfect for copy-paste
  - Use --output to save directly to an HTML file
  - Use --base to prefix your icon URLs with a path or CDN
"
    )]
    Link {
        /// Path to the manifest.webmanifest file
        #[arg(
            help = "Path to your manifest.webmanifest file",
            value_name = "MANIFEST"
        )]
        manifest: String,

        /// Base URL path to prefix for all icon links
        #[arg(
            long,
            help = "Add a URL prefix to all icon paths (e.g. /assets or CDN URL)",
            value_name = "URL"
        )]
        base: Option<String>,

        /// Output HTML file path (default: print to stdout)
        #[arg(
            short,
            long,
            help = "Save the output to a file instead of printing it",
            value_name = "FILE"
        )]
        output: Option<String>,
    },
}
