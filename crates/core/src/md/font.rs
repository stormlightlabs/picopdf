use std::fs;
use std::path::Path;
use ttf_parser::Face;

/// Font configuration for PDF rendering.
///
/// Allows customization of fonts used in the PDF output while providing
/// sensible defaults that work without external dependencies.
#[derive(Debug, Clone)]
pub struct FontConfig {
    pub regular: FontSource,
    pub bold: FontSource,
    pub monospace: FontSource,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            regular: FontSource::BuiltIn("Helvetica"),
            bold: FontSource::BuiltIn("Helvetica-Bold"),
            monospace: FontSource::BuiltIn("Courier"),
        }
    }
}

/// Source of a font, either built-in PDF font or external file.
///
/// Built-in fonts are part of the PDF standard and always available.
/// Custom fonts must be embedded in the PDF from external files.
#[derive(Debug, Clone)]
pub enum FontSource {
    BuiltIn(&'static str),
    TrueType {
        path: String,
        data: Vec<u8>,
        info: FontInfo,
    },
}

/// Parsed information from a TrueType font.
///
/// Contains metrics and metadata needed for PDF embedding.
#[derive(Debug, Clone)]
pub struct FontInfo {
    pub postscript_name: String,
    pub ascent: i16,
    pub descent: i16,
    pub cap_height: i16,
    pub units_per_em: u16,
    pub bbox: (i16, i16, i16, i16),
}

impl FontConfig {
    /// Creates a font configuration with custom font files.
    ///
    /// Loads font files from disk. If a font file cannot be loaded,
    /// returns an error with details about which file failed.
    pub fn from_files(regular: Option<&Path>, bold: Option<&Path>, monospace: Option<&Path>) -> Result<Self, String> {
        let mut config = Self::default();

        if let Some(path) = regular {
            config.regular = load_font_file(path, "regular")?;
        }

        if let Some(path) = bold {
            config.bold = load_font_file(path, "bold")?;
        }

        if let Some(path) = monospace {
            config.monospace = load_font_file(path, "monospace")?;
        }

        Ok(config)
    }
}

/// Loads a font file from disk and parses it.
///
/// Returns a FontSource::TrueType with the loaded data and parsed metrics,
/// or an error if the file cannot be read or parsed.
fn load_font_file(path: &Path, font_type: &str) -> Result<FontSource, String> {
    let data =
        fs::read(path).map_err(|e| format!("Failed to load {} font from '{}': {}", font_type, path.display(), e))?;

    let face = Face::parse(&data, 0)
        .map_err(|e| format!("Failed to parse {} font from '{}': {:?}", font_type, path.display(), e))?;

    let postscript_name = face
        .names()
        .into_iter()
        .find(|n| n.name_id == ttf_parser::name_id::POST_SCRIPT_NAME)
        .and_then(|n| n.to_string())
        .unwrap_or_else(|| "CustomFont".to_string());

    let info = FontInfo {
        postscript_name,
        ascent: face.ascender(),
        descent: face.descender(),
        cap_height: face.capital_height().unwrap_or_else(|| face.ascender()),
        units_per_em: face.units_per_em(),
        bbox: (
            face.global_bounding_box().x_min,
            face.global_bounding_box().y_min,
            face.global_bounding_box().x_max,
            face.global_bounding_box().y_max,
        ),
    };

    Ok(FontSource::TrueType { path: path.display().to_string(), data, info })
}
