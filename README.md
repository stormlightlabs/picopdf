# picopdf

A minimalist markdown to PDF converter with custom font support.

## Usage

### Markdown to PDF

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

### PDF Manipulation Tools

Merge multiple PDFs:

```bash
picopdf tool merge file1.pdf file2.pdf file3.pdf -o merged.pdf
```

Split PDF into individual pages:

```bash
picopdf tool split -i input.pdf -o ./output_dir
```

Split PDF by page ranges:

```bash
picopdf tool split -i input.pdf -o ./output_dir -r "1-3,5,7-9"
```

Extract specific pages:

```bash
picopdf tool extract -i input.pdf -o output.pdf -p "1,3,5-7"
```

Delete specific pages:

```bash
picopdf tool delete -i input.pdf -o output.pdf -p "2,4,6-8"
```

Rotate all pages 90 degrees:

```bash
picopdf tool rotate -i input.pdf -o output.pdf
```

Rotate specific pages by custom angle:

```bash
picopdf tool rotate -i input.pdf -o output.pdf -a 180 -p "1,3,5"
```

Reorganize pages in custom order:

```bash
picopdf tool organize -i input.pdf -o output.pdf -n "3,1,2,4-6"
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
