//! Image processing for PNG and ICO outputs.

use crate::error::{FavisError, Result};
use ico::{IconDir, IconImage, ResourceType};
use image::{imageops::FilterType, ImageEncoder};
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Tracks files created during processing for cleanup on interruption
#[derive(Debug)]
struct FileTracker {
    files: Vec<PathBuf>,
    cancelled: Arc<AtomicBool>,
}

impl FileTracker {
    fn new_with_cancellation(cancelled: Arc<AtomicBool>) -> Self {
        Self {
            files: Vec::new(),
            cancelled,
        }
    }

    fn track(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    fn cleanup(&self) {
        for file in &self.files {
            if file.exists() {
                let _ = fs::remove_file(file); // Ignore errors during cleanup
            }
        }
    }
}

impl Drop for FileTracker {
    fn drop(&mut self) {
        if self.is_cancelled() {
            self.cleanup();
        }
    }
}

/// Processes a source image (PNG/JPEG/GIF) and generates resized PNGs and an optional ICO.
/// Includes graceful handling of user cancellation and cleanup of partial files.
///
/// # Arguments
/// * `src_path` - Path to the source image file.
/// * `out_dir` - Directory inside which to save outputs.
/// * `png_sizes` - List of square sizes (in px) to generate PNGs.
/// * `ico_sizes` - List of sizes to include in the ICO; if empty, no ICO is generated.
/// * `progress` - Optional progress bar for user feedback.
/// * `cancelled` - Shared cancellation flag for graceful interruption.
pub fn process(
    src_path: &str,
    out_dir: &str,
    png_sizes: &[u32],
    ico_sizes: &[u32],
    progress: Option<&ProgressBar>,
    cancelled: Arc<AtomicBool>,
) -> Result<()> {
    // Read and decode source
    if let Some(pb) = progress {
        pb.set_message(format!(
            "{} {}",
            "Loading source image:".cyan().bold(),
            src_path.yellow()
        ));
    }

    let img = image::open(src_path)
        .map_err(|_| FavisError::file_not_found(format!("Cannot open image file: {}", src_path)))?;

    // Check minimum image dimensions for quality
    if img.width() < 64 || img.height() < 64 {
        return Err(FavisError::image_too_small(64));
    }

    // Ensure output directory exists
    fs::create_dir_all(out_dir)
        .map_err(|_| FavisError::write_error(format!("Cannot create output directory: {}", out_dir)))?;

    let mut file_tracker = FileTracker::new_with_cancellation(cancelled);

    // Helper: Save resized PNG
    fn save_resized_png(img: &image::DynamicImage, size: u32, out_dir: &str, file_tracker: &mut FileTracker) -> Result<()> {
        let mut resized = img.resize_exact(size, size, FilterType::Lanczos3);

        // Clear edge artifacts by ensuring transparency or solid color
        resized = resized.adjust_contrast(1.0); // Adjust contrast to minimize border artifacts

        let mut out_path = PathBuf::from(out_dir);
        out_path.push(format!("favicon-{}x{}.png", size, size));

        file_tracker.track(out_path.clone());

        let file = File::create(&out_path)
            .map_err(|_| FavisError::write_error(format!("Cannot create PNG file: {}", out_path.display())))?;

        let buf_writer = BufWriter::new(file);
        let encoder = image::codecs::png::PngEncoder::new(buf_writer);
        let rgba = resized.to_rgba8();

        encoder.write_image(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            image::ExtendedColorType::Rgba8,
        ).map_err(|_| FavisError::write_error("Cannot encode PNG image"))?;

        Ok(())
    }

    // Helper: Get RGBA for ICO
    fn get_rgba_for_ico(img: &image::DynamicImage, size: u32) -> Vec<u8> {
        let resized = img.resize_exact(size, size, FilterType::Lanczos3);
        resized.to_rgba8().into_raw()
    }

    // Generate PNGs
    for &size in png_sizes {
        // Check for cancellation before each PNG
        if file_tracker.is_cancelled() {
            return Err(FavisError::user_cancelled());
        }
        
        if let Some(pb) = progress {
            pb.set_message(format!(
                "{} {}x{}",
                "Creating PNG".cyan().bold(),
                size.to_string().yellow(),
                size.to_string().yellow()
            ));
        }
        save_resized_png(&img, size, out_dir, &mut file_tracker)?;
    }

    // Generate ICO if requested
    if !ico_sizes.is_empty() {
        // Check for cancellation before ICO generation
        if file_tracker.is_cancelled() {
            return Err(FavisError::user_cancelled());
        }
        
        if let Some(pb) = progress {
            pb.set_message(format!(
                "{}",
                "Creating ICO file with multiple sizes...".cyan().bold()
            ));
        }

        let mut icon_dir = IconDir::new(ResourceType::Icon);
        for &size in ico_sizes {
            // Check for cancellation during ICO size processing
            if file_tracker.is_cancelled() {
                return Err(FavisError::user_cancelled());
            }
            
            if let Some(pb) = progress {
                pb.set_message(format!(
                    "{} {}x{}",
                    "Adding size to ICO:".cyan().bold(),
                    size.to_string().yellow(),
                    size.to_string().yellow()
                ));
            }
            let rgba = get_rgba_for_ico(&img, size);
            let icon_image = IconImage::from_rgba_data(size, size, rgba);
            // encode_png returns Result<IconDirEntry, _>, so handle error and add entry
            let entry = ico::IconDirEntry::encode(&icon_image)
                .map_err(|_| FavisError::processing_error(format!("Cannot encode {}x{} icon for ICO", size, size)))?;
            icon_dir.add_entry(entry);
        }

        let mut ico_path = PathBuf::from(out_dir);
        ico_path.push("favicon.ico");

        // Track ICO file for cleanup if cancelled
        file_tracker.track(ico_path.clone());

        if let Some(pb) = progress {
            pb.set_message(format!("{}", "Writing favicon.ico file...".cyan().bold()));
        }

        let mut file = BufWriter::new(File::create(&ico_path)
            .map_err(|_| FavisError::write_error(format!("Cannot create ICO file: {}", ico_path.display())))?);
        icon_dir.write(&mut file)
            .map_err(|_| FavisError::write_error("Cannot write ICO file data"))?;
    }

    // If we get here, processing completed successfully - don't cleanup files
    std::mem::forget(file_tracker);
    
    Ok(())
}
