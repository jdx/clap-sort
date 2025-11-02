//! # clap-sort
//!
//! A library to validate that clap subcommands are sorted alphabetically.
//!
//! This crate provides functionality to parse Rust source files and check that
//! clap subcommands defined in enums are sorted alphabetically by their CLI names.

use std::path::Path;
use syn::{Attribute, File, Meta, Variant};
use syn::visit::Visit;

/// Error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
    pub enum_name: String,
    pub unsorted_variants: Vec<(String, String)>, // (actual_order, expected_order)
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Extract the CLI name from a clap attribute, if present
fn get_cli_name(variant: &Variant) -> Option<String> {
    for attr in &variant.attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("command") || meta_list.path.is_ident("clap") {
                // Parse the attribute content
                let tokens = meta_list.tokens.to_string();

                // Look for name = "..." pattern
                if let Some(start) = tokens.find("name = \"") {
                    let after_equals = &tokens[start + 8..];
                    if let Some(end) = after_equals.find('"') {
                        return Some(after_equals[..end].to_string());
                    }
                }
            }
        }
    }
    None
}

/// Check if an enum has the Subcommand derive
fn has_subcommand_derive(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("derive") {
                let tokens = meta_list.tokens.to_string();
                if tokens.contains("Subcommand") {
                    return true;
                }
            }
        }
    }
    false
}

/// Validate that an enum's variants are sorted alphabetically by their CLI names
pub fn validate_enum_sorted(enum_item: &syn::ItemEnum) -> Result<(), ValidationError> {
    // Only check enums with #[derive(Subcommand)]
    if !has_subcommand_derive(&enum_item.attrs) {
        return Ok(());
    }

    let mut cli_names = Vec::new();

    for variant in &enum_item.variants {
        let name = get_cli_name(variant)
            .unwrap_or_else(|| variant.ident.to_string().to_lowercase().replace('_', "-"));
        cli_names.push(name);
    }

    // Check if sorted
    let mut sorted_names = cli_names.clone();
    sorted_names.sort();

    if cli_names != sorted_names {
        return Err(ValidationError {
            message: format!(
                "Enum '{}' has unsorted subcommands.\nActual order: {:?}\nExpected order: {:?}",
                enum_item.ident, cli_names, sorted_names
            ),
            enum_name: enum_item.ident.to_string(),
            unsorted_variants: cli_names.into_iter().zip(sorted_names).collect(),
        });
    }

    Ok(())
}

/// Visitor to find and validate all enums in a syntax tree
struct EnumValidator {
    errors: Vec<ValidationError>,
}

impl<'ast> Visit<'ast> for EnumValidator {
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        if let Err(e) = validate_enum_sorted(node) {
            self.errors.push(e);
        }
    }
}

/// Validate that all clap Subcommand enums in a source file are sorted
pub fn validate_file(source: &str) -> Result<(), Vec<ValidationError>> {
    let syntax_tree: File = syn::parse_str(source)
        .map_err(|e| vec![ValidationError {
            message: format!("Failed to parse source: {}", e),
            enum_name: String::new(),
            unsorted_variants: Vec::new(),
        }])?;

    let mut validator = EnumValidator { errors: Vec::new() };
    validator.visit_file(&syntax_tree);

    if validator.errors.is_empty() {
        Ok(())
    } else {
        Err(validator.errors)
    }
}

/// Validate a Rust source file at the given path
pub fn validate_file_path(path: &Path) -> Result<(), Vec<ValidationError>> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| vec![ValidationError {
            message: format!("Failed to read file: {}", e),
            enum_name: String::new(),
            unsorted_variants: Vec::new(),
        }])?;

    validate_file(&source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_enum() {
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
    }

    #[test]
    fn test_unsorted_enum() {
        let source = r#"
            use clap::Subcommand;

            #[derive(Subcommand)]
            enum Commands {
                List,
                Add,
                Delete,
            }
        "#;

        let result = validate_file(source);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].enum_name, "Commands");
    }

    #[test]
    fn test_custom_names_sorted() {
        let source = r#"
            use clap::Subcommand;

            #[derive(Subcommand)]
            enum Commands {
                #[command(name = "add")]
                AddCmd,
                #[command(name = "delete")]
                DeleteCmd,
                #[command(name = "list")]
                ListCmd,
            }
        "#;

        assert!(validate_file(source).is_ok());
    }

    #[test]
    fn test_custom_names_unsorted() {
        let source = r#"
            use clap::Subcommand;

            #[derive(Subcommand)]
            enum Commands {
                #[command(name = "list")]
                ListCmd,
                #[command(name = "add")]
                AddCmd,
                #[command(name = "delete")]
                DeleteCmd,
            }
        "#;

        let result = validate_file(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_non_subcommand_enum_ignored() {
        let source = r#"
            #[derive(Debug)]
            enum NotACommand {
                Zebra,
                Apple,
                Banana,
            }
        "#;

        // Should not error since it's not a Subcommand enum
        assert!(validate_file(source).is_ok());
    }

    #[test]
    fn test_multiple_enums_mixed() {
        let source = r#"
            use clap::Subcommand;

            #[derive(Subcommand)]
            enum Commands {
                Add,
                List,
            }

            #[derive(Debug)]
            enum Other {
                Zebra,
                Apple,
            }

            #[derive(Subcommand)]
            enum MoreCommands {
                Delete,
                Update,
            }
        "#;

        assert!(validate_file(source).is_ok());
    }
}
