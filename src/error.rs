//! Centralized error handling with helpful recovery suggestions.

use owo_colors::OwoColorize;
use std::fmt;

/// Custom error type with context and recovery suggestions
#[derive(Debug)]
pub struct FavisError {
    pub context: String,
    pub suggestion: Option<String>,
}

impl FavisError {
    pub fn new(context: impl Into<String>, suggestion: Option<String>) -> Self {
        Self {
            context: context.into(),
            suggestion,
        }
    }

    pub fn file_not_found(path: impl Into<String>) -> Self {
        let context = format!("Hmm, can't find that file: {}", path.into());
        Self::new(context, Some("Double-check the path and make sure the file exists!".to_string()))
    }

    pub fn permission_denied(path: impl Into<String>) -> Self {
        let context = format!("Permission denied: {}", path.into());
        Self::new(context, Some("Try running with elevated permissions or check file/directory permissions.".to_string()))
    }

    pub fn invalid_format(details: impl Into<String>) -> Self {
        let context = details.into();
        let suggestion = if context.contains("isn't supported") {
            "Try an SVG (best option!) or a PNG with --raster-ok."
        } else if context.contains("--raster-ok flag") {
            "Add --raster-ok to use PNG files (quality might not be perfect at larger sizes)."
        } else {
            "Use an SVG file for best results, or PNG with --raster-ok."
        };
        Self::new(context, Some(suggestion.to_string()))
    }

    pub fn image_too_small(min_size: u32) -> Self {
        let context = format!("Oops! Image is too small - needs to be at least {}x{} pixels", min_size, min_size);
        Self::new(context, Some("Try a larger source image or use an SVG for crisp results at any size!".to_string()))
    }

    pub fn invalid_svg(reason: impl Into<String>) -> Self {
        let context = format!("SVG trouble: {}", reason.into());
        Self::new(context, Some("Check the SVG syntax or try a different SVG file.".to_string()))
    }

    pub fn write_error(path: impl Into<String>) -> Self {
        let context = format!("Can't write to: {}", path.into());
        Self::new(context, Some("Make sure the output directory exists and you have write permissions.".to_string()))
    }

    pub fn processing_error(details: impl Into<String>) -> Self {
        let context = format!("Processing hiccup: {}", details.into());
        let suggestion = if context.contains("memory") || context.contains("allocation") {
            Some("Try with a smaller image or close other apps to free up memory.".to_string())
        } else {
            Some("Check system resources and give it another shot!".to_string())
        };
        Self::new(context, suggestion)
    }

    pub fn user_cancelled() -> Self {
        Self::new(
            "Operation cancelled by user",
            Some("All partial files have been cleaned up automatically.".to_string())
        )
    }

    // TODO: Use for signal-based interruption handling
    #[allow(dead_code)]
    pub fn interrupted(details: impl Into<String>) -> Self {
        let context = format!("Process interrupted: {}", details.into());
        Self::new(context, Some("Cleaning up temporary files and partial outputs".to_string()))
    }

    /// Display user-friendly error message with colors and symbols
    pub fn display_friendly(&self) {
        eprintln!("{} {}", "✗".red().bold(), self.context.red().bold());
        if let Some(suggestion) = &self.suggestion {
            eprintln!("{} {}", "💡".yellow().bold(), suggestion.yellow());
        }
    }
}

impl fmt::Display for FavisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl std::error::Error for FavisError {}

impl From<std::io::Error> for FavisError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => FavisError::file_not_found(err.to_string()),
            std::io::ErrorKind::PermissionDenied => FavisError::permission_denied(err.to_string()),
            _ => FavisError::processing_error(err.to_string()),
        }
    }
}

impl From<image::ImageError> for FavisError {
    fn from(err: image::ImageError) -> Self {
        match err {
            image::ImageError::IoError(io_err) => FavisError::from(io_err),
            image::ImageError::Limits(_) => FavisError::image_too_small(512),
            image::ImageError::Unsupported(_) => FavisError::invalid_format(err.to_string()),
            _ => FavisError::processing_error(err.to_string()),
        }
    }
}

impl From<resvg::usvg::Error> for FavisError {
    fn from(err: resvg::usvg::Error) -> Self {
        FavisError::invalid_svg(err.to_string())
    }
}

impl From<serde_json::Error> for FavisError {
    fn from(err: serde_json::Error) -> Self {
        FavisError::invalid_format(format!("JSON parsing error: {}", err))
    }
}

/// Helper macro for creating context-aware errors
#[allow(unused_macros)]
macro_rules! context_error {
    ($kind:expr, $msg:expr) => {
        FavisError::new($kind, $msg)
    };
    ($kind:expr, $msg:expr, $($arg:tt)*) => {
        FavisError::new($kind, format!($msg, $($arg)*))
    };
}

#[allow(unused_imports)]
pub(crate) use context_error;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, FavisError>;
