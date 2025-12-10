use crate::md::layout::PositionedLine;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use std::io;

/// Renders positioned lines to PDF format using pdf-writer.
///
/// This final stage takes fully positioned glyphs and generates the PDF
/// byte stream with proper font embedding and text rendering operators.
pub fn render_to_pdf(lines: &[PositionedLine]) -> io::Result<Vec<u8>> {
    let mut pdf = Pdf::new();

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let content_id = Ref::new(4);
    let font_regular_id = Ref::new(5);
    let font_bold_id = Ref::new(6);
    let font_mono_id = Ref::new(7);

    pdf.catalog(catalog_id).pages(page_tree_id);

    pdf.pages(page_tree_id).kids([page_id]).count(1);

    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);
    let mut resources = page.resources();
    resources
        .fonts()
        .pair(Name(b"F1"), font_regular_id)
        .pair(Name(b"F2"), font_bold_id)
        .pair(Name(b"F3"), font_mono_id);
    resources.finish();
    page.finish();

    embed_font(&mut pdf, font_regular_id, "Helvetica");
    embed_font(&mut pdf, font_bold_id, "Helvetica-Bold");
    embed_font(&mut pdf, font_mono_id, "Courier");

    let mut content = Content::new();

    for line in lines {
        if line.glyphs.is_empty() {
            continue;
        }

        let text: String = line.glyphs.iter().map(|g| g.ch).collect();
        let first_glyph = &line.glyphs[0];

        let font_name = if first_glyph.is_bold {
            Name(b"F2")
        } else if first_glyph.is_monospace {
            Name(b"F3")
        } else {
            Name(b"F1")
        };

        content.begin_text();
        content.set_font(font_name, first_glyph.font_size);
        content.set_text_matrix([1.0, 0.0, 0.0, 1.0, first_glyph.x, 842.0 - first_glyph.y]);
        content.show(Str(text.as_bytes()));
        content.end_text();
    }

    pdf.stream(content_id, &content.finish());

    Ok(pdf.finish())
}

/// Embeds a standard PDF font into the document.
///
/// Uses Type1 fonts which are part of the PDF standard and don't require external font files to be embedded.
fn embed_font(pdf: &mut Pdf, font_id: Ref, base_font: &str) {
    pdf.type1_font(font_id).base_font(Name(base_font.as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md::layout::{Glyph, PositionedLine};

    #[test]
    fn test_render_empty() {
        let lines = vec![];
        let result = render_to_pdf(&lines);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(!pdf.is_empty(), "PDF should have basic structure even when empty");
    }

    #[test]
    fn test_render_single_line() {
        let lines = vec![PositionedLine {
            glyphs: vec![
                Glyph { ch: 'H', x: 50.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: false },
                Glyph { ch: 'i', x: 56.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: false },
            ],
            y: 50.0,
            height: 16.0,
        }];

        let result = render_to_pdf(&lines);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(pdf.len() > 100, "PDF with content should be larger");
    }

    #[test]
    fn test_render_with_different_fonts() {
        let lines = vec![PositionedLine {
            glyphs: vec![
                Glyph { ch: 'A', x: 50.0, y: 50.0, font_size: 12.0, is_bold: true, is_monospace: false },
                Glyph { ch: 'B', x: 56.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: true },
            ],
            y: 50.0,
            height: 16.0,
        }];

        let result = render_to_pdf(&lines);
        assert!(result.is_ok());
    }
}
