use pulldown_cmark::{Event, Parser, Tag, TagEnd};

/// Intermediate representation of the markdown document structure.
///
/// This IR captures the semantic structure without styling or layout concerns.
#[derive(Debug, Clone)]
pub enum IRNode {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    CodeBlock { lang: Option<String>, code: String },
    ListItem { text: String },
    Text { content: String },
}

/// Parses markdown text into an intermediate representation using pulldown-cmark.
///
/// This stage focuses purely on extracting document structure, deferring all styling and layout decisions to later pipeline stages.
pub fn parse_markdown(markdown: &str) -> Vec<IRNode> {
    let parser = Parser::new(markdown);
    let mut nodes = Vec::new();
    let mut current_text = String::new();
    let mut in_heading = None;
    let mut in_paragraph = false;
    let mut in_code_block = false;
    let mut code_lang = None;
    let mut in_list_item = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    in_heading = Some(level as u8);
                    current_text.clear();
                }
                Tag::Paragraph => {
                    in_paragraph = true;
                    current_text.clear();
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    code_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => Some(lang.to_string()),
                        pulldown_cmark::CodeBlockKind::Indented => None,
                    };
                    current_text.clear();
                }
                Tag::Item => {
                    in_list_item = true;
                    current_text.clear();
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {
                    if let Some(level) = in_heading.take() {
                        nodes.push(IRNode::Heading { level, text: current_text.clone() });
                        current_text.clear();
                    }
                }
                TagEnd::Paragraph => {
                    if in_paragraph {
                        nodes.push(IRNode::Paragraph { text: current_text.clone() });
                        current_text.clear();
                        in_paragraph = false;
                    }
                }
                TagEnd::CodeBlock => {
                    if in_code_block {
                        nodes.push(IRNode::CodeBlock { lang: code_lang.take(), code: current_text.clone() });
                        current_text.clear();
                        in_code_block = false;
                    }
                }
                TagEnd::Item => {
                    if in_list_item {
                        nodes.push(IRNode::ListItem { text: current_text.clone() });
                        current_text.clear();
                        in_list_item = false;
                    }
                }
                _ => {}
            },
            Event::Text(text) => current_text.push_str(&text),
            Event::Code(code) => current_text.push_str(&code),
            Event::SoftBreak | Event::HardBreak => current_text.push(' '),
            _ => {}
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let markdown = "# Heading 1\n## Heading 2";
        let nodes = parse_markdown(markdown);

        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            IRNode::Heading { level, text } => {
                assert_eq!(*level, 1);
                assert_eq!(text, "Heading 1");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_parse_paragraph() {
        let markdown = "This is a paragraph.";
        let nodes = parse_markdown(markdown);

        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            IRNode::Paragraph { text } => {
                assert_eq!(text, "This is a paragraph.");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_parse_code_block() {
        let markdown = "```rust\nfn main() {}\n```";
        let nodes = parse_markdown(markdown);

        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            IRNode::CodeBlock { lang, code } => {
                assert_eq!(lang.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}\n");
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn test_parse_list() {
        let markdown = "- Item 1\n- Item 2";
        let nodes = parse_markdown(markdown);

        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            IRNode::ListItem { text } => {
                assert_eq!(text, "Item 1");
            }
            _ => panic!("Expected list item"),
        }
    }
}
