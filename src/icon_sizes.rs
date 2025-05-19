//! Centralized icon size definitions with priority and purpose metadata.

/// Icon priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPriority {
    /// Level 1: Required sizes only (minimal set)
    Required = 1,
    /// Level 2: Recommended sizes (good compatibility)
    Recommended = 2,
    /// Level 3: All sizes (maximum compatibility)
    Extended = 3,
}

/// Icon purpose/usage type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconPurpose {
    /// Standard favicons for browsers
    Favicon,
    /// Apple Touch Icons for iOS
    AppleTouch,
    /// Android homescreen icons
    Android,
    /// Windows Tiles/Icons
    Windows,
    /// Generic PWA icon
    PWA,
}

/// Icon size definition with metadata.
#[derive(Debug, Clone)]
pub struct IconSize {
    /// Size in pixels (square)
    pub size: u32,
    /// Primary purpose(s) of this icon
    pub purposes: Vec<IconPurpose>,
    /// Priority level (1-3)
    pub priority: IconPriority,
    /// Human-readable description of where this icon is used
    #[allow(dead_code)]
    pub description: &'static str,
}

/// Get all defined icon sizes.
pub fn get_all_sizes() -> Vec<IconSize> {
    vec![
        // Priority 1: Required sizes
        IconSize {
            size: 16,
            purposes: vec![IconPurpose::Favicon],
            priority: IconPriority::Required,
            description: "Classic favicon, browser tabs",
        },
        IconSize {
            size: 32,
            purposes: vec![IconPurpose::Favicon],
            priority: IconPriority::Required,
            description: "Standard favicon for modern browsers",
        },
        IconSize {
            size: 180,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Required,
            description: "Apple Touch Icon for iPhone (retina display)",
        },
        IconSize {
            size: 192,
            purposes: vec![IconPurpose::Android, IconPurpose::PWA],
            priority: IconPriority::Required,
            description: "Android homescreen icon",
        },

        // Priority 2: Recommended sizes
        IconSize {
            size: 48,
            purposes: vec![IconPurpose::Favicon],
            priority: IconPriority::Recommended,
            description: "Windows site icon",
        },
        IconSize {
            size: 76,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Recommended,
            description: "Apple Touch Icon for iPad (non-retina)",
        },
        IconSize {
            size: 120,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Recommended,
            description: "Apple Touch Icon for iPhone (X/Plus)",
        },
        IconSize {
            size: 152,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Recommended,
            description: "Apple Touch Icon for iPad, iPad mini",
        },
        IconSize {
            size: 96,
            purposes: vec![IconPurpose::Android],
            priority: IconPriority::Recommended,
            description: "Google TV icon",
        },
        IconSize {
            size: 128,
            purposes: vec![IconPurpose::Favicon],
            priority: IconPriority::Recommended,
            description: "Chrome Web Store icon",
        },
        IconSize {
            size: 512,
            purposes: vec![IconPurpose::PWA],
            priority: IconPriority::Recommended,
            description: "PWA splash screen icon",
        },

        // Priority 3: Extended sizes
        IconSize {
            size: 57,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Extended,
            description: "Apple Touch Icon (older iPhone, pre-retina)",
        },
        IconSize {
            size: 72,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Extended,
            description: "Apple Touch Icon (older iPad, pre-retina)",
        },
        IconSize {
            size: 114,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Extended,
            description: "Apple Touch Icon (older iPhone, retina)",
        },
        IconSize {
            size: 144,
            purposes: vec![IconPurpose::AppleTouch],
            priority: IconPriority::Extended,
            description: "Apple Touch Icon (older iPad, retina)",
        },
        IconSize {
            size: 64,
            purposes: vec![IconPurpose::Favicon],
            priority: IconPriority::Extended,
            description: "Windows site icon (medium)",
        },
        IconSize {
            size: 256,
            purposes: vec![IconPurpose::Favicon, IconPurpose::Windows],
            priority: IconPriority::Extended,
            description: "Windows site icon (large)",
        },
        IconSize {
            size: 384,
            purposes: vec![IconPurpose::PWA],
            priority: IconPriority::Extended,
            description: "PWA icon (large)",
        },
    ]
}

/// Filter sizes by priority level (inclusive).
///
/// Returns all sizes with priority <= the given level:
/// - Level 1: Required sizes only
/// - Level 2: Required + Recommended sizes
/// - Level 3: All sizes (Required + Recommended + Extended)
pub fn filter_by_priority(priority: IconPriority) -> Vec<IconSize> {
    get_all_sizes()
        .into_iter()
        .filter(|size| (size.priority as u8) <= (priority as u8))
        .collect()
}

/// Get PNG sizes based on priority level.
pub fn get_png_sizes(priority: IconPriority) -> Vec<u32> {
    filter_by_priority(priority)
        .into_iter()
        .map(|size| size.size)
        .collect()
}

/// Get ICO sizes based on priority level.
pub fn get_ico_sizes(priority: IconPriority) -> Vec<u32> {
    filter_by_priority(priority)
        .into_iter()
        .filter(|size| {
            size.purposes.contains(&IconPurpose::Favicon)
                && size.size <= 256 // ICO format supports up to 256x256
        })
        .map(|size| size.size)
        .collect()
}

/// Filter sizes by purpose.
#[allow(dead_code)]
pub fn filter_by_purpose(purpose: IconPurpose, priority: IconPriority) -> Vec<IconSize> {
    filter_by_priority(priority)
        .into_iter()
        .filter(|size| size.purposes.contains(&purpose))
        .collect()
}
