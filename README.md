# clap-sort

A Rust library to validate that clap `Subcommand` enums are sorted alphabetically.

## Overview

When using clap's derive API, it's a good practice to keep subcommands sorted alphabetically for easier maintenance and better UX. This crate helps enforce that convention by validating the clap `Command` structure at runtime.

## Features

- Validates that clap subcommands are sorted alphabetically
- Works with both builder and derive APIs
- Easy integration via unit tests
- Zero dependencies beyond clap
- Lightweight and fast

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
clap-sort = "0.1"
```

## Usage

### Unit Test Integration (Recommended)

The best way to use `clap-sort` is to add a unit test to your CLI project:

```rust
#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn test_subcommands_are_sorted() {
        let cmd = cli::Cli::command();
        clap_sort::assert_sorted(&cmd);
    }
}
```

This approach ensures that:
- Your subcommands stay sorted as part of your normal test suite
- CI/CD will catch any unsorted commands before merge
- Developers get immediate feedback when running `cargo test`

### Full Example with Derive API

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,      // ✓ Sorted
    Delete,   // ✓ Sorted
    List,     // ✓ Sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_sorted() {
        clap_sort::assert_sorted(&Cli::command());
    }
}
```

### Non-Panicking Validation

If you prefer Result-based error handling:

```rust
use clap::CommandFactory;

#[test]
fn test_subcommands() {
    let cmd = Cli::command();
    match clap_sort::is_sorted(&cmd) {
        Ok(()) => println!("Commands are sorted!"),
        Err(msg) => panic!("{}", msg),
    }
}
```

## How It Works

The library validates the runtime `Command` structure by:

1. Extracting all subcommand names from the clap `Command`
2. Comparing them with their alphabetically sorted order
3. Panicking (or returning an error) if they don't match

This approach works with both the builder API and derive API, and validates the actual command structure as clap sees it.

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

## Real-world Example

Here's a complete example showing how unsorted commands are caught:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,    // ❌ Should be third
    Add,     // ❌ Should be first
    Delete,  // ❌ Should be second
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_sorted() {
        clap_sort::assert_sorted(&Cli::command());
    }
}
```

When you run `cargo test`, this will fail with:

```
thread 'tests::test_sorted' panicked at 'Subcommands are not sorted alphabetically!
Actual order: ["list", "add", "delete"]
Expected order: ["add", "delete", "list"]'
```

## License

MIT or Apache-2.0 (your choice)
