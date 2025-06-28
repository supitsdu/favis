//! Main entrypoint for the `favis` CLI.

use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

mod cli;
mod error;
mod img;
mod link;
mod manifest;
mod progress;
mod svg;

use error::{FavisError, Result};

use crate::progress::create_spinner;
use crate::svg::PixmapExt;
use cli::{Cli, Commands, SizeLevel};
mod icon_sizes;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Setup global cancellation flag for signal handling
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = cancelled.clone();
    
    // Setup graceful signal handling for Ctrl+C
    ctrlc::set_handler(move || {
        cancelled_clone.store(true, Ordering::Relaxed);
        // Silent cancellation - let the error handler provide the user message
    }).expect("Error setting Ctrl+C handler");

    // Run the CLI with cancellation support
    if let Err(err) = run_cli(cli, cancelled) {
        err.display_friendly();
        std::process::exit(1);
    }
    
    Ok(())
}

fn run_cli(cli: Cli, cancelled: Arc<AtomicBool>) -> Result<()> {
    match cli.command {
        Some(Commands::Generate {
            source,
            coverage,
            manifest: gen_manifest,
            output,
            raster_ok,
        }) => {
            // Validate source file exists
            if !std::path::Path::new(&source).exists() {
                return Err(FavisError::file_not_found(&source));
            }

            // Check file extension to determine format
            let source_lower = source.to_lowercase();
            let is_svg = source_lower.ends_with(".svg");
            let is_png = source_lower.ends_with(".png");

            // Validate that the file is a supported image format
            // Primary focus: SVG (vector graphics)
            // Secondary support: PNG (raster, with quality warnings)
            if !is_svg && !is_png {
                return Err(FavisError::invalid_format(
                    "Oops! That file format isn't supported."
                ));
            }

            // Check if using PNG (raster) and require explicit approval
            if is_png && !raster_ok {
                return Err(FavisError::invalid_format(
                    "PNG detected! You'll need the --raster-ok flag to continue."
                ));
            }

            // Setup progress spinner
            let spinner = create_spinner("Starting favicon generation");

            // Show warning for PNG images if proceeding
            if is_png && raster_ok {
                spinner.set_message(format!(
                    "{} PNG raster image quality may be poor at larger sizes",
                    "Warning:".yellow().bold()
                ));
                std::thread::sleep(std::time::Duration::from_millis(1500)); // Show warning briefly
            }

            // Convert CLI SizeLevel to internal IconPriority
            let priority = match coverage {
                SizeLevel::Required => icon_sizes::IconPriority::Required,
                SizeLevel::Recommended => icon_sizes::IconPriority::Recommended,
                SizeLevel::Extended => icon_sizes::IconPriority::Extended,
            };

            // Get the appropriate sizes based on priority
            let png_sizes = icon_sizes::get_png_sizes(priority);
            let ico_sizes = icon_sizes::get_ico_sizes(priority);

            spinner.set_message(format!(
                "{} {}",
                "Processing source file:".cyan().bold(),
                source.yellow()
            ));

            // SVG support: detect extension
            if is_svg {
                spinner.set_message(format!("{}", "Loading SVG file...".cyan().bold()));
                let data = std::fs::read(&source)
                    .map_err(|_| FavisError::file_not_found(format!("Cannot read SVG file: {}", source)))?;

                // Validate SVG data
                if data.is_empty() {
                    return Err(FavisError::invalid_svg("SVG file is empty"));
                }

                // First render the SVG at its original size
                spinner.set_message(format!("{}", "Rendering SVG to bitmap...".cyan().bold()));
                let pixmap = svg::render_svg_original_size(&data, Some(&spinner))?;

                // Convert the Pixmap to a DynamicImage
                spinner.set_message(format!("{}", "Converting to image format...".cyan().bold()));
                let original_image = pixmap.to_dynamic_image()?;

                // Create output directory if it doesn't exist
                if let Err(_) = std::fs::create_dir_all(&output) {
                    return Err(FavisError::write_error(format!("Cannot create output directory: {}", output)));
                }

                // Create a temporary file path for the original-sized PNG
                let temp_dir = std::env::temp_dir();
                let temp_file = temp_dir.join("favis_temp_original.png");
                let temp_path = temp_file.to_string_lossy();

                // Save the original image to the temp file
                spinner.set_message(format!("{}", "Saving temporary PNG file...".cyan().bold()));
                original_image.save(&temp_file)
                    .map_err(|_| FavisError::write_error("Cannot save temporary PNG file"))?;

                // Now process it like a regular PNG
                match img::process(&temp_path, &output, &png_sizes, &ico_sizes, Some(&spinner), cancelled.clone()) {
                    Ok(_) => {},
                    Err(ref e) if e.to_string().contains("cancelled") => {
                        spinner.abandon();
                        return Err(FavisError::user_cancelled());
                    }
                    Err(e) => return Err(e),
                }

                // Clean up the temporary file
                spinner.set_message(format!(
                    "{}",
                    "Cleaning up temporary files...".cyan().bold()
                ));
                if temp_file.exists() {
                    let _ = std::fs::remove_file(temp_file); // Ignore cleanup errors
                }
            } else {
                match img::process(&source, &output, &png_sizes, &ico_sizes, Some(&spinner), cancelled.clone()) {
                    Ok(_) => {},
                    Err(ref e) if e.to_string().contains("cancelled") => {
                        spinner.abandon();
                        return Err(FavisError::user_cancelled());
                    }
                    Err(e) => return Err(e),
                }
            }

            if gen_manifest {
                manifest::generate_manifest(&output, priority, Some(&spinner))?;
            }

            spinner.finish_with_message(format!(
                "{} {}",
                "✓".green().bold(),
                "All favicon assets generated successfully!".green().bold()
            ));
        }
        Some(Commands::Link {
            manifest,
            base,
            output,
        }) => {
            // Create spinner for progress indication
            let spinner = create_spinner("Generating HTML link tags");

            // Call the link generation function
            link::generate_links(
                &manifest,
                base.as_deref(),
                output.as_deref(),
                Some(&spinner),
            )?;
        }
        None => {
            // If no subcommand, print help and exit
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
