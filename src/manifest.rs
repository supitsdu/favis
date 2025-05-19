//! Web manifest generator for PWA icons.

use anyhow::Result;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, path::Path};

/// Icon entry in the webmanifest
#[derive(Serialize)]
struct ManifestIcon {
    src: String,
    sizes: String,
    #[serde(rename = "type")]
    mime_type: String,
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

/// Generates a `manifest.webmanifest` in `out_dir` using provided PNG sizes.
pub fn generate_manifest(out_dir: &str, sizes: &[u32], progress: Option<&ProgressBar>) -> Result<()> {
    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Creating web manifest...".cyan().bold()));
    }
    
    let icons: Vec<ManifestIcon> = sizes
        .iter()
        .map(|&s| ManifestIcon {
            src: format!("favicon-{}x{}.png", s, s),
            sizes: format!("{}x{}", s, s),
            mime_type: "image/png".into(),
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