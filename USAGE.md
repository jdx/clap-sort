# Usage Guide

## Quick Start

### Validate a single file

```bash
cd ~/src/clap-sort
./cli/target/debug/clap-sort-cli examples/sorted.rs
```

Expected output:
```
✓ examples/sorted.rs: All Subcommand enums are sorted
```

### Check for errors

```bash
./cli/target/debug/clap-sort-cli examples/unsorted.rs
```

Expected output:
```
✗ examples/unsorted.rs: Found 1 error(s)
  Enum 'Commands' has unsorted subcommands.
Actual order: ["list", "add", "update", "delete"]
Expected order: ["add", "delete", "list", "update"]
```

### Validate multiple files

```bash
./cli/target/debug/clap-sort-cli examples/sorted.rs examples/unsorted.rs
```

### Validate fnox codebase

```bash
./cli/target/debug/clap-sort-cli ~/src/fnox/src/main.rs
```

## Integration with fnox

You can add this validation to fnox's CI pipeline or as a pre-commit hook:

### As a CI check

Add to `.github/workflows/ci.yml` or similar:

```yaml
- name: Check command sorting
  run: |
    cd ~/src/clap-sort
    ./cli/target/debug/clap-sort-cli src/main.rs
```

### As a pre-commit hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
~/src/clap-sort/cli/target/debug/clap-sort-cli src/main.rs
```

## Library Usage

You can also use clap-sort as a library in your tests:

```rust
#[cfg(test)]
mod tests {
    use clap_sort::validate_file_path;
    use std::path::Path;

    #[test]
    fn test_commands_are_sorted() {
        let path = Path::new("src/main.rs");
        validate_file_path(path).expect("Commands should be sorted");
    }
}
```

## Exit Codes

- `0` - All files passed validation
- `1` - One or more files failed validation or an error occurred
