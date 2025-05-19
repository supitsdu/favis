// src/link.rs

//! Generate HTML <link> tags or JSON metadata from a webmanifest

use anyhow::{Context, Result};
use crate::icon_sizes::{get_all_sizes, IconPurpose};
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::{fs::{self, File}, io::Write};

/// Minimal representation of the 'icons' array in webmanifest
#[derive(Debug, Deserialize)]
struct Manifest {
    icons: Vec<IconEntry>,
}

#[derive(Debug, Deserialize)]
struct IconEntry {
    src: String,
    sizes: Option<String>,
    #[serde(rename = "type")]
    mime_type: Option<String>,
    #[allow(dead_code)]
    purpose: Option<String>,
}

/// Represents a <link> tag for favicon
#[derive(Debug)]
struct LinkTag {
    rel: &'static str,
    href: String,
    sizes: Option<String>,
    type_attr: Option<String>,
}

impl LinkTag {
    /// Formats as HTML <link ... />
    fn to_html(&self) -> String {
        let mut parts = vec![format!("rel=\"{}\"", self.rel)];
        parts.push(format!("href=\"{}\"", self.href));
        if let Some(s) = &self.sizes {
            parts.push(format!("sizes=\"{}\"", s));
        }
        if let Some(t) = &self.type_attr {
            parts.push(format!("type=\"{}\"", t));
        }
        format!("<link {}/>", parts.join(" "))
    }
}

/// Reads a manifest, builds link tags, and returns HTML snippet
pub fn generate_links_from_manifest(
    manifest_path: &str,
    base_url: Option<&str>,
) -> Result<String> {
    // Read and parse manifest file
    let raw = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read manifest `{}`", manifest_path))?;
    let manifest: Manifest = serde_json::from_str(&raw)
        .context("Invalid JSON in manifest.webmanifest")?;

    // Load all known icon sizes and build a lookup by size string
    let known_sizes = get_all_sizes();
    let mut size_map = std::collections::HashMap::new();
    for icon_size in &known_sizes {
        let size_str = format!("{}x{}", icon_size.size, icon_size.size);
        size_map.insert(size_str, icon_size.purposes.clone());
    }

    // Prepare tags, dedupe by (rel,sizes)
    let mut seen = std::collections::HashSet::new();
    let mut tags = Vec::new();

    for icon in manifest.icons {
        // Build href with optional base
        let href = if let Some(base) = base_url {
            format!("{}/{}", base.trim_end_matches('/'), icon.src.trim_start_matches('/'))
        } else {
            icon.src.clone()
        };

        // Determine rel using icon_sizes.rs metadata
        let rel = if let Some(sizes) = &icon.sizes {
            match size_map.get(sizes) {
                Some(purposes) if purposes.contains(&IconPurpose::AppleTouch) => "apple-touch-icon",
                Some(purposes) if purposes.contains(&IconPurpose::Favicon) => {
                    if icon.src.to_lowercase().ends_with(".ico") {
                        "shortcut icon"
                    } else {
                        "icon"
                    }
                }
                Some(purposes) if purposes.contains(&IconPurpose::Android) => "icon",
                Some(purposes) if purposes.contains(&IconPurpose::PWA) => "icon",
                _ => "icon",
            }
        } else if icon.src.to_lowercase().ends_with(".ico") {
            "shortcut icon"
        } else {
            "icon"
        };

        let sizes = icon.sizes.clone();
        let type_attr = icon.mime_type.clone();

        // Dedup
        let key = (rel, sizes.clone());
        if seen.insert(key) {
            tags.push(LinkTag { rel, href, sizes, type_attr });
        }
    }

    // Generate HTML
    // Sort tags by rel priority, then by numeric size
    const REL_PRIORITY: &[&str] = &["shortcut icon", "icon", "apple-touch-icon"];
    tags.sort_by(|a, b| {
        // Compare rel priority
        let a_rel = REL_PRIORITY.iter().position(|&r| r == a.rel).unwrap_or(usize::MAX);
        let b_rel = REL_PRIORITY.iter().position(|&r| r == b.rel).unwrap_or(usize::MAX);
        a_rel.cmp(&b_rel)
            // If rel is equal, compare numeric size parsed from "NxN"
            .then_with(|| {
                let parse_size = |s: &Option<String>| {
                    s.as_deref()
                        .and_then(|sz| sz.split('x').next())
                        .and_then(|n| n.parse::<u32>().ok())
                        .unwrap_or(0)
                };
                parse_size(&a.sizes).cmp(&parse_size(&b.sizes))
            })
    });

    let mut html = String::new();
    for tag in &tags {
        html.push_str(&tag.to_html());
        html.push('\n');
    }
    Ok(html)
}

/// Public API: Generate HTML link tags from manifest and write to file if requested
pub fn generate_links(
    manifest_path: &str,
    base_url: Option<&str>,
    output_path: Option<&str>,
    progress: Option<&ProgressBar>,
) -> Result<()> {
    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Reading manifest...".cyan().bold()));
    }
    let html = generate_links_from_manifest(manifest_path, base_url)?;

    if let Some(pb) = progress {
        pb.set_message(format!("{}", "Generating HTML link tags...".cyan().bold()));
    }

    if let Some(path) = output_path {
        if let Some(pb) = progress {
            pb.set_message(format!("{} {}", "Writing HTML to".cyan().bold(), path.yellow()));
        }
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        if let Some(pb) = progress {
            pb.set_message(format!("{}", "HTML link tags written successfully.".green().bold()));
        }
    } else {
        // Print to stdout if no output file specified
        println!("{}", html);
    }
    Ok(())
}
