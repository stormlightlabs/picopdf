use crate::md::styler::{Style, StyledBlock};

/// A positioned glyph with its exact location and styling.
///
/// Represents a single character after layout, ready for rendering.
#[derive(Debug, Clone)]
pub struct Glyph {
    pub ch: char,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_monospace: bool,
}

/// A line of positioned glyphs.
///
/// Represents a single line of text after line breaking and positioning.
#[derive(Debug, Clone)]
pub struct PositionedLine {
    pub glyphs: Vec<Glyph>,
    pub y: f32,
    pub height: f32,
}

/// Layout engine that converts styled blocks into positioned lines.
///
/// Handles text measurement, line breaking, and vertical positioning while preparing content for final PDF rendering.
pub struct LayoutEngine {
    page_width: f32,
    margin: f32,
    current_y: f32,
}

impl LayoutEngine {
    fn new(page_width: f32, _page_height: f32, margin: f32) -> Self {
        Self { page_width, margin, current_y: margin }
    }

    fn char_width(&self, style: &Style) -> f32 {
        if style.is_monospace { style.font_size * 0.6 } else { style.font_size * 0.5 }
    }

    fn layout_block(&mut self, block: &StyledBlock) -> Vec<PositionedLine> {
        let mut lines = Vec::new();
        let max_width = self.page_width - (2.0 * self.margin);
        let char_width = self.char_width(&block.style);

        self.current_y += block.style.margin_top;

        let words: Vec<&str> = block.content.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_width = 0.0;

        for word in words {
            let word_width = word.len() as f32 * char_width;
            let space_width = char_width;

            if line_width + word_width + space_width > max_width && !current_line.is_empty() {
                lines.push(self.create_line(&current_line, &block.style));
                self.current_y += block.style.line_height;
                current_line.clear();
                line_width = 0.0;
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                line_width += space_width;
            }

            current_line.push_str(word);
            line_width += word_width;
        }

        if !current_line.is_empty() {
            lines.push(self.create_line(&current_line, &block.style));
            self.current_y += block.style.line_height;
        }

        self.current_y += block.style.margin_bottom;
        lines
    }

    fn create_line(&self, text: &str, style: &Style) -> PositionedLine {
        let char_width = self.char_width(style);
        let mut glyphs = Vec::new();
        let mut x = self.margin;

        for ch in text.chars() {
            glyphs.push(Glyph {
                ch,
                x,
                y: self.current_y,
                font_size: style.font_size,
                is_bold: style.is_bold,
                is_monospace: style.is_monospace,
            });
            x += char_width;
        }

        PositionedLine { glyphs, y: self.current_y, height: style.line_height }
    }
}

/// Layouts styled blocks into positioned lines with glyphs.
///
/// Performs line breaking and vertical positioning to prepare content for final rendering to PDF format.
pub fn layout_blocks(blocks: &[StyledBlock]) -> Vec<PositionedLine> {
    let mut engine = LayoutEngine::new(595.0, 842.0, 50.0);
    let mut all_lines = Vec::new();

    for block in blocks {
        let lines = engine.layout_block(block);
        all_lines.extend(lines);
    }

    all_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md::styler::{Style, StyledBlock};

    #[test]
    fn test_simple_layout() {
        let blocks = vec![StyledBlock {
            content: "Hello world".to_string(),
            style: Style {
                font_size: 12.0,
                line_height: 16.0,
                margin_top: 0.0,
                margin_bottom: 0.0,
                is_bold: false,
                is_monospace: false,
            },
            is_list_item: false,
        }];

        let lines = layout_blocks(&blocks);
        assert_eq!(lines.len(), 1);
        assert!(!lines[0].glyphs.is_empty());
    }

    #[test]
    fn test_line_breaking() {
        let long_text =
            "This is a very long line that should be broken into multiple lines when laid out on the page".to_string();
        let blocks = vec![StyledBlock {
            content: long_text,
            style: Style {
                font_size: 12.0,
                line_height: 16.0,
                margin_top: 0.0,
                margin_bottom: 0.0,
                is_bold: false,
                is_monospace: false,
            },
            is_list_item: false,
        }];

        let lines = layout_blocks(&blocks);
        assert!(lines.len() > 1, "Long text should break into multiple lines");
    }

    #[test]
    fn test_glyph_positioning() {
        let blocks = vec![StyledBlock {
            content: "AB".to_string(),
            style: Style {
                font_size: 12.0,
                line_height: 16.0,
                margin_top: 0.0,
                margin_bottom: 0.0,
                is_bold: false,
                is_monospace: false,
            },
            is_list_item: false,
        }];

        let lines = layout_blocks(&blocks);
        assert_eq!(lines[0].glyphs.len(), 2);
        assert!(
            lines[0].glyphs[1].x > lines[0].glyphs[0].x,
            "Second glyph should be positioned after first"
        );
    }
}
