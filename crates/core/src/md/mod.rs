/// Stage 1: Markdown -> AST/IR (structure)
mod parser;

/// Stage 2: AST -> styled blocks (semantics/theme)
mod styler;

/// Stage 3: Blocks -> positioned lines & glyphs (layout)
mod layout;

/// Stage 4: Lines -> PDF (backend)
mod renderer;

/// Font configuration and loading
mod font;

pub use font::{FontConfig, FontSource};
pub use layout::{LayoutEngine, Page, PositionedLine};
pub use parser::parse_markdown;
pub use renderer::render_to_pdf;
pub use styler::{Style, StyledBlock};

use std::io;

/// Converts markdown text to PDF bytes through the four-stage pipeline.
///
/// Pipeline stages:
/// 1. Parse markdown into intermediate representation
/// 2. Apply styling and semantics to create styled blocks
/// 3. Layout blocks into positioned lines and glyphs
/// 4. Render positioned content to PDF format
///
/// Uses default built-in PDF fonts (Helvetica, Courier).
pub fn markdown_to_pdf(markdown: &str) -> io::Result<Vec<u8>> {
    markdown_to_pdf_with_fonts(markdown, &FontConfig::default())
}

/// Converts markdown text to PDF bytes with custom font configuration.
///
/// Allows specification of custom fonts for regular, bold, and monospace text.
/// Falls back to built-in PDF fonts if custom fonts are not provided.
pub fn markdown_to_pdf_with_fonts(markdown: &str, fonts: &FontConfig) -> io::Result<Vec<u8>> {
    let ir = parse_markdown(markdown);
    let styled = styler::apply_styles(ir);
    let positioned = layout::layout_blocks(&styled);
    renderer::render_to_pdf_with_fonts(&positioned, fonts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_pipeline() {
        let markdown = "# Title\n\nThis is a paragraph.\n\n## Subtitle\n\n- Item 1\n- Item 2";
        let result = markdown_to_pdf(markdown);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(pdf.len() > 200, "Complete PDF should have substantial content");
    }

    #[test]
    fn test_pipeline_with_code() {
        let markdown = "# Code Example\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let result = markdown_to_pdf(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_empty() {
        let markdown = "";
        let result = markdown_to_pdf(markdown);
        assert!(result.is_ok());
    }
}
