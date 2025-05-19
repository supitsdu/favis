# 🚀 Favis – Favicon and Web Icon Generator

**favis** is a user-friendly CLI tool designed to effortlessly create favicons, app icons, and web manifests from a single source image.

---

## ✨ What is favis?

**favis** simplifies generating icons for modern websites and Progressive Web Apps (PWAs). With just one command, you can:

* ✅ Generate optimized PNG favicons in all necessary sizes
* 🎯 Create multi-size `favicon.ico` files
* 📱 Produce web manifest files ideal for PWAs
* 🔗 Output organized HTML `<link>` tags for easy integration

---

## 📖 Quick Start Guide

### 🖼️ Step 1: Generate Your Icons

Create icons from your source image (**SVG recommended** for best results):

```bash
# Basic icon generation
favis generate logo.svg

# Full set with manifest, saved to 'public' directory
favis generate logo.svg --coverage extended --manifest --output ./public

# Using PNG image (if SVG is unavailable)
favis generate logo.png --raster-ok
```

### 📝 Step 2: Generate HTML Tags

Generate HTML `<link>` tags to include in your website:

```bash
# Print tags to terminal (default)
favis link ./public/manifest.webmanifest

# Save tags to an HTML file with URL prefix
favis link ./public/manifest.webmanifest --base /assets/icons --output ./public/favicon-links.html
```

---

## 🎉 Features

### 📦 Flexible Icon Sets

Choose your preferred icon coverage level:

| Level          | Description                                 | Speed    |
| -------------- | ------------------------------------------- | -------- |
| 🚩 Required    | Essential icons                             | Fastest  |
| 🔖 Recommended | Balanced for wide compatibility (default)   | Moderate |
| 🌐 Extended    | Complete icon set for maximum compatibility | Slowest  |

### 🔍 Optimized SVG Icons

Using SVG images ensures:

* ✨ Crisp quality at all sizes
* 📱 Optimal appearance on high-resolution displays
* 🔄 Minimal loss of quality

### 📃 Automated Web Manifests

favis makes PWA manifest generation easy:

* 🤖 Automatically handles icon definitions
* 💾 Preserves existing custom fields
* 📑 Generates fully compliant web manifest files

### 🔗 Simple HTML Integration

Generate accurate and ready-to-use HTML `<link>` tags:

* ✅ Correct `rel` attributes automatically set
* 🗂️ Tags logically grouped and organized
* 📌 Supports custom base URL prefixes

---

## 📏 Icon Sizes

| Coverage       | Icon Sizes                                                                                                                     |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| 🚩 Required    | `16×16` (Browser tabs), `32×32` (Modern browsers), `180×180` (Apple Touch), `192×192` (Android/PWA)                                    |
| 🔖 Recommended | Required sizes + `48×48`, `128×128` (Additional browser), `76×76`, `120×120`, `152×152` (Apple), `96×96` (Google TV), `512×512` (PWA splash) |
| 🌐 Extended    | Recommended sizes + `57×57`, `72×72`, `114×114`, `144×144` (Legacy Apple), `64×64`, `256×256` (Windows), `384×384` (Extra PWA)               |

---

## 💡 Best Practices

* 🌟 Always use SVG images when available.
* 🛠️ Generate web manifests with `--manifest` for PWAs.
* ⚡ Use the `link` command to quickly integrate icons into your website.

---

## 📜 License

MIT License

---

## 🤝 Contribute to favis

We welcome contributions! Submit pull requests to help improve **favis** and make web icons simpler for everyone. 🚀
