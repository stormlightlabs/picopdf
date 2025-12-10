# picopdf

A minimalist markdown to PDF converter with custom font support.

## Usage

Convert markdown to PDF using default fonts:

```bash
picopdf write -i input.md -o output.pdf
```

Use custom TrueType fonts:

```bash
picopdf write -i input.md -o output.pdf \
  --font-regular ~/fonts/regular.ttf \
  --font-bold ~/fonts/bold.ttf \
  --font-mono ~/fonts/mono.ttf
```

## Architecture

Four-stage markdown conversion pipeline:

1. Markdown to AST/IR using pulldown-cmark
2. AST to styled blocks with semantic styling
3. Blocks to positioned lines and glyphs via layout engine
4. Lines to PDF using pdf-writer with embedded fonts

## Font Support

Default fonts are built-in PDF Type1 fonts (Helvetica, Courier) requiring no embedding.
Custom TrueType/OpenType fonts are fully embedded in the PDF with proper Unicode support.

Default PDFs are around pretty small (2-5KB), see this file as an [example](./README.pdf).
Custom font PDFs include the full font files, typically 2-7MB depending on font complexity.

## Supported Markdown

- Headings (H1-H4)
- Paragraphs
- Lists
- Code blocks
- Inline code

## Example

```bash
# Using 0xProto Nerd Font
picopdf write -i README.md -o README.pdf \
  --font-regular ~/.local/share/fonts/0xProto-Regular.ttf \
  --font-bold ~/.local/share/fonts/0xProto-Bold.ttf \
  --font-mono ~/.local/share/fonts/0xProtoMono-Regular.ttf
```
