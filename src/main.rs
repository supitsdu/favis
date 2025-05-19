//! Main entrypoint for the `wfig` CLI.

use anyhow::Result;
use clap::{Parser, CommandFactory};

mod cli;
mod img;
mod svg;
mod manifest;

use cli::{Cli, Commands};
use crate::svg::PixmapExt;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Generate { source, size_range, only_ati, manifest: gen_manifest, output }) => {
            // Parse size range (we don't actually use this currently, but keeping for future use)
            let _sizes: Vec<u32> = size_range
                .split('-')
                .filter_map(|s| s.parse().ok())
                .collect();

            let png_sizes = if only_ati {
                vec![48, 72, 96, 128, 192, 256, 384, 512]
            } else {
                vec![16, 32, 48, 72, 96, 128, 256, 384, 512]
            };
            let ico_sizes = vec![16, 32, 48, 64, 96, 128, 256];

            // SVG support: detect extension
            if source.ends_with(".svg") {
                let data = std::fs::read(&source)?;
                
                // First render the SVG at its original size
                let pixmap = svg::render_svg_original_size(&data)?;
                
                // Convert the Pixmap to a DynamicImage
                let original_image = pixmap.to_dynamic_image()?;
                
                // Create a temporary file path for the original-sized PNG
                let temp_dir = std::env::temp_dir();
                let temp_file = temp_dir.join("wfig_temp_original.png");
                let temp_path = temp_file.to_string_lossy();
                
                // Save the original image to the temp file
                original_image.save(&temp_file)?;
                
                // Now process it like a regular PNG
                img::process(&temp_path, &output, &png_sizes, &ico_sizes)?;
                
                // Clean up the temporary file
                if temp_file.exists() {
                    std::fs::remove_file(temp_file)?;
                }
            } else {
                img::process(&source, &output, &png_sizes, &ico_sizes)?;
            }

            if gen_manifest {
                manifest::generate_manifest(&output, &png_sizes)?;
            }
        }
        None => {
            // If no subcommand, print help and exit
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}

