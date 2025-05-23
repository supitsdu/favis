//! Main entrypoint for the `favis` CLI.

use anyhow::Result;
use clap::{CommandFactory, Parser};
use owo_colors::OwoColorize;

mod cli;
mod img;
mod link;
mod manifest;
mod progress;
mod svg;

use crate::progress::create_spinner;
use crate::svg::PixmapExt;
use cli::{Cli, Commands, SizeLevel};
mod icon_sizes;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Generate {
            source,
            coverage,
            manifest: gen_manifest,
            output,
            raster_ok,
        }) => {
            // Check if the source file is raster (not SVG)
            let is_svg = source.to_lowercase().ends_with(".svg");
            if !is_svg && !raster_ok {
                eprintln!(
                    "{}: raster source detected. Please supply an SVG for best results.",
                    "Error".red().bold()
                );
                eprintln!("(Override with --raster-ok, but expect quality loss.)");
                std::process::exit(1);
            }

            // Setup progress spinner
            let spinner = create_spinner("Starting favicon generation");

            // Show warning for raster images if proceeding
            if !is_svg && raster_ok {
                spinner.set_message(format!(
                    "{} Raster image quality may be poor at larger sizes",
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
                let data = std::fs::read(&source)?;

                // First render the SVG at its original size
                spinner.set_message(format!("{}", "Rendering SVG to bitmap...".cyan().bold()));
                let pixmap = svg::render_svg_original_size(&data, Some(&spinner))?;

                // Convert the Pixmap to a DynamicImage
                spinner.set_message(format!("{}", "Converting to image format...".cyan().bold()));
                let original_image = pixmap.to_dynamic_image()?;

                // Create a temporary file path for the original-sized PNG
                let temp_dir = std::env::temp_dir();
                let temp_file = temp_dir.join("favis_temp_original.png");
                let temp_path = temp_file.to_string_lossy();

                // Save the original image to the temp file
                spinner.set_message(format!("{}", "Saving temporary PNG file...".cyan().bold()));
                original_image.save(&temp_file)?;

                // Now process it like a regular PNG
                img::process(&temp_path, &output, &png_sizes, &ico_sizes, Some(&spinner))?;

                // Clean up the temporary file
                spinner.set_message(format!(
                    "{}",
                    "Cleaning up temporary files...".cyan().bold()
                ));
                if temp_file.exists() {
                    std::fs::remove_file(temp_file)?;
                }
            } else {
                img::process(&source, &output, &png_sizes, &ico_sizes, Some(&spinner))?;
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
