use crate::md::parser::IRNode;

/// Style properties applied to text.
///
/// Encapsulates visual styling decisions separate from document structure.
#[derive(Debug, Clone)]
pub struct Style {
    pub font_size: f32,
    pub line_height: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub is_bold: bool,
    pub is_monospace: bool,
}

impl Style {
    fn heading(level: u8) -> Self {
        let font_size = match level {
            1 => 24.0,
            2 => 20.0,
            3 => 16.0,
            _ => 14.0,
        };

        Self {
            font_size,
            line_height: font_size * 1.2,
            margin_top: font_size * 0.8,
            margin_bottom: font_size * 0.4,
            is_bold: true,
            is_monospace: false,
        }
    }

    fn paragraph() -> Self {
        Self {
            font_size: 12.0,
            line_height: 16.0,
            margin_top: 6.0,
            margin_bottom: 6.0,
            is_bold: false,
            is_monospace: false,
        }
    }

    fn code_block() -> Self {
        Self {
            font_size: 10.0,
            line_height: 14.0,
            margin_top: 8.0,
            margin_bottom: 8.0,
            is_bold: false,
            is_monospace: true,
        }
    }

    fn list_item() -> Self {
        Self {
            font_size: 12.0,
            line_height: 16.0,
            margin_top: 2.0,
            margin_bottom: 2.0,
            is_bold: false,
            is_monospace: false,
        }
    }
}

/// A block of content with associated styling.
///
/// Represents the semantic unit after style application but before layout.
#[derive(Debug, Clone)]
pub struct StyledBlock {
    pub content: String,
    pub style: Style,
    pub is_list_item: bool,
    pub is_code_block: bool,
    pub keep_with_next: bool,
}

/// Applies semantic styling to IR nodes, converting structure to styled blocks.
///
/// This stage maps document semantics (headings, code, etc.) to visual styles while remaining independent of specific layout or rendering concerns.
pub fn apply_styles(nodes: Vec<IRNode>) -> Vec<StyledBlock> {
    nodes
        .into_iter()
        .map(|node| match node {
            IRNode::Heading { level, text } => StyledBlock {
                content: text,
                style: Style::heading(level),
                is_list_item: false,
                is_code_block: false,
                keep_with_next: true,
            },
            IRNode::Paragraph { text } => StyledBlock {
                content: text,
                style: Style::paragraph(),
                is_list_item: false,
                is_code_block: false,
                keep_with_next: false,
            },
            IRNode::CodeBlock { code, .. } => StyledBlock {
                content: code,
                style: Style::code_block(),
                is_list_item: false,
                is_code_block: true,
                keep_with_next: false,
            },
            IRNode::ListItem { text } => StyledBlock {
                content: format!("- {}", text),
                style: Style::list_item(),
                is_list_item: true,
                is_code_block: false,
                keep_with_next: false,
            },
            IRNode::Text { content } => StyledBlock {
                content,
                style: Style::paragraph(),
                is_list_item: false,
                is_code_block: false,
                keep_with_next: false,
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md::parser::IRNode;

    #[test]
    fn test_heading_style() {
        let nodes = vec![IRNode::Heading { level: 1, text: "Title".to_string() }];
        let blocks = apply_styles(nodes);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content, "Title");
        assert_eq!(blocks[0].style.font_size, 24.0);
        assert!(blocks[0].style.is_bold);
    }

    #[test]
    fn test_paragraph_style() {
        let nodes = vec![IRNode::Paragraph { text: "Content".to_string() }];
        let blocks = apply_styles(nodes);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content, "Content");
        assert_eq!(blocks[0].style.font_size, 12.0);
        assert!(!blocks[0].style.is_bold);
    }

    #[test]
    fn test_code_block_style() {
        let nodes = vec![IRNode::CodeBlock { lang: Some("rust".to_string()), code: "let x = 5;".to_string() }];
        let blocks = apply_styles(nodes);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content, "let x = 5;");
        assert!(blocks[0].style.is_monospace);
    }

    #[test]
    fn test_list_item_style() {
        let nodes = vec![IRNode::ListItem { text: "First item".to_string() }];
        let blocks = apply_styles(nodes);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content, "- First item");
        assert!(blocks[0].is_list_item);
    }
}
