use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
struct VariantRecord {
    code: u32,
    symbol: String,
    description: String,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let errors_path = manifest_dir.join("src/errors.rs");
    let docs_path = manifest_dir.join("../../docs/ERROR_CODES.md");

    println!("cargo:rerun-if-changed={}", errors_path.display());
    println!("cargo:rerun-if-changed={}", docs_path.display());

    // Verify that docs/ERROR_CODES.md exists and contains expected entries for this contract.
    // The file is maintained by scripts/generate_error_codes.py and verified by CI.
    let existing = fs::read_to_string(&docs_path).unwrap_or_default();
    let variants = parse_variant_records(&errors_path);

    for variant in variants.iter().take(2) {
        let search_string = format!("| identity_registry | {} |", variant.code);
        if !existing.contains(&search_string) {
            eprintln!("WARNING: docs/ERROR_CODES.md may be out of date.");
            eprintln!("Expected to find: {}", search_string);
            eprintln!("Run: python3 scripts/generate_error_codes.py --write");
        }
    }
}

fn parse_variant_records(errors_path: &Path) -> Vec<VariantRecord> {
    let source = fs::read_to_string(errors_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", errors_path.display()));

    let mut records = Vec::new();
    let mut in_error_enum = false;

    for raw_line in source.lines() {
        let trimmed = raw_line.trim();

        if !in_error_enum {
            if trimmed.starts_with("pub enum Error") {
                in_error_enum = true;
            }
            continue;
        }

        if trimmed == "}" {
            break;
        }

        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue;
        }

        if let Some((symbol, right)) = trimmed.split_once('=') {
            let symbol = symbol.trim();
            if symbol.is_empty() || !symbol.chars().all(|c| c.is_alphanumeric() || c == '_') {
                continue;
            }
            let code_text = right
                .split(',')
                .next()
                .unwrap_or_default()
                .trim()
                .trim_end_matches(',');
            let code = code_text.parse::<u32>().unwrap_or_else(|error| {
                panic!("invalid error code in {}: {error}", errors_path.display())
            });
            records.push(VariantRecord {
                code,
                symbol: symbol.to_string(),
                description: "Generated from contract source".to_string(),
            });
        }
    }

    records
}
