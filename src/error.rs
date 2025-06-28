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
        let context = path.into();
        Self::new(context, Some("Check the file path and ensure the file exists".to_string()))
    }

    pub fn permission_denied(path: impl Into<String>) -> Self {
        let context = path.into();
        Self::new(context, Some("Try running with elevated permissions or check file/directory permissions".to_string()))
    }

    pub fn invalid_format(details: impl Into<String>) -> Self {
        let context = details.into();
        Self::new(context, Some("Provide an SVG file (recommended) or PNG file with --raster-ok flag".to_string()))
    }

    pub fn image_too_small(min_size: u32) -> Self {
        let context = format!("Image should be at least {}x{} pixels", min_size, min_size);
        Self::new(context, Some("Use a larger source image or an SVG for better quality at all sizes".to_string()))
    }

    pub fn invalid_svg(reason: impl Into<String>) -> Self {
        let context = reason.into();
        Self::new(context, Some("Check SVG syntax or try a different SVG file".to_string()))
    }

    pub fn write_error(path: impl Into<String>) -> Self {
        let context = path.into();
        Self::new(context, Some("Ensure the output directory exists and is writable".to_string()))
    }

    pub fn processing_error(details: impl Into<String>) -> Self {
        let context = details.into();
        let suggestion = if context.contains("memory") || context.contains("allocation") {
            Some("Try with a smaller image or close other applications to free memory".to_string())
        } else {
            Some("Check system resources and try again".to_string())
        };
        Self::new(context, suggestion)
    }

    pub fn user_cancelled() -> Self {
        Self::new(
            "Operation cancelled by user",
            Some("Partial files may have been created and will be cleaned up".to_string())
        )
    }

    // TODO: Use for signal-based interruption handling
    #[allow(dead_code)]
    pub fn interrupted(details: impl Into<String>) -> Self {
        let context = format!("Process interrupted: {}", details.into());
        Self::new(context, Some("Cleaning up temporary files and partial outputs".to_string()))
    }

    /// Display user-friendly error message with colors and suggestions
    pub fn display_friendly(&self) {
        eprintln!("{}: {}", "Error".red().bold(), self.context);
        if let Some(suggestion) = &self.suggestion {
            eprintln!("{}: {}", "Suggestion".cyan().bold(), suggestion);
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
