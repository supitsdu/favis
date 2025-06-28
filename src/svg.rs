//! SVG rendering to PNG using resvg.

use crate::error::{FavisError, Result};
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{self, Tree};

/// Render SVG data to a pixmap at the specified size.
pub fn render_svg(
    svg_data: &[u8],
    width: u32,
    height: u32,
    progress: Option<&ProgressBar>,
) -> Result<Pixmap> {
    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Parsing SVG data...".cyan().bold()));
    }

    let opt = usvg::Options::default();
    let tree = Tree::from_data(svg_data, &opt)?;

    if let Some(pb) = progress {
        pb.set_message(format!(
            "{} {}x{} {}",
            "Rendering SVG to".cyan().bold(),
            width.to_string().yellow(),
            height.to_string().yellow(),
            "pixels...".cyan().bold()
        ));
    }

    let mut pixmap = Pixmap::new(width, height).ok_or_else(|| {
        FavisError::processing_error(format!(
            "Cannot create {}x{} pixmap - insufficient memory",
            width, height
        ))
    })?;

    resvg::render(&tree, usvg::Transform::default(), &mut pixmap.as_mut());

    if let Some(pb) = progress {
        pb.set_message(format!(
            "{} {}",
            "SVG rendering".cyan().bold(),
            "complete".green().bold()
        ));
    }

    Ok(pixmap)
}

/// Get the original dimensions of an SVG file
pub fn get_svg_dimensions(svg_data: &[u8]) -> Result<(u32, u32)> {
    let opt = usvg::Options::default();
    let tree = Tree::from_data(svg_data, &opt)?;

    let size = tree.size();
    Ok((size.width() as u32, size.height() as u32))
}

/// Render SVG data to a pixmap at its original size.
pub fn render_svg_original_size(svg_data: &[u8], progress: Option<&ProgressBar>) -> Result<Pixmap> {
    let (width, height) = get_svg_dimensions(svg_data)?;
    render_svg(svg_data, width, height, progress)
}

/// Extension trait for Pixmap operations
pub trait PixmapExt {
    fn to_dynamic_image(&self) -> Result<image::DynamicImage>;
}

impl PixmapExt for Pixmap {
    fn to_dynamic_image(&self) -> Result<image::DynamicImage> {
        // Convert Pixmap to image::DynamicImage
        let width = self.width();
        let height = self.height();
        let data = self.data();

        // Create an RgbaImage from the pixmap data
        let img = image::RgbaImage::from_raw(width, height, data.to_vec()).ok_or_else(|| {
            FavisError::processing_error("Cannot convert pixmap data to image format")
        })?;

        Ok(image::DynamicImage::ImageRgba8(img))
    }
}
