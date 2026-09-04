---
title: "picopdf project specification"
status: "Ready for milestones 1–3; later format exporters have research gates"
---

# picopdf project specification

## Objective

picopdf makes PDFs useful in both directions:

```text
Markdown and supported source files → picopdf write → PDF
PDF → picopdf read → structured Markdown and JSON
PDF → picopdf tool → transformed PDF or page images
```

The next primary feature is `picopdf read`: local conversion of a PDF into a small, documented bundle that people and coding agents can inspect without loading the original PDF into context. The first reader uses the `picopdf-docling` Python package as an external process. That package adapts Docling's typed Python API to picopdf's smaller intermediate representation (IR). The Rust CLI owns the IR contract, bundle layout, and Markdown output.

picopdf remains a Rust CLI with a small default installation. OCR, machine-learning models, office conversion, rasterization, and lossy compression stay in external tools rather than being embedded in `picopdf-core`.

## Users and use cases

- A reader can convert a textbook chapter into Markdown while preserving equations, tables, figures, reading order, and source pages.
- An agent can read a selected page range and cite the source page for each extracted block.
- A script can consume stable JSON without parsing status messages or Docling's full schema.
- A user can make a scanned PDF searchable without asking picopdf to summarize it.
- A user can render PDF pages as JPEG images or combine common image formats into a PDF.
- A user can reduce a PDF's file size with an explicit lossless or lossy mode.
- A user can convert office documents to PDF when LibreOffice is installed.
- In later releases, a user can export recovered document content to DOCX, table data to XLSX, or a presentation to PPTX, subject to the format-specific acceptance gates below.

Note-taking and summarization belong to the program or agent that consumes the bundle. picopdf preserves and converts source content; it does not interpret the work or generate study notes.

## Success criteria

### Structured reading

- `picopdf read --input book.pdf --output book/` creates a self-contained bundle with `document.md`, `document.json`, `manifest.json`, and an `assets/` directory when assets exist.
- The bundle preserves Docling's reading order and represents headings, paragraphs, lists, code, equations, tables, and figures without exposing Docling-specific types in the public schema.
- Every content block has at least one source record with a one-based page. A source record includes a bounding box when the backend provides one.
- `--pages N` and `--pages N-M` restrict conversion to the requested inclusive page range and reject zero, reversed, malformed, and out-of-range values.
- `--formula` enables formula enrichment. `--pictures` exports referenced figure images and available captions or classifications.
- `--output -` writes only Markdown to standard output. `--json --output -` writes only the normalized IR as JSON. Neither form mixes progress or diagnostics into standard output.
- Running without `picopdf-docling` produces a short error, a non-zero dependency exit code, and an installation command. It does not panic or print a Rust backtrace by default.
- The Rust binary does not embed Python, Torch, Docling, OCR models, or a Python interpreter.
- The development flake provides the pinned `picopdf-docling` package beside the Rust toolchain without making Nix part of picopdf's runtime contract.

### Existing behavior

- Markdown-to-PDF and all implemented manipulation commands continue to work with their current flags.
- Existing command lines are not removed or reinterpreted without a documented deprecation period.
- Paths that are not valid UTF-8 do not panic.

### CLI behavior

- The CLI follows the applicable [Command Line Interface Guidelines](https://clig.dev/): concise default help, full `-h`/`--help`, examples, standard streams, stable exit behavior, early validation, useful dependency errors, and restrained output.
- `--version`, `--quiet`, and `--no-color` are available globally. `NO_COLOR` and `TERM=dumb` are honored, and color is disabled when the relevant stream is not a terminal.
- Human progress and diagnostics go to standard error. Primary streamed data goes to standard output.
- Long operations report progress on a terminal and remain plain when redirected. `--quiet` suppresses non-essential status output.
- New file-oriented commands accept `-` where streaming is possible. A command rejects `-` when it needs a seekable input or must produce multiple files, and explains why.
- Success exits with 0. Usage errors use clap's exit code 2. Runtime or data errors use 1. A missing or incompatible external backend uses 3.

## Current state

- The workspace contains `picopdf`, the clap-based CLI, and `picopdf-core`.
- `picopdf-core` renders a limited Markdown subset through `pulldown-cmark` and `pdf-writer`. It uses `lopdf` for merge, split, extract, delete, rotate, and organize operations.
- The CLI currently requires `--input` and `--output` for `write` and most tools. Merge accepts multiple positional inputs, which is appropriate for one operation over several files.
- Status messages currently use standard output, colors are unconditional, and several path conversions use `to_str().unwrap()`. These must be corrected before adding streamed reader output.
- The core has 18 unit tests. There are no CLI integration tests or fixture PDFs for tools.
- Rust formatting, workspace tests, and clippy checks pass. The core has 18 unit tests; CLI integration tests have not been added yet.
- The repository is licensed under MIT. External backend licenses and model licenses must be documented before release, even when the tools are invoked as separate processes.
- The repository has a locked `py/picopdf-docling/` scaffold and a locked development flake. The default shell does not yet include the sidecar, and `.envrc` and model setup remain to be added. Released Rust binaries must not depend on Nix.

## Command-line interface

### Stable command families

```text
picopdf write   supported input → PDF
picopdf read    PDF → document bundle or streamed structured text
picopdf tool    focused PDF operations
```

Do not add a catch-all command or implicit subcommand. New flags have long names. Short names are reserved for common conventions such as `-i/--input`, `-o/--output`, and `-q/--quiet`.

### Read

```bash
picopdf read \
  --input textbook.pdf \
  --output textbook/ \
  --backend docling \
  --formula \
  --pictures

picopdf read --input textbook.pdf --pages 75-93 --output chapter/
picopdf read --input textbook.pdf --pages 75-93 --output -
picopdf read --input textbook.pdf --pages 75-93 --json --output -
```

Options for the first release:

- `-i, --input <PDF>`: required PDF path. Standard input is deferred because the sidecar and PDF parsers need seekable input; `--input -` must fail with that explanation.
- `-o, --output <PATH|->`: required bundle directory, or `-` for one streamed document.
- `--backend <docling>`: defaults to `docling`. The explicit option reserves a stable selection point for a native backend.
- `--pages <N|N-M>`: one one-based page or inclusive contiguous range.
- `--formula`: enable formula enrichment.
- `--pictures`: include normalized figure assets. Without it, figure blocks retain captions, descriptions, and provenance but have no asset path.
- `--timeout <SECONDS>`: stop conversion after 1,800 seconds by default. The process adapter must terminate and reap the child.
- `--allow-model-downloads`: permit the Python sidecar to let Docling fetch missing model artifacts. Without it, conversion runs in the tested offline mode and tells the user how to download models explicitly when artifacts are missing.
- `--json`: with `--output -`, emit `document.json` instead of Markdown. It is invalid for directory output because a bundle already includes JSON.
- `--force`: replace known bundle files in an existing directory. Unknown files are never deleted.

For directory output, picopdf writes to a sibling staging directory and renames completed files into place. On failure or interruption, it removes the staging directory when possible and leaves an existing bundle unchanged. Without `--force`, picopdf fails if any target bundle file already exists.

For streamed output, picopdf must not create durable assets. `--pictures` with `--output -` is therefore an error. Figure captions, descriptions, and provenance remain in the stream.

### Future commands

The existing vocabulary extends without adding a new top-level `convert` command:

```bash
# Additional sources accepted by write
picopdf write --input report.docx --output report.pdf
picopdf write --input slides.pptx --output slides.pdf
picopdf write --input figures.xlsx --output figures.pdf
picopdf write --input scan.jpg --output scan.pdf
picopdf write --input front.jpg --input back.png --output scan.pdf

# Additional read formats, added only after their acceptance gates pass
picopdf read --input report.pdf --format docx --output report.docx
picopdf read --input tables.pdf --format xlsx --output tables.xlsx
picopdf read --input deck.pdf --format pptx --output deck.pptx

# Focused PDF tools
picopdf tool render --input report.pdf --output pages/ --format jpg
picopdf tool ocr --input scan.pdf --output searchable.pdf
picopdf tool compress --input large.pdf --output small.pdf --mode lossless
picopdf tool compress --input large.pdf --output small.pdf --mode strong
```

`write` continues to infer the source format from the extension. A later `--from` option may resolve extensionless or ambiguous input, but must not change existing Markdown behavior. `read` defaults to the structured bundle; `--format` is introduced only when another output format ships.

## Document IR

### Ownership and compatibility

The IR is the stable boundary between reader backends and output renderers. It lives under `picopdf_core::read` and derives serde serialization. `document.json` contains `schema_version`. The first schema version is `1`.

Within a schema version:

- fields are not removed or reinterpreted;
- new optional fields and enum variants may be added;
- block order is document reading order;
- page numbers are one-based;
- bounding boxes use PDF points in `[left, top, right, bottom]` order with a top-left origin;
- asset paths are relative, use `/` separators in JSON and Markdown, and cannot escape the bundle.

A breaking schema change increments `schema_version` and requires migration notes and fixtures for both versions.

### Data model

The implementation may refine Rust names, but the serialized contract must express:

```text
Document
  schema_version
  metadata: title?, author?, language?, page_count
  pages[]
    number
    width_points?
    height_points?
    blocks[]

Source
  page
  bbox?: [left, top, right, bottom]

Block
  id
  sources[]
  kind: heading | paragraph | list | code | equation | table | figure | other
  kind-specific content
```

Required kind-specific content:

- Heading: level and text.
- Paragraph: text.
- List: ordered flag and ordered items; nesting must not be flattened silently.
- Code: text and optional language.
- Equation: LaTeX when enrichment provides it, plus original text when available.
- Table: rows of cells; each cell can carry text, row span, and column span.
- Figure: optional relative asset path, caption, description, and classifications.
- Other: upstream label and any text. This preserves content when a new Docling item cannot yet be mapped.

Each block has one or more source records so content that spans pages does not lose provenance. The page containing the first source record owns the block in `pages[]`.

Each block ID is deterministic for the same normalized document and page selection. IDs must not depend on timestamps, temporary paths, or hash-map iteration order.

### Bundle

```text
book/
├── document.md
├── document.json
├── manifest.json
└── assets/
    ├── figure-001.png
    └── figure-002.png
```

`manifest.json` records:

- bundle schema version;
- source filename and SHA-256 digest;
- selected pages;
- picopdf version;
- backend name and reported version;
- enabled read options;
- generated relative file paths;
- normalization warnings.

The manifest does not include a generation timestamp by default, so repeated runs with the same source, options, picopdf version, backend version, and model artifacts can be compared. picopdf does not promise byte-identical output across backend or model versions.

### Markdown rendering

Markdown is rendered from the picopdf IR, not copied from Docling's Markdown export.

- Page changes use `<!-- picopdf:page=73 -->`.
- Headings use their recovered level, clamped to Markdown levels 1–6 with a manifest warning when changed.
- Display equations use `$$` fences and preserve LaTeX.
- Simple tables use Markdown tables. Tables with row spans, column spans, or multiline cells use HTML tables.
- Figures use relative Markdown image links when an asset exists. Captions and descriptions remain text when no asset exists.
- A source comment follows equations, tables, and figures and can follow any block with a bounding box: `<!-- source: page=73 bbox=94,281,488,353 -->`.
- Backend diagnostics, confidence values, and internal IDs do not appear in prose unless they help a consumer locate or assess recovered content.

## Docling backend

The backend is a Python distribution named `picopdf-docling` under `py/picopdf-docling/`. It exposes a `picopdf-docling` console script and pins the Docling version it adapts. The Rust CLI invokes that executable with `std::process::Command`; it never invokes `python`, `uv`, or Docling's CLI directly.

Typical installation outside the repository is:

```bash
cargo install picopdf
uv tool install picopdf-docling
picopdf-docling models download
```

`uv` installs the sidecar but is not involved in each conversion. Other isolated Python installers may be documented if they preserve the console script and supported dependency versions.

The process protocol is versioned independently from the Python package and the public document schema:

```bash
picopdf-docling --protocol-version

picopdf-docling convert \
  --input input.pdf \
  --output TEMP_DIR \
  --pages 75-93 \
  --formula \
  --pictures \
  --offline
```

The sidecar writes only to the private temporary output directory supplied by Rust:

```text
TEMP_DIR/
├── document.json    picopdf IR
├── backend.json     protocol, package, Docling, model, and warning data
└── assets/
```

The Python package owns all Docling-specific work. It uses Docling's typed Python API, traverses the document body to preserve reading order and grouping, converts coordinate origins, normalizes supported items into the picopdf IR, exports requested assets, and records unmapped content as `other` blocks with warnings. It must not render final Markdown, construct the public manifest, or write directly to the user's destination.

Rust owns the protocol client, public IR validation, Markdown and JSON rendering, manifest, staging, overwrite rules, streams, and user-facing errors. Before conversion it checks `--protocol-version` and accepts only a documented protocol range. A mismatch reports the detected and supported versions plus an upgrade command.

Arguments are passed separately, never through a shell. Rust enforces the overall timeout and terminates and reaps the sidecar on expiry. The sidecar's stderr is captured; picopdf prints a short explanation and useful final diagnostic lines on failure, with full output reserved for verbose/debug mode.

Docling can download model artifacts on first use. picopdf passes `--offline` unless the user supplies `--allow-model-downloads`. Missing artifacts produce an explicit `picopdf-docling models download` hint. The sidecar must prove that offline mode cannot access the network for its pinned Docling configuration.

The Python package and Rust workspace share checked-in protocol and IR fixtures. A change to Python normalization cannot merge unless Rust accepts its output, and a Rust schema change cannot merge unless the Python package emits it.

## Other conversion features

### Searchable PDF OCR

`picopdf tool ocr` is separate from `read`:

```text
scanned or mixed PDF → searchable PDF
```

Use OCRmyPDF as an external backend. OCRmyPDF is designed to add an OCR text layer while retaining the PDF and can deskew pages; Tesseract alone does not recover headings, paragraphs, or reliable reading order. The first interface should expose input, output, language, deskew, and force/reprocess behavior without mirroring every OCRmyPDF flag. Backend-specific advanced options remain available in OCRmyPDF itself.

### PDF compression

Compression has two explicit modes because “compression” can change content:

- `lossless`: use qpdf-compatible structural and stream compression. If the candidate is larger, write the original bytes and report that no smaller lossless representation was found.
- `strong`: use Ghostscript `pdfwrite` with documented image downsampling and lossy JPEG settings. The help text must state that metadata, interactive features, color, image quality, and internal PDF structure can change. If the candidate is larger, keep the input bytes.

Both modes write atomically, validate that the result opens as a PDF, and report input and output byte sizes. “Basic” and “strong” from the old to-do list map to `lossless` and `strong`; `lossless` is the default.

### PDF pages to JPEG

Use `pdftocairo` from Poppler as an external renderer for the first implementation. It supports PDF input, page ranges, resolution, JPEG quality, and one image per page. picopdf owns predictable output names, overwrite checks, and errors. The default is 150 PPI, matching `pdftocairo`; expose `--resolution` and `--quality` with validated bounds.

### Images to PDF

Implement JPEG, PNG, BMP, GIF, and TIFF input in Rust if maintained decoders cover the required formats. Repeating `--input` creates a multipage PDF in argument order; Markdown and office input continue to accept one source. Preserve aspect ratio, honor image orientation metadata where available, and define page size and margins. A multi-frame GIF or TIFF produces one page per frame unless the user selects the first frame. This work requires fixture-based review because color profiles, alpha channels, animation, and multipage TIFFs differ by decoder.

### Office files to PDF

Use LibreOffice's `soffice --headless --convert-to pdf --outdir …` as an external adapter for DOC/DOCX, XLS/XLSX, and PPT/PPTX. Use a temporary user profile so concurrent invocations do not share LibreOffice state. Fonts and LibreOffice's import filters affect layout; the command reports the detected LibreOffice version and does not promise pixel identity with Microsoft Office.

### PDF to editable office formats

These exporters follow the structured reader because PDF does not contain the source document model:

- DOCX: map headings, paragraphs, lists, tables, equations, and images from the IR. Page-perfect reconstruction is not the goal. The acceptance gate is editable content in reading order with page references and no silent text loss.
- XLSX: export detected tables, not arbitrary page layout. Each table becomes a named worksheet, merged cells preserve spans, and a source sheet records page and bounding-box provenance. The acceptance gate is exact cell text on a representative born-digital and scanned fixture.
- PPTX: requires a prototype and product decision. A page image placed on each slide is not “editable PowerPoint.” Implementation starts only after a prototype shows that text, tables, and figures from representative decks remain separately editable and useful.

The unmaintained `pdf2docx` project is not selected as a core backend. Camelot is a possible comparison backend for difficult table extraction, but Docling's tables and the picopdf IR remain the initial path so users do not need two ML stacks.

## Architecture and project structure

Keep the two Rust crates and add one separately packaged Python backend:

```text
crates/
├── cli/
│   ├── src/
│   └── tests/                 command behavior and stream separation
└── core/
    └── src/
        ├── md/                Markdown → PDF
        ├── read/
        │   ├── document.rs    IR and schema
        │   ├── backend.rs     process protocol client
        │   └── markdown.rs    IR → Markdown
        └── tools/             PDF transformations

py/
└── picopdf-docling/
    ├── pyproject.toml
    ├── uv.lock
    ├── src/picopdf_docling/
    │   ├── cli.py
    │   ├── convert.py
    │   ├── normalize.py
    │   └── protocol.py
    └── tests/
```

Dependency direction:

```text
CLI → core::md
    → core::read → backend protocol → picopdf-docling → Docling Python API
                 → Document IR → Markdown/JSON/bundle
    → core::tools

future adapters → external ocrmypdf, pdftocairo, gs, qpdf, soffice
```

Process discovery, protocol probing, exit-status translation, temporary paths, and bounded diagnostic capture should share one small Rust utility once a second external backend is added. Do not create that abstraction for one process client alone.

## Development environment and cross-platform distribution

There is one user-facing `picopdf` CLI. Structured reading additionally requires the `picopdf-docling` console script on `PATH`. The same Rust binary and versioned process protocol apply on macOS, Linux, and WSL.

Nix is the development environment for this repository, not the product runtime. The flake must expose:

- `packages.default` and `packages.picopdf`: the standalone Rust CLI for local build checks;
- `devShells.rust`: a lightweight Rust toolchain and project checks;
- `devShells.default`: Rust plus the locked `py/picopdf-docling/` environment;
- `checks`: Rust tests, Python tests, protocol compatibility tests, and a sidecar smoke check without model inference.

As more adapters ship, an opt-in `devShells.full` may add LibreOffice, Ghostscript, OCRmyPDF, Poppler, and qpdf for compatibility testing without growing the default shell.

Use a committed `flake.lock` and `py/picopdf-docling/uv.lock`. `uv2nix` with wheel preference is the leading way to expose the Python package in `devShells.default`; a nixpkgs Python overlay is the fallback. No `nix/docling/` directory is needed. Keep the integration in `flake.nix` until a concrete set of reusable Nix overrides is large enough to justify a small `nix/python.nix` module.

The flake may use Nix-specific patches to make the development environment work. Those patches must not leak into the process protocol or hide a portability defect. Integration tests outside Nix use a fake `picopdf-docling`, and release smoke tests install the real Python package through uv on each supported operating system.

Model artifacts are separate from the Python runtime. Flake evaluation, `nix develop`, and direnv must not download weights. The development shell can use fixed-output model derivations or an explicit ignored local cache after licenses and hashes are reviewed. End users install the Rust binary and `picopdf-docling` independently, then provision models explicitly unless they pass `--allow-model-downloads`.

Commit a minimal `.envrc` that loads `use flake .#default` and watches `flake.nix`, `flake.lock`, `py/picopdf-docling/pyproject.toml`, and `py/picopdf-docling/uv.lock`. It must not source `.env`, read secrets, mutate the repository, install packages, or download models. Each developer authorizes it with `direnv allow` after review. `.direnv/` and any project-local model cache stay ignored.

Required development checks:

```bash
nix flake check
nix build .#picopdf
nix develop .#rust -c cargo test --workspace
nix develop -c picopdf-docling --protocol-version
nix develop -c cargo run -- --help
```

Release builds must also pass outside the development shell for `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, and `aarch64-apple-darwin`. WSL uses the Linux build and gets a separate smoke test for path and process behavior. Native Windows support is deferred until its backend and path behavior are tested.

## Error and safety requirements

- Validate extensions, page ranges, output conflicts, and option combinations before starting an external process.
- Preserve `OsStr`/`Path` values across Rust and child-process boundaries. Do not convert paths to UTF-8 strings unless a file format requires UTF-8.
- Never invoke a backend through a shell or accept raw pass-through arguments in the first release.
- Treat PDFs and office files as untrusted. Run tools locally with the user's permissions, no network access requested by picopdf, and no automatic external plugin loading.
- Do not send documents to a remote Docling service. The sidecar must disable Docling's backend-managed model downloads in offline mode. A future remote backend requires explicit design and user opt-in.
- Do not overwrite existing read-bundle files without `--force`. Never delete unknown files from an output directory.
- Clean temporary files after success. Failed cleanup must not hide the conversion result or primary error.
- Handle Ctrl-C promptly. A completed output appears atomically; a partial output does not replace it.
- Do not print secrets passed to protected-document backends. If password support is added, accept a password file or hidden prompt, not a command-line value.

## Testing plan

### Test boundary

Use the compiled Rust CLI as the highest stable boundary for command syntax, streams, exit codes, filesystem effects, overwrite behavior, and external-process failures. Test Docling normalization in `py/picopdf-docling/`, and test IR validation and Markdown rendering in `picopdf-core`, using shared checked-in JSON fixtures. Do not require downloaded ML models in the default test suite.

### Required fixtures

Keep small, redistributable fixtures that cover:

- born-digital text with headings and a list;
- a scanned page;
- one equation;
- one simple table and one table with spans;
- one captioned figure;
- two pages with headers or footers;
- non-ASCII text and a non-UTF-8 path test on platforms that support it;
- malformed, encrypted, empty, and out-of-range inputs;
- recorded Docling documents and expected IR from every supported sidecar/Docling API combination.

Generated fixtures must have source scripts or clear provenance. Do not commit model caches or large generated files.

### Automated checks

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
uv run --project py/picopdf-docling --locked pytest
nix flake check
```

CLI integration tests must also prove:

- help and version work at the top level and each command level;
- status text is on stderr and streamed Markdown/JSON on stdout is clean;
- `NO_COLOR`, `TERM=dumb`, redirection, `--no-color`, and `--quiet` disable decoration as specified;
- invalid ranges and incompatible flags fail before backend execution;
- a missing `picopdf-docling` exits with 3 and includes an installation hint;
- a fake `picopdf-docling` can report protocol versions, record exact argv, emit warnings, fail, hang, and return fixture JSON;
- output staging leaves no replaced or half-written bundle after a simulated failure;
- JSON validates against the checked-in picopdf schema and Markdown references only existing assets.

Tests for external tools run in two tiers:

1. default tests use fake executables and recorded outputs;
2. opt-in compatibility tests run installed real tools against the small fixtures and record detected versions.

### Human review

Before each backend release, compare representative output with the source in a PDF viewer. Review reading order, equation fidelity, table cells and spans, figure association, page provenance, and any content moved to `other` blocks. For lossy compression and office conversion, also review visible quality and document metadata.

## Development commands

```bash
cargo build --workspace
cargo run -- --help
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
uv run --project py/picopdf-docling --locked pytest
nix flake check
```

No release is ready while any command above fails. Nix verifies the development environment; the non-Nix release matrix verifies the runtime contract.

## Boundaries for implementation agents

### Always

- Preserve existing command lines unless the spec marks a change as additive.
- Add behavior tests at the CLI or public core boundary.
- Use checked-in fixtures or fake executables in default tests.
- Keep user documents and backend execution local.
- Update `SPEC.md`, `ROADMAP.md`, `TODO.md`, help, and README when a public contract changes.

### Ask first

- Add a new crate, embed a runtime, add an ML dependency outside `py/picopdf-docling/`, add a network service or telemetry, or use an impure development-environment workaround.
- Change the public IR schema incompatibly.
- Add a remote conversion backend.
- Select a PDF-to-office exporter after a research gate.
- Change overwrite behavior for an existing command.

### Never

- Invoke external tools through a shell.
- Download or run a model without an explicit user command.
- Send a document or extracted content over the network by default.
- Hide lossy conversion behind a lossless-sounding mode.
- Claim PDF-to-office output is editable unless the format-specific acceptance gate passes.
- Commit model weights, private documents, generated output trees, or secrets.

## Deferred milestones

- A native reader for simple PDFs should avoid Docling when a usable text layer and reading order can be recovered locally. It must emit the same IR and pass the same fixtures before `--backend auto` becomes the default.
- Automatic backend selection is deferred until native and Docling results can be compared. Selection must be explainable in verbose output and directly overridable.
- Searchable-PDF OCR, rendering, compression, image input, and office input follow the structured reader in the order defined by `ROADMAP.md`.
- DOCX, XLSX, and PPTX output remain behind their research and acceptance gates. They are product work, not discarded backlog.
- Shell completions and man pages follow CLI stabilization; help text and web-readable repository docs come first.
- Accelerator-specific development shells follow a reproducible CPU-only Docling environment. End users can use any compatible accelerator configuration supported by their `picopdf-docling` installation.

## Risks and open questions

- Docling's Python API can change. `picopdf-docling` pins and adapts one tested version, while the Rust/Python process protocol changes only through its own versioning rules.
- The Rust CLI and Python sidecar have separate releases. Protocol compatibility checks and coordinated release tests must prevent either package from silently becoming unusable after an independent upgrade.
- `uv.lock` pins development and CI, but published Python installations resolve from `pyproject.toml`. The package must tightly constrain its supported Docling version and report resolved versions in `backend.json`.
- Docling model output and licenses can vary by selected enrichment model. The manifest must record sidecar, Docling, model, and option versions, and release notes must name tested model configurations.
- Docling does not expose one universal offline switch for every optional OCR or enrichment engine. The compatibility spike must prove the default no-download mode for the supported configuration before release.
- A “native” PDF reader can extract characters yet still produce poor reading order. It must not become the automatic default based only on the presence of a text layer.
- Formula, table, and figure recovery are probabilistic. Warnings and source provenance are required; picopdf must not present recovered content as exact without verification.
- Strong compression and office conversion can alter metadata, links, fonts, color, and layout. Help and success output must describe the selected mode without promising a smaller or identical file.
- PDF-to-PPTX may not produce useful editable slides. The prototype gate can end with the feature remaining deferred.
- Supporting stdin for PDFs may require secure temporary files because backends need seekable input. Decide this only after the file-input path is stable.
- Some Docling dependencies publish platform-specific binary wheels. Nix may need development-only fixes, while release smoke tests must separately prove that normal Docling installations work on each supported operating system.

## Research references

- [Command Line Interface Guidelines](https://clig.dev/): help, streams, errors, flags, interactivity, robustness, future compatibility, and signals.
- [Docling CLI reference](https://docling-project.github.io/docling/reference/cli/): capability and option reference for images, OCR, tables, page ranges, enrichment, and timeouts that the Python adapter must map through Docling's API.
- [Docling document model](https://docling-project.github.io/docling/concepts/docling_document/): body hierarchy, content items, pages, layout, and provenance.
- [Docling supported formats](https://docling-project.github.io/docling/usage/supported_formats/): PDF and office input plus Markdown, JSON, and chunk exports.
- [OCRmyPDF introduction](https://ocrmypdf.readthedocs.io/en/stable/introduction.html): searchable PDF behavior, Tesseract limits, PDF/A defaults, and metadata risks.
- [qpdf CLI](https://qpdf.readthedocs.io/en/stable/cli.html): structural PDF transformations, streams, exit status, and image optimization.
- [Ghostscript PDF optimization](https://ghostscript.com/blog/optimizing-pdfs.html): lossy image compression, downsampling, file-size uncertainty, and metadata changes.
- [LibreOffice conversion filters](https://help.libreoffice.org/latest/en-US/text/shared/guide/convertfilters.html): headless conversion syntax and PDF/Office filters.
- [`pdftocairo` manual](https://www.mankier.com/1/pdftocairo): JPEG page rendering, ranges, resolution, quality, standard streams, and exit codes.
- [`pdf2docx` project status](https://github.com/ArtifexSoftware/pdf2docx): the former Artifex implementation is no longer actively maintained.
- [Camelot](https://github.com/camelot-dev/camelot): table extraction and XLSX export for comparison during the PDF-to-XLSX research gate.
- [uv2nix](https://pyproject-nix.github.io/uv2nix/): locked Python workspaces, wheel-based package overlays, virtual environments, and Nix development shells.
- [direnv standard library](https://direnv.net/man/direnv-stdlib.1.html): `use flake`, file watches, and explicit environment authorization.
- [`nix flake check`](https://nix.dev/manual/nix/2.28/command-ref/new-cli/nix3-flake-check.html): evaluation and build checks for package and development-shell outputs.
