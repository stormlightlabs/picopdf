use crate::md::font::{FontConfig, FontSource};
use crate::md::layout::Page;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use std::io;

/// Renders pages to PDF format using pdf-writer.
///
/// This final stage takes fully positioned glyphs and generates the PDF
/// byte stream with proper font embedding and text rendering operators.
///
/// Uses default built-in PDF fonts.
pub fn render_to_pdf(pages: &[Page]) -> io::Result<Vec<u8>> {
    render_to_pdf_with_fonts(pages, &FontConfig::default())
}

/// Renders pages to PDF format with custom fonts.
///
/// Supports both built-in PDF fonts and custom font files.
/// Custom fonts are embedded in the PDF as TrueType fonts.
pub fn render_to_pdf_with_fonts(pages: &[Page], fonts: &FontConfig) -> io::Result<Vec<u8>> {
    let mut pdf = Pdf::new();

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let font_regular_id = Ref::new(3);
    let font_bold_id = Ref::new(4);
    let font_mono_id = Ref::new(5);

    let mut next_id = 6;
    let mut page_ids = Vec::new();
    let mut content_ids = Vec::new();

    for _ in 0..pages.len() {
        page_ids.push(Ref::new(next_id));
        next_id += 1;
        content_ids.push(Ref::new(next_id));
        next_id += 1;
    }

    pdf.catalog(catalog_id).pages(page_tree_id);

    pdf.pages(page_tree_id)
        .kids(page_ids.iter().copied())
        .count(pages.len() as i32);

    embed_font_source(&mut pdf, font_regular_id, &fonts.regular);
    embed_font_source(&mut pdf, font_bold_id, &fonts.bold);
    embed_font_source(&mut pdf, font_mono_id, &fonts.monospace);

    for (page_idx, page_content) in pages.iter().enumerate() {
        let page_id = page_ids[page_idx];
        let content_id = content_ids[page_idx];

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

        let mut content = Content::new();

        for line in &page_content.lines {
            if line.glyphs.is_empty() {
                continue;
            }

            let text: String = line.glyphs.iter().map(|g| g.ch).collect();
            let first_glyph = &line.glyphs[0];

            let (font_name, font_source) = if first_glyph.is_bold {
                (Name(b"F2"), &fonts.bold)
            } else if first_glyph.is_monospace {
                (Name(b"F3"), &fonts.monospace)
            } else {
                (Name(b"F1"), &fonts.regular)
            };

            content.begin_text();
            content.set_font(font_name, first_glyph.font_size);
            content.set_text_matrix([1.0, 0.0, 0.0, 1.0, first_glyph.x, 842.0 - first_glyph.y]);

            match font_source {
                FontSource::BuiltIn(_) => {
                    content.show(Str(text.as_bytes()));
                }
                FontSource::TrueType { .. } => {
                    let utf16: Vec<u16> = text.encode_utf16().collect();
                    let mut bytes = Vec::new();
                    for code in utf16 {
                        bytes.push((code >> 8) as u8);
                        bytes.push(code as u8);
                    }
                    content.show(Str(&bytes));
                }
            }

            content.end_text();
        }

        pdf.stream(content_id, &content.finish());
    }

    Ok(pdf.finish())
}

/// Embeds a font into the PDF document.
///
/// Handles both built-in Type1 fonts and custom TrueType fonts.
/// Built-in fonts don't require embedding, while custom fonts are
/// fully embedded in the PDF.
fn embed_font_source(pdf: &mut Pdf, font_id: Ref, source: &FontSource) {
    match source {
        FontSource::BuiltIn(name) => {
            pdf.type1_font(font_id).base_font(Name(name.as_bytes()));
        }
        FontSource::TrueType { data, info, .. } => {
            embed_truetype_font(pdf, font_id, data, info);
        }
    }
}

/// Embeds a TrueType font into the PDF.
///
/// Creates the necessary PDF objects: font descriptor, CIDFont, and Type0 font.
fn embed_truetype_font(pdf: &mut Pdf, font_id: Ref, data: &[u8], info: &crate::md::font::FontInfo) {
    let descriptor_id = Ref::new(font_id.get() + 100);
    let cid_font_id = Ref::new(font_id.get() + 200);
    let font_stream_id = Ref::new(font_id.get() + 300);
    let cid_to_gid_id = Ref::new(font_id.get() + 400);

    let mut type0 = pdf.type0_font(font_id);
    type0.base_font(Name(info.postscript_name.as_bytes()));
    type0.encoding_predefined(Name(b"Identity-H"));
    type0.descendant_font(cid_font_id);
    type0.to_unicode(cid_to_gid_id);
    type0.finish();

    let mut cid_font = pdf.cid_font(cid_font_id);
    cid_font.subtype(pdf_writer::types::CidFontType::Type2);
    cid_font.base_font(Name(info.postscript_name.as_bytes()));
    cid_font.system_info(pdf_writer::types::SystemInfo {
        registry: Str(b"Adobe"),
        ordering: Str(b"Identity"),
        supplement: 0,
    });
    cid_font.font_descriptor(descriptor_id);
    cid_font.default_width(1000.0);
    let mut widths = cid_font.widths();
    widths.same(0, 255, 500.0);
    widths.finish();
    cid_font.finish();

    let mut descriptor = pdf.font_descriptor(descriptor_id);
    descriptor.name(Name(info.postscript_name.as_bytes()));
    descriptor.flags(pdf_writer::types::FontFlags::SYMBOLIC);
    descriptor.bbox(pdf_writer::Rect::new(
        info.bbox.0 as f32,
        info.bbox.1 as f32,
        info.bbox.2 as f32,
        info.bbox.3 as f32,
    ));
    descriptor.italic_angle(0.0);
    descriptor.ascent(info.ascent as f32);
    descriptor.descent(info.descent as f32);
    descriptor.cap_height(info.cap_height as f32);
    descriptor.stem_v(80.0);
    descriptor.font_file2(font_stream_id);
    descriptor.finish();

    let mut font_stream = pdf.stream(font_stream_id, data);
    font_stream.filter(pdf_writer::Filter::FlateDecode);
    font_stream.finish();

    let cmap = create_identity_cmap();
    pdf.stream(cid_to_gid_id, cmap.as_bytes());
}

/// Creates an identity CMap for Unicode to CID mapping.
///
/// This simple CMap maps each Unicode code point to the same CID value.
fn create_identity_cmap() -> String {
    r#"/CIDInit /ProcSet findresource begin
12 dict begin
begincmap
/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def
/CMapName /Adobe-Identity-UCS def
/CMapType 2 def
1 begincodespacerange
<0000> <FFFF>
endcodespacerange
1 beginbfrange
<0000> <FFFF> <0000>
endbfrange
endcmap
CMapName currentdict /CMap defineresource pop
end
end"#
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md::layout::{Glyph, Page, PositionedLine};

    #[test]
    fn test_render_empty() {
        let pages = vec![];
        let result = render_to_pdf(&pages);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(!pdf.is_empty(), "PDF should have basic structure even when empty");
    }

    #[test]
    fn test_render_single_line() {
        let pages = vec![Page {
            lines: vec![PositionedLine {
                glyphs: vec![
                    Glyph { ch: 'H', x: 50.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: false },
                    Glyph { ch: 'i', x: 56.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: false },
                ],
                y: 50.0,
                height: 16.0,
            }],
        }];

        let result = render_to_pdf(&pages);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(pdf.len() > 100, "PDF with content should be larger");
    }

    #[test]
    fn test_render_with_different_fonts() {
        let pages = vec![Page {
            lines: vec![PositionedLine {
                glyphs: vec![
                    Glyph { ch: 'A', x: 50.0, y: 50.0, font_size: 12.0, is_bold: true, is_monospace: false },
                    Glyph { ch: 'B', x: 56.0, y: 50.0, font_size: 12.0, is_bold: false, is_monospace: true },
                ],
                y: 50.0,
                height: 16.0,
            }],
        }];

        let result = render_to_pdf(&pages);
        assert!(result.is_ok());
    }
}
