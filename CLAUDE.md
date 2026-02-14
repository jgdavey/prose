# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                  # Debug build
cargo build --release        # Release build
cargo test                   # Run all tests (unit, integration, doc tests)
cargo test <test_name>       # Run a single test by name
cargo bench                  # Run Criterion benchmarks
cargo fmt --all -- --check   # Check formatting
cargo clippy                 # Lint
cargo doc --no-deps          # Build documentation
```

## Architecture

`prose` is a CLI tool and library for reformatting/rewrapping text to
a target width, similar to `par`. It is Unicode-aware and uses
Dijkstra's algorithm to find optimal line breaks that minimize
jaggedness.

### Core modules

- **`src/analysis.rs`** — Parses input text into `Block` structs. Detects common prefixes/suffixes (code comment markers like `//`, `#`, `;;`, C-style `/* */`, email quote `>`). Uses `Cow<'a, str>` tokens for zero-copy performance.
- **`src/reformat.rs`** — Contains `Reformatter` which uses Dijkstra's shortest path (via `pathfinding` crate) to find optimal line breaks. Cost function is `(target_width - line_width)²`. Supports three `FormatMode`s: `PlainText`, `Markdown` (uses `pulldown-cmark` to only reformat paragraph blocks), and `Code`.
- **`src/lib.rs`** — Public API re-exports: `FormatMode`, `FormatOpts`, `Reformatter`, `reformat()`.
- **`src/main.rs`** — CLI binary using `clap` derive. Reads from file or stdin, processes paragraph-by-paragraph.

### Testing

Integration tests in `tests/integration.rs` use snapshot-style
testing: input files in `tests/data/inputs/` are reformatted and
compared against expected outputs in `tests/data/outputs/`. A custom
`assert_diff!` macro provides colored diffs on failure.

## Key details

- Rust edition 2024
- Dual-licensed MIT/Apache-2.0
- Both a library crate and binary crate
- Unicode display widths via `unicode-width` crate
- Many examples are provided in the README.md file for input/output
