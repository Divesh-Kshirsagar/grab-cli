# grab-cli

A fast, lightweight grep-like CLI tool built with Rust for searching patterns in text streams and files.

`grab-cli` leverages high-performance search algorithms via `memchr` / `memmem` for literal string searching and the `regex` engine for regular expression matching, with ANSI color-highlighted outputs and precise line and column location reporting.

---

## Features

- **Literal Matching (Default)**: Uses fast substring searching with vector-accelerated search routines (`memchr` / `memmem`).
- **Regular Expressions**: Full regex support using Rust's `regex` crate with the `-r` / `--regex` flag.
- **Dual Input Modes**: Reads directly from files via `--path` or seamlessly streams from standard input (`stdin`).
- **Accurate Coordinates**: Reports exact **line number** and **column number** for every occurrence.
- **Formatted & Colorized Output**: Highlights matched terms in bold red with clear color-coded line and column indicators.

---

## Installation

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (Rust 2024 edition compatible / recent stable release)

### Build from Source

Clone the repository and build using Cargo:

```bash
git clone https://github.com/username/grab-cli.git
cd grab-cli
cargo build --release
```

The compiled binary will be located at `target/release/grab-cli`.

You can also install it directly onto your system path:

```bash
cargo install --path .
```

---

## Usage

```text
Usage: grab-cli [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  The pattern to find

Options:
  -p, --path <PATH>  The optional path to a file
  -r, --regex        Enable regular expression search mode
  -h, --help         Print help
  -V, --version      Print version
```

---

## Examples

### 1. Literal Search in a File

Search for exact text occurrences in a specific file:

```bash
grab-cli "fn main" -p src/main.rs
```

### 2. Search via Standard Input (`stdin`)

Pipe the output of another command into `grab-cli`:

```bash
cat Cargo.toml | grab-cli "clap"
```

or using `curl`:

```bash
curl -s https://example.com | grab-cli "Example"
```

### 3. Regular Expression Search

Use `-r` or `--regex` to enable regex pattern matching:

```bash
grab-cli -r "pub fn [a-zA-Z0-9_]+" -p src/matcher/literal.rs
```

Matching patterns from a pipeline:

```bash
echo "Call 123-456-7890 or 987-654-3210" | grab-cli -r "\d{3}-\d{3}-\d{4}"
```

---

## Roadmap / Planned Features

- [ ] Multi-threaded directory traversal using `ignore` and `rayon`.
- [ ] Memory-mapped file support with `memmap2` for ultra-large files.
- [ ] Multi-pattern search with `aho-corasick`.
- [ ] Interactive TUI search powered by `ratatui` and `crossterm`.

---

## License

This project is licensed under the [MIT License](LICENSE).
