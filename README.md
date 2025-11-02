# clap-sort

A Rust library and CLI tool to validate that clap `Subcommand` enums are sorted alphabetically by their CLI names.

## Overview

When using clap's derive API, it's a good practice to keep subcommands sorted alphabetically for easier maintenance. This crate helps enforce that convention by parsing Rust source files and checking that all enums with `#[derive(Subcommand)]` have their variants sorted.

## Features

- Validates that clap `Subcommand` enum variants are sorted alphabetically
- Respects custom `#[command(name = "...")]` attributes
- Handles kebab-case conversion from snake_case variant names
- Ignores non-Subcommand enums
- Can be used as a library or CLI tool

## Installation

### As a library

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
clap-sort = { path = "path/to/clap-sort" }
```

### As a CLI tool

Build and install:

```bash
cd cli
cargo build --release
cargo install --path .
```

## Usage

### Library

```rust
use clap_sort::{validate_file, validate_file_path};
use std::path::Path;

// Validate a source string
let source = r#"
    use clap::Subcommand;

    #[derive(Subcommand)]
    enum Commands {
        Add,
        Delete,
        List,
    }
"#;

assert!(validate_file(source).is_ok());

// Validate a file
let path = Path::new("src/main.rs");
match validate_file_path(path) {
    Ok(()) => println!("All subcommands are sorted!"),
    Err(errors) => {
        for error in errors {
            eprintln!("{}", error);
        }
    }
}
```

### CLI

```bash
# Check a single file
clap-sort-cli src/main.rs

# Check multiple files
clap-sort-cli src/main.rs src/commands.rs

# Example output for unsorted file:
# ✗ src/commands.rs: Found 1 error(s)
#   Enum 'Commands' has unsorted subcommands.
#   Actual order: ["list", "add", "delete"]
#   Expected order: ["add", "delete", "list"]
```

## How It Works

The library uses `syn` to parse Rust source files and find all enums with `#[derive(Subcommand)]`. For each variant, it:

1. Checks for `#[command(name = "...")]` or `#[clap(name = "...")]` attributes
2. If no custom name, converts the variant identifier to kebab-case (e.g., `AddUser` → `add-user`)
3. Compares the actual order with the alphabetically sorted order
4. Reports any mismatches

## Examples

### ✅ Correctly sorted

```rust
#[derive(Subcommand)]
enum Commands {
    Add,
    Delete,
    List,
}
```

### ✅ Correctly sorted with custom names

```rust
#[derive(Subcommand)]
enum Commands {
    #[command(name = "add")]
    AddCmd,
    #[command(name = "delete")]
    DeleteCmd,
    #[command(name = "list")]
    ListCmd,
}
```

### ❌ Unsorted

```rust
#[derive(Subcommand)]
enum Commands {
    List,    // Should be third
    Add,     // Should be first
    Delete,  // Should be second
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests covering:
- Sorted enums
- Unsorted enums
- Custom command names
- Non-Subcommand enums (should be ignored)
- Multiple enums in one file

## Use Cases

- **CI/CD**: Add to your pipeline to enforce sorted subcommands
- **Pre-commit hooks**: Validate files before committing
- **Development**: Quickly check if your commands are properly sorted
- **Code review**: Automatically verify PR changes maintain sort order

## License

MIT or Apache-2.0 (your choice)
