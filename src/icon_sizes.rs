//! Centralized icon size definitions for wfig.

/// PNG icon sizes for ATI (Android, Tablet, iOS) only.
pub const PNG_SIZES_ATI: &[u32] = &[48, 72, 96, 128, 192, 256, 384, 512];

/// PNG icon sizes for all platforms.
pub const PNG_SIZES_ALL: &[u32] = &[16, 32, 48, 72, 96, 128, 256, 384, 512];

/// ICO icon sizes (Windows multi-resolution icons).
pub const ICO_SIZES: &[u32] = &[16, 32, 48, 64, 96, 128, 256];
