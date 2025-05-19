//! Image processing for PNG and ICO outputs.

use anyhow::Result;
use image::{imageops::FilterType, ImageEncoder};
use ico::{IconDir, IconImage, ResourceType};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::BufWriter;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;

/// Processes a source image (PNG/JPEG/GIF) and generates resized PNGs and an optional ICO.
///
/// # Arguments
/// * `src_path` - Path to the source image file.
/// * `out_dir` - Directory inside which to save outputs.
/// * `png_sizes` - List of square sizes (in px) to generate PNGs.
/// * `ico_sizes` - List of sizes to include in the ICO; if empty, no ICO is generated.
pub fn process(
    src_path: &str, 
    out_dir: &str, 
    png_sizes: &[u32], 
    ico_sizes: &[u32],
    progress: Option<&ProgressBar>,
) -> Result<()> {
    // Read and decode source
    if let Some(pb) = progress {
        pb.set_message(format!("{} {}", "Loading source image:".cyan().bold(), src_path.yellow()));
    }
    let img = image::open(src_path)?;

    // Ensure output directory exists
    fs::create_dir_all(out_dir)?;

    // Helper: Save resized PNG
    fn save_resized_png(img: &image::DynamicImage, size: u32, out_dir: &str) -> Result<()> {
        let mut resized = img.resize_exact(size, size, FilterType::Lanczos3);

        // Clear edge artifacts by ensuring transparency or solid color
        resized = resized.adjust_contrast(1.0); // Adjust contrast to minimize border artifacts

        let mut out_path = PathBuf::from(out_dir);
        out_path.push(format!("favicon-{}x{}.png", size, size));
        let file = File::create(&out_path)?;
        let buf_writer = BufWriter::new(file);
        let encoder = image::codecs::png::PngEncoder::new(buf_writer);
        let rgba = resized.to_rgba8();
        encoder.write_image(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            image::ExtendedColorType::Rgba8,
        )?;
        Ok(())
    }

    // Helper: Get RGBA for ICO
    fn get_rgba_for_ico(img: &image::DynamicImage, size: u32) -> Vec<u8> {
        let resized = img.resize_exact(size, size, FilterType::Lanczos3);
        resized.to_rgba8().into_raw()
    }

    // Generate PNGs
    for &size in png_sizes {
        if let Some(pb) = progress {
            pb.set_message(format!("{} {}x{}", "Creating PNG".cyan().bold(), size.to_string().yellow(), size.to_string().yellow()));
        }
        save_resized_png(&img, size, out_dir)?;
    }
    
    // Generate ICO if requested
    if !ico_sizes.is_empty() {
        if let Some(pb) = progress {
            pb.set_message(format!("{}", "Creating ICO file with multiple sizes...".cyan().bold()));
        }
        
        let mut icon_dir = IconDir::new(ResourceType::Icon);
        for &size in ico_sizes {
            if let Some(pb) = progress {
                pb.set_message(format!("{} {}x{}", "Adding size to ICO:".cyan().bold(), size.to_string().yellow(), size.to_string().yellow()));
            }
            let rgba = get_rgba_for_ico(&img, size);
            let icon_image = IconImage::from_rgba_data(size, size, rgba);
            // encode_png returns Result<IconDirEntry, _>, so handle error and add entry
            let entry = ico::IconDirEntry::encode(&icon_image)?;
            icon_dir.add_entry(entry);
        }
        
        let mut ico_path = PathBuf::from(out_dir);
        ico_path.push("favicon.ico");
        
        if let Some(pb) = progress {
            pb.set_message(format!("{}", "Writing favicon.ico file...".cyan().bold()));
        }
        
        let mut file = BufWriter::new(File::create(ico_path)?);
        icon_dir.write(&mut file)?;
    }

    Ok(())
}