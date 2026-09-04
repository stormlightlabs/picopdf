# To-do

See [SPEC.md](SPEC.md) for behavior and acceptance criteria and [ROADMAP.md](ROADMAP.md) for dependencies and sequence.

## Foundation

- [ ] Bring format, tests, and clippy checks to green.
- [ ] Add `py/picopdf-docling/` as a locked uv package with the internal sidecar console script.
- [ ] Add a locked Nix flake for the Rust package, Python package, checks, and development shells.
- [ ] Expose the locked Python package in the default development shell without creating a second picopdf CLI.
- [ ] Add an opt-in full development shell as OCR, rendering, compression, and office backends ship.
- [ ] Add a minimal direnv `.envrc` that loads the flake without installing packages or downloading models.
- [ ] Define explicit model setup for the sidecar on Nix, macOS, Linux, and WSL.
- [ ] Test the compiled CLI, including streams, exit codes, help, color, quiet mode, paths, and failures.
- [ ] Preserve current command lines and document the compatibility policy.

## Read from PDF

- [ ] Add the versioned Document/Page/Block IR with source-page and bounding-box provenance.
- [ ] Use Docling's Python API in `picopdf-docling` and normalize it to the versioned picopdf IR.
- [ ] Add the Rust process client for the versioned `picopdf-docling` protocol; do not invoke uv or Python directly.
- [ ] Render normalized IR as Markdown and JSON in Rust.
- [ ] Produce `document.md`, `document.json`, `manifest.json`, and referenced figure assets as a bundle.
- [ ] Support one-page and contiguous page-range selection.
- [ ] Support formula enrichment and optional picture assets.
- [ ] Stream clean Markdown or JSON to stdout.
- [ ] Add a native reader for supported born-digital PDFs.
- [ ] Add tested, explainable automatic backend selection after the native reader is reliable.

## Write to PDF

- [x] Convert Markdown to PDF with built-in or custom fonts.
- [ ] Convert JPEG, PNG, BMP, GIF, and TIFF images to PDF, including documented multi-frame behavior.
- [ ] Convert DOC/DOCX to PDF through a LibreOffice adapter.
- [ ] Convert XLS/XLSX to PDF through a LibreOffice adapter.
- [ ] Convert PPT/PPTX to PDF through a LibreOffice adapter.

## PDF tools

- [x] Merge multiple PDFs.
- [x] Split a PDF into pages or page ranges.
- [x] Rotate selected or all pages.
- [x] Delete pages.
- [x] Extract pages into a new PDF.
- [x] Reorder pages with organize.
- [ ] Render PDF pages as JPEG with a `pdftocairo` adapter.
- [ ] Add a searchable text layer with an OCRmyPDF adapter.
- [ ] Add lossless structural compression with qpdf.
- [ ] Add strong, explicitly lossy compression with Ghostscript.

## Editable office exports

These formats require the research gates in roadmap milestone 7. A PDF does not contain the original office document model, so “editable” must be tested rather than assumed.

- [ ] Research and prototype PDF/IR to DOCX; preserve editable content and reading order.
- [ ] Research and prototype PDF/IR tables to XLSX; preserve cell text, spans, and provenance.
- [ ] Research and prototype PDF/IR to PPTX; require separate editable objects rather than page images.

## Distribution and documentation

- [ ] Document external backend installation, tested versions, licenses, operating-system support, and fidelity limits.
- [ ] Build standalone macOS and Linux releases and smoke-test the Linux release under WSL.
- [ ] Add shell completions after CLI names stabilize.
- [ ] Evaluate version-matched man pages.
