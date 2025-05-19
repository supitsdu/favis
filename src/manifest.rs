//! Web manifest generator for PWA icons.

use anyhow::Result;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, path::Path};

use crate::icon_sizes::{IconPriority, IconPurpose, IconSize, filter_by_purpose, filter_by_priority};

/// Icon entry in the webmanifest
#[derive(Serialize)]
struct ManifestIcon {
    src: String,
    sizes: String,
    #[serde(rename = "type")]
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<String>,
}

/// Simplified webmanifest structure
#[derive(Serialize)]
struct Manifest {
    name: String,
    short_name: String,
    icons: Vec<ManifestIcon>,
    start_url: String,
    display: String,
    theme_color: String,
    background_color: String,
}

/// Map our internal purpose to PWA manifest purpose
fn map_purpose_to_manifest(purpose: &IconPurpose) -> Option<&'static str> {
    match purpose {
        IconPurpose::PWA => Some("any"),
        IconPurpose::Android => Some("maskable"),
        _ => None
    }
}

/// Generates a `manifest.webmanifest` in `out_dir` using provided priority level.
pub fn generate_manifest(out_dir: &str, priority: IconPriority, progress: Option<&ProgressBar>) -> Result<()> {
    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Creating web manifest...".cyan().bold()));
    }
    
    // Get all icon sizes for the requested priority level
    let icon_sizes = filter_by_priority(priority);
    
    // Create manifest icons with proper purpose values
    let icons: Vec<ManifestIcon> = icon_sizes
        .iter()
        .map(|size| {
            // Find the primary purpose for the manifest
            let purpose = size.purposes.iter()
                .find_map(map_purpose_to_manifest)
                .map(String::from);
                
            ManifestIcon {
                src: format!("favicon-{}x{}.png", size.size, size.size),
                sizes: format!("{}x{}", size.size, size.size),
                mime_type: "image/png".into(),
                purpose,
            }
        })
        .collect();

    let manifest = Manifest {
        name: "My App".into(),
        short_name: "App".into(),
        icons,
        start_url: "/".into(),
        display: "standalone".into(),
        theme_color: "#ffffff".into(),
        background_color: "#ffffff".into(),
    };

    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Writing manifest.webmanifest...".cyan().bold()));
    }
    
    let json = serde_json::to_string_pretty(&manifest)?;
    let path = Path::new(out_dir).join("manifest.webmanifest");
    fs::write(path, json)?;
    
    Ok(())
}