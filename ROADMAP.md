# picopdf roadmap

This roadmap sequences the work in [SPEC.md](SPEC.md). Milestones are dependency order, not release dates. Each milestone must leave the workspace releasable and pass its exit criterion before dependent work starts. Nix and direnv reproduce the development environment; release checks verify the same CLI outside Nix on macOS, Linux, and WSL.

## Milestone 1: Establish a reliable CLI boundary

**Exit criterion:** Existing commands retain their syntax, CLI behavior is covered by integration tests, and `direnv` loads a pinned Nix development shell where `picopdf-docling --protocol-version` works without model downloads.

- [ ] Fix the current clippy failures in the Markdown parser. — blocked by: none
- [ ] Add small generated PDF fixtures and CLI integration-test support. — blocked by: none
- [ ] Scaffold `py/picopdf-docling/` as a uv package with a console script, protocol-version command, tests, and an exact supported Docling dependency. — blocked by: none
- [ ] Commit the sidecar's `uv.lock` and prove its basic command works through `uv run --locked`. — blocked by: Python package scaffold
- [ ] Spike the locked Python package in Nix with uv2nix and a nixpkgs Python overlay; record any required overrides. — blocked by: Python lock
- [ ] Add a locked flake with a standalone picopdf package, lightweight Rust shell, and Rust checks. — blocked by: clippy fixes
- [ ] Add the default development shell with Rust and the locked Python package, plus a minimal `.envrc` using `use flake .#default`. — blocked by: Rust flake; Python/Nix spike
- [ ] Ensure flake evaluation, shell entry, and protocol-version checks do not install packages, run inference, or download models. — blocked by: default development shell
- [ ] Move status and diagnostics to stderr; reserve stdout for primary streamed data. — blocked by: CLI integration-test support
- [ ] Replace UTF-8 path unwraps with `Path`/`OsStr` handling and test supported non-UTF-8 paths. — blocked by: CLI integration-test support
- [ ] Add global `--version`, `--quiet`, and `--no-color`; honor `NO_COLOR`, `TERM=dumb`, and terminal detection. — blocked by: CLI integration-test support
- [ ] Add concise default help, useful examples, backend installation hints, and the project support URL. — blocked by: CLI stream behavior
- [ ] Define and test exit codes 0, 1, 2, and 3. — blocked by: CLI integration-test support
- [ ] Document the public CLI compatibility policy in the README. — blocked by: stabilized CLI behavior

Verification:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
nix flake check
nix develop -c picopdf-docling --protocol-version
```

## Milestone 2: Ship structured PDF reading with the Python sidecar

**Exit criterion:** A user with `picopdf-docling` installed can convert representative PDFs or page ranges into a validated bundle, clean Markdown on stdout, or clean JSON on stdout.

- [ ] Define schema version 1 of the Document/Page/Block IR and check in its JSON schema. — blocked by: milestone 1
- [ ] Add shared Docling-document and expected-IR fixtures for reading order, lists, equations, tables, figures, unknown items, and coordinate origins. — blocked by: IR schema
- [ ] Implement and test Docling API conversion and IR normalization in `py/picopdf-docling/`. — blocked by: shared fixtures; Python package scaffold
- [ ] Render and test the IR as Markdown in Rust with page and bounding-box provenance. — blocked by: IR schema
- [ ] Add a fake-sidecar harness for protocol versions, success, failure, timeout, argv capture, and malformed output. — blocked by: milestone 1
- [ ] Implement the Rust process-protocol client without invoking a shell, Python interpreter, uv, or Docling directly. — blocked by: fake-sidecar harness; IR schema
- [ ] Validate the sidecar protocol version and IR schema before accepting output; reject incompatibilities with an upgrade hint. — blocked by: process-protocol client
- [ ] Implement `picopdf-docling models download` and offline conversion; document licenses and cache behavior for macOS, Linux, WSL, and the Nix shell. — blocked by: Python conversion flow
- [ ] Implement `picopdf read` with `--input`, `--output`, `--backend`, `--pages`, `--formula`, `--pictures`, `--timeout`, `--allow-model-downloads`, `--json`, and `--force`. — blocked by: process-protocol client; Python conversion flow; Markdown renderer
- [ ] Write bundles through a staging directory and protect existing bundle files. — blocked by: `picopdf read`
- [ ] Generate deterministic asset names and a manifest with source, sidecar, Docling, model, options, files, and warnings. — blocked by: bundle writer
- [ ] Test the real sidecar through `nix develop`, then install it with `uv tool install` outside Nix and repeat the process test. Prove that offline mode cannot download models and manually review the fixtures. — blocked by: complete read flow; model setup
- [ ] Add non-Nix release builds and backend smoke tests for Linux and macOS; smoke-test the Linux build under WSL. — blocked by: complete read flow
- [ ] Update README usage, installation, protocol compatibility, limits, bundle schema, and examples. — blocked by: reviewed read flow

Verification:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
uv run --project py/picopdf-docling --locked pytest
picopdf-docling --protocol-version
cargo run -- read --input tests/fixtures/textbook.pdf --pages 1-3 --output /tmp/picopdf-book
cargo run -- read --input tests/fixtures/textbook.pdf --pages 1-3 --output -
cargo run -- read --input tests/fixtures/textbook.pdf --pages 1-3 --json --output - | jq .
```

## Milestone 3: Add clear PDF extraction and accessibility tools

**Exit criterion:** Users can render pages to JPEG and add a searchable text layer through documented external backends, with predictable files and actionable dependency errors.

- [ ] Add Poppler and OCRmyPDF to an opt-in Nix development shell without changing the default shell. — blocked by: locked flake from milestone 1
- [ ] Implement `picopdf tool render --format jpg` with a `pdftocairo` adapter. — blocked by: external-process behavior from milestone 2
- [ ] Support page selection, 150 PPI default resolution, validated JPEG quality, stable filenames, and safe overwrite behavior. — blocked by: render adapter
- [ ] Implement `picopdf tool ocr` with an OCRmyPDF adapter. — blocked by: external-process behavior from milestone 2
- [ ] Expose language, deskew, and force/reprocess choices without passing arbitrary backend arguments. — blocked by: OCR adapter
- [ ] Add fake-backend tests and opt-in real-backend smoke tests for both tools. — blocked by: render and OCR interfaces
- [ ] Document that OCR creates a searchable PDF while `read` creates structured content. — blocked by: completed tool behavior

Verification:

```bash
cargo test --workspace
cargo run -- tool render --input tests/fixtures/two-pages.pdf --output /tmp/pages --format jpg
cargo run -- tool ocr --input tests/fixtures/scanned.pdf --output /tmp/searchable.pdf
```

## Milestone 4: Add honest PDF compression

**Exit criterion:** Lossless and strong modes are visibly distinct, outputs are valid and atomic, and picopdf never replaces an input with a larger candidate.

- [ ] Add qpdf and Ghostscript to the opt-in Nix development shell and record their licenses. — blocked by: locked flake from milestone 1
- [ ] Add a research fixture set for image-heavy, text-heavy, structured, linked, and already optimized PDFs. — blocked by: milestone 1
- [ ] Implement `tool compress --mode lossless` with a qpdf adapter and before/after byte reporting. — blocked by: external-process behavior from milestone 2; compression fixtures
- [ ] Implement `tool compress --mode strong` with a Ghostscript adapter and explicit lossy warnings. — blocked by: lossless command contract; compression fixtures
- [ ] Validate resulting PDFs, use atomic output, and retain the original bytes when a candidate is larger. — blocked by: both compression adapters
- [ ] Test metadata, link, image-quality, and size behavior; document what each backend can change. — blocked by: complete compression flow

Verification:

```bash
cargo test --workspace
cargo run -- tool compress --input tests/fixtures/image-heavy.pdf --output /tmp/lossless.pdf --mode lossless
cargo run -- tool compress --input tests/fixtures/image-heavy.pdf --output /tmp/strong.pdf --mode strong
```

## Milestone 5: Accept images and office files in `write`

**Exit criterion:** `write` converts the promised image and office formats to valid PDFs while preserving the existing Markdown interface.

- [ ] Research maintained Rust decoders for JPEG, PNG, BMP, GIF, and TIFF; record license, orientation, color-profile, alpha, and frame support. — blocked by: milestone 2
- [ ] Define image page-size, margin, scaling, orientation, and multi-frame behavior with fixture examples. — blocked by: image decoder research
- [ ] Extend `write` to image inputs and test one-page, multi-input, transparent, oriented, animated, and multipage cases. — blocked by: image behavior decision
- [ ] Add LibreOffice to the opt-in Nix development shell and record its closure size. — blocked by: locked flake from milestone 1
- [ ] Implement a LibreOffice adapter with a private temporary user profile for DOC/DOCX, XLS/XLSX, and PPT/PPTX. — blocked by: external-process behavior from milestone 2
- [ ] Extend `write` to office input, infer formats from extensions, and report missing fonts or conversion failures clearly. — blocked by: LibreOffice adapter
- [ ] Compare representative office outputs on Linux, macOS, and Windows where CI or maintainers can support them. — blocked by: office input
- [ ] Update help and README with backend installation and fidelity limits. — blocked by: reviewed image and office flows

Verification:

```bash
cargo test --workspace
cargo run -- write --input tests/fixtures/photo.jpg --output /tmp/photo.pdf
cargo run -- write --input tests/fixtures/report.docx --output /tmp/report.pdf
cargo run -- write --input tests/fixtures/sheet.xlsx --output /tmp/sheet.pdf
cargo run -- write --input tests/fixtures/deck.pptx --output /tmp/deck.pdf
```

## Milestone 6: Add a native reader and automatic selection

**Exit criterion:** The native backend passes the shared IR fixtures for supported born-digital PDFs, and automatic selection never chooses it solely because selectable text exists.

- [ ] Prototype text, font, geometry, image, and reading-order recovery with current Rust PDF dependencies. — blocked by: stable IR from milestone 2
- [ ] Define the exact PDF features the native backend supports and the signals that require Docling. — blocked by: native prototype
- [ ] Implement `--backend native` against the same IR and renderer contracts. — blocked by: approved support boundary
- [ ] Compare native and Docling output on single-column, multi-column, table, equation, figure, and malformed fixtures. — blocked by: native backend
- [ ] Add `--backend auto` only when selection is explainable, tested, and directly overridable. — blocked by: backend comparison
- [ ] Keep `docling` as the fallback for scanned or structurally complex PDFs. — blocked by: automatic selection

## Milestone 7: Research and gate editable office exports

**Exit criterion:** Each format has an approved output contract and a passing prototype before implementation tickets are written. A failed prototype leaves the feature deferred rather than relabeled as editable.

### DOCX

- [ ] Compare direct OOXML generation, a maintained Rust DOCX library, and conversion from picopdf Markdown/HTML. — blocked by: stable IR from milestone 2
- [ ] Prototype headings, paragraphs, nested lists, equations, tables with spans, images, and page references. — blocked by: backend comparison
- [ ] Approve only if content stays editable, follows reading order, and loses no fixture text silently. — blocked by: DOCX prototype
- [ ] Implement `read --format docx` and compatibility tests after approval. — blocked by: approved DOCX gate

### XLSX

- [ ] Compare IR table export with Camelot on born-digital, borderless, and scanned table fixtures. — blocked by: stable IR from milestone 2
- [ ] Define workbook naming, one-sheet-per-table behavior, spans, types, and provenance sheet. — blocked by: comparison
- [ ] Approve only if representative fixtures preserve exact cell text and make uncertain recovery visible. — blocked by: XLSX prototype
- [ ] Implement `read --format xlsx` and compatibility tests after approval. — blocked by: approved XLSX gate

### PPTX

- [ ] Define what “editable” means for text, images, tables, equations, and layout. — blocked by: stable IR from milestone 2
- [ ] Prototype representative presentation PDFs as separate slide objects, not page-sized background images. — blocked by: editability definition
- [ ] Review the prototype in LibreOffice Impress and Microsoft PowerPoint where available. — blocked by: PPTX prototype
- [ ] Implement `read --format pptx` only if the prototype is useful and meets the editability definition. — blocked by: approved PPTX gate

## Milestone 8: Package the stable CLI

**Exit criterion:** Supported platforms can install picopdf, discover backend requirements, and access version-matched terminal and web-readable documentation.

- [ ] Generate shell completions after command and flag names stabilize. — blocked by: milestones 1–7 command decisions
- [ ] Evaluate generated man pages and package them where the installation channel supports them. — blocked by: stable help
- [ ] Publish backend compatibility ranges, licenses, and platform-specific installation instructions. — blocked by: all shipped external adapters
- [ ] Publish standalone macOS and Linux artifacts; verify the Linux artifact under WSL. — blocked by: release checks on each target
- [ ] Add release checks for clean source archives, licenses, help snapshots, and smoke tests. — blocked by: packaging decisions
