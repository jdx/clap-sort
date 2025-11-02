//! # clap-sort
//!
//! A library to validate that clap subcommands are sorted alphabetically.
//!
//! This crate provides functionality to validate that clap `Subcommand` enums
//! have their variants sorted alphabetically by their CLI names at runtime.

/// Validates that subcommands are sorted alphabetically.
///
/// This function takes a clap `Command` and checks that all its subcommands
/// are sorted alphabetically by name.
///
/// # Example
///
/// ```rust
/// use clap::{Command, Subcommand};
///
/// #[derive(Subcommand)]
/// enum Commands {
///     Add,
///     Delete,
///     List,
/// }
///
/// let cmd = clap::Command::new("mycli");
/// // Add subcommands to cmd...
/// clap_sort::assert_sorted(&cmd);
/// ```
pub fn assert_sorted(cmd: &clap::Command) {
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    if subcommands.is_empty() {
        return;
    }

    let mut sorted = subcommands.clone();
    sorted.sort();

    if subcommands != sorted {
        panic!(
            "Subcommands are not sorted alphabetically!\nActual order: {:?}\nExpected order: {:?}",
            subcommands, sorted
        );
    }
}

/// Checks if subcommands are sorted, returning a Result instead of panicking.
///
/// # Example
///
/// ```rust
/// use clap::Command;
///
/// let cmd = Command::new("mycli");
/// match clap_sort::is_sorted(&cmd) {
///     Ok(()) => println!("Commands are sorted!"),
///     Err(msg) => eprintln!("Error: {}", msg),
/// }
/// ```
pub fn is_sorted(cmd: &clap::Command) -> Result<(), String> {
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    if subcommands.is_empty() {
        return Ok(());
    }

    let mut sorted = subcommands.clone();
    sorted.sort();

    if subcommands != sorted {
        Err(format!(
            "Subcommands are not sorted alphabetically!\nActual order: {:?}\nExpected order: {:?}",
            subcommands, sorted
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Command, CommandFactory, Parser, Subcommand};

    #[test]
    fn test_sorted_subcommands() {
        let cmd = Command::new("test")
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"))
            .subcommand(Command::new("list"));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Subcommands are not sorted")]
    fn test_unsorted_subcommands() {
        let cmd = Command::new("test")
            .subcommand(Command::new("list"))
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"));

        assert_sorted(&cmd);
    }

    #[test]
    fn test_is_sorted_ok() {
        let cmd = Command::new("test")
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"))
            .subcommand(Command::new("list"));

        assert!(is_sorted(&cmd).is_ok());
    }

    #[test]
    fn test_is_sorted_err() {
        let cmd = Command::new("test")
            .subcommand(Command::new("list"))
            .subcommand(Command::new("add"));

        assert!(is_sorted(&cmd).is_err());
    }

    #[test]
    fn test_no_subcommands() {
        let cmd = Command::new("test");
        assert_sorted(&cmd);
        assert!(is_sorted(&cmd).is_ok());
    }

    #[test]
    fn test_with_derive_sorted() {
        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            Add,
            Delete,
            List,
        }

        let cmd = Cli::command();
        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Subcommands are not sorted")]
    fn test_with_derive_unsorted() {
        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            List,
            Add,
            Delete,
        }

        let cmd = Cli::command();
        assert_sorted(&cmd);
    }
}
