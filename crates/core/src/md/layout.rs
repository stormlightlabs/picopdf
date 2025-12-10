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

/// A page containing positioned lines.
///
/// Represents a single PDF page with its content.
#[derive(Debug, Clone)]
pub struct Page {
    pub lines: Vec<PositionedLine>,
}

/// Layout engine that converts styled blocks into positioned lines.
///
/// Handles text measurement, line breaking, and vertical positioning while preparing content for final PDF rendering.
pub struct LayoutEngine {
    page_width: f32,
    page_height: f32,
    margin: f32,
    current_y: f32,
    pages: Vec<Page>,
}

impl LayoutEngine {
    fn new(page_width: f32, page_height: f32, margin: f32) -> Self {
        Self { page_width, page_height, margin, current_y: margin, pages: vec![Page { lines: Vec::new() }] }
    }

    fn should_page_break(&self, additional_height: f32) -> bool {
        self.current_y + additional_height > self.page_height - self.margin
    }

    fn usable_height(&self) -> f32 {
        self.page_height - (2.0 * self.margin)
    }

    fn new_page(&mut self) {
        self.current_y = self.margin;
        self.pages.push(Page { lines: Vec::new() });
    }

    fn current_page_mut(&mut self) -> &mut Page {
        self.pages.last_mut().expect("LayoutEngine always has a page")
    }

    fn char_width(&self, style: &Style) -> f32 {
        if style.is_monospace { style.font_size * 0.6 } else { style.font_size * 0.5 }
    }

    fn measure_block_height(&self, block: &StyledBlock) -> f32 {
        let max_width = self.page_width - (2.0 * self.margin);
        let char_width = self.char_width(&block.style);
        let mut line_width = 0.0;
        let mut line_count = 0;

        for word in block.content.split_whitespace() {
            let word_width = word.len() as f32 * char_width;
            let space_width = if line_width == 0.0 { 0.0 } else { char_width };

            if line_width > 0.0 && line_width + space_width + word_width > max_width {
                line_count += 1;
                line_width = 0.0;
            }

            if line_width > 0.0 {
                line_width += space_width;
            }

            line_width += word_width;
        }

        if line_width > 0.0 {
            line_count += 1;
        }

        block.style.margin_top + block.style.margin_bottom + (line_count as f32 * block.style.line_height)
    }

    fn layout_block(&mut self, block: &StyledBlock, block_height: f32) {
        let is_page_start = (self.current_y - self.margin).abs() < f32::EPSILON;
        if self.should_page_break(block_height) && !is_page_start {
            self.new_page();
        }
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
                self.add_line(&current_line, &block.style);
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
            self.add_line(&current_line, &block.style);
        }

        self.current_y += block.style.margin_bottom;
    }

    fn add_line(&mut self, text: &str, style: &Style) {
        if text.is_empty() {
            return;
        }

        if self.should_page_break(style.line_height) {
            self.new_page();
        }

        let line = self.create_line(text, style, self.current_y);
        self.current_page_mut().lines.push(line);
        self.current_y += style.line_height;
    }

    fn create_line(&self, text: &str, style: &Style, y: f32) -> PositionedLine {
        let char_width = self.char_width(style);
        let mut glyphs = Vec::new();
        let mut x = self.margin;

        for ch in text.chars() {
            glyphs.push(Glyph {
                ch,
                x,
                y,
                font_size: style.font_size,
                is_bold: style.is_bold,
                is_monospace: style.is_monospace,
            });
            x += char_width;
        }

        PositionedLine { glyphs, y, height: style.line_height }
    }
}

/// Layouts styled blocks into pages with positioned lines.
///
/// Performs line breaking, vertical positioning, and pagination to prepare content for final rendering to PDF format.
pub fn layout_blocks(blocks: &[StyledBlock]) -> Vec<Page> {
    let mut engine = LayoutEngine::new(595.0, 842.0, 50.0);
    let mut idx = 0;

    while idx < blocks.len() {
        let block = &blocks[idx];
        let block_height = engine.measure_block_height(block);

        if block.keep_with_next
            && let Some(next_block) = blocks.get(idx + 1)
        {
            let next_height = engine.measure_block_height(next_block);
            let combined_height = block_height + next_height;
            if combined_height <= engine.usable_height() && engine.should_page_break(combined_height) {
                engine.new_page();
            }
        }

        engine.layout_block(block, block_height);
        idx += 1;
    }

    engine.pages
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
            keep_with_next: false,
        }];

        let pages = layout_blocks(&blocks);
        assert_eq!(pages.len(), 1);
        assert!(!pages[0].lines[0].glyphs.is_empty());
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
            keep_with_next: false,
        }];

        let pages = layout_blocks(&blocks);
        let total_lines: usize = pages.iter().map(|p| p.lines.len()).sum();
        assert!(total_lines > 1, "Long text should break into multiple lines");
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
            keep_with_next: false,
        }];

        let pages = layout_blocks(&blocks);
        assert_eq!(pages[0].lines[0].glyphs.len(), 2);
        assert!(
            pages[0].lines[0].glyphs[1].x > pages[0].lines[0].glyphs[0].x,
            "Second glyph should be positioned after first"
        );
    }

    #[test]
    fn test_keep_with_next_moves_block() {
        let filler = StyledBlock {
            content: "filler".to_string(),
            style: Style {
                font_size: 12.0,
                line_height: 700.0,
                margin_top: 0.0,
                margin_bottom: 0.0,
                is_bold: false,
                is_monospace: false,
            },
            is_list_item: false,
            keep_with_next: false,
        };

        let heading = StyledBlock {
            content: "KEEP_WITH_NEXT".to_string(),
            style: Style {
                font_size: 20.0,
                line_height: 24.0,
                margin_top: 10.0,
                margin_bottom: 6.0,
                is_bold: true,
                is_monospace: false,
            },
            is_list_item: false,
            keep_with_next: true,
        };

        let list = StyledBlock {
            content: "- bullet".to_string(),
            style: Style {
                font_size: 12.0,
                line_height: 16.0,
                margin_top: 2.0,
                margin_bottom: 2.0,
                is_bold: false,
                is_monospace: false,
            },
            is_list_item: true,
            keep_with_next: false,
        };

        let pages = layout_blocks(&[filler, heading, list]);
        assert!(
            pages.len() >= 2,
            "Expect filler to consume first page so heading moves to next page"
        );
        let first_page_text: String = pages[0]
            .lines
            .iter()
            .flat_map(|line| line.glyphs.iter().map(|g| g.ch))
            .collect();
        assert!(
            !first_page_text.contains("KEEP_WITH_NEXT"),
            "Heading should have been moved to next page"
        );
    }
}
