//! Web manifest generator for PWA icons.

use anyhow::Result;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use serde::{Serialize, Deserialize};
use std::{fs, path::Path, collections::HashMap};

use crate::icon_sizes::{IconPriority, IconPurpose, filter_by_priority};

/// Icon entry in the webmanifest
#[derive(Serialize, Deserialize)]
struct ManifestIcon {
    src: String,
    sizes: String,
    #[serde(rename = "type")]
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<String>,
}

/// Simplified webmanifest structure
#[derive(Serialize, Deserialize)]
struct Manifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    icons: Vec<ManifestIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    // Preserve any additional fields
    #[serde(flatten)]
    additional_fields: HashMap<String, serde_json::Value>,
}

/// Map our internal purpose to PWA manifest purpose
fn map_purpose_to_manifest(purpose: &IconPurpose) -> Option<&'static str> {
    match purpose {
        IconPurpose::PWA => Some("any"),
        IconPurpose::Android => Some("maskable"),
        _ => None
    }
}

/// Try to read an existing manifest file
fn read_existing_manifest(path: &Path) -> Result<Option<Manifest>> {
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let manifest: Manifest = serde_json::from_str(&content)?;
        Ok(Some(manifest))
    } else {
        Ok(None)
    }
}

/// Generates or updates a `manifest.webmanifest` in `out_dir` using provided priority level.
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

    let path = Path::new(out_dir).join("manifest.webmanifest");
    
    // Try to read existing manifest
    let mut manifest = match read_existing_manifest(&path)? {
        Some(existing) => {
            if let Some(pb) = progress {
                pb.set_message(format!("{}", "Updating existing manifest...".cyan().bold()));
            }
            existing
        },
        None => {
            if let Some(pb) = progress {
                pb.set_message(format!("{}", "Creating new minimal manifest...".cyan().bold()));
            }
            Manifest {
                name: None,
                short_name: None,
                icons: vec![],
                start_url: None,
                display: None,
                theme_color: None,
                background_color: None,
                additional_fields: HashMap::new(),
            }
        }
    };
    
    // Update only the icons field
    manifest.icons = icons;

    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Writing manifest.webmanifest...".cyan().bold()));
    }
    
    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(path, json)?;
    
    Ok(())
}