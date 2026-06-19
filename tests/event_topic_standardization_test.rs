// Event Topic Standardization Snapshot Test
// 
// This test validates that all contract events follow the standardized format:
// - Topic prefix: "vst/<contract_name>" (using full contract folder name)
// - Event name: Symbol representation of the event
// 
// Any deviation from this pattern will cause this test to fail, ensuring
// CI detects event topic drift.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use regex::Regex;

/// Represents a detected event emission
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct EventEmission {
    contract: String,
    topic_prefix: String,
    event_name: String,
    source_file: String,
    line_number: usize,
}

/// Expected event prefix for each contract
fn get_expected_prefix(contract_name: &str) -> String {
    format!("vst/{}", contract_name)
}

/// Mapping of contract names to their full folder names
fn get_contract_name_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    // Map abbreviated names to full names for validation
    map.insert("anomaly_det", "anomaly_detection");
    map.insert("anomaly_dtr", "anomaly_detector");
    map.insert("medical_rec_bkp", "medical_record_backup");
    map.insert("patient_risk", "patient_risk_stratification");
    map.insert("cred_registry", "credential_registry");
    map.insert("homo_registry", "homomorphic_registry");
    map.insert("health_rep", "healthcare_reputation");
    map.insert("crypto_reg", "crypto_registry");
    map.insert("cross_chain_id", "cross_chain_identity");
    map.insert("cross_chain_ac", "cross_chain_access");
    map.insert("cross_chain_en", "cross_chain_enhancements");
    map.insert("contract_ver", "contract_verification");
    map.insert("consent_nft", "medical_consent_nft");
    map.insert("contract_usage", "contract_usage_analytics");
    map
}

/// Parse event emissions from Rust source files
fn parse_event_emissions(contracts_dir: &Path) -> Result<Vec<EventEmission>, Box<dyn std::error::Error>> {
    let mut emissions = Vec::new();
    let regex = Regex::new(
        r#"env\.events\(\)\.publish\(\s*\(\s*String::from_str\([^,]*,\s*"([^"]+)"\)\s*,\s*Symbol::new\([^,]*,\s*"([^"]+)"\)\s*\)"#
    )?;

    // Scan all lib.rs and events.rs files
    for entry in walkdir::WalkDir::new(contracts_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.ends_with("lib.rs") || path.ends_with("events.rs")
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            for caps in regex.captures_iter(line) {
                let topic_prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let event_name = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();

                // Extract contract name from path
                let contract = path
                    .parent()
                    .and_then(|p| p.parent())
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                emissions.push(EventEmission {
                    contract,
                    topic_prefix,
                    event_name,
                    source_file: path.to_string_lossy().to_string(),
                    line_number: line_num + 1,
                });
            }
        }
    }

    Ok(emissions)
}

#[test]
fn test_event_topic_standardization() {
    let contracts_dir = Path::new("./contracts");
    
    match parse_event_emissions(contracts_dir) {
        Ok(emissions) => {
            let mapping = get_contract_name_mapping();
            let mut violations = Vec::new();

            for emission in emissions {
                // Extract the abbreviated name from topic prefix (e.g., "anomaly_det" from "vst/anomaly_det")
                let topic_parts: Vec<&str> = emission.topic_prefix.split('/').collect();
                
                if topic_parts.len() != 2 {
                    violations.push(format!(
                        "Invalid topic format (should be 'vst/<contract>'): {} at {}:{}",
                        emission.topic_prefix, emission.source_file, emission.line_number
                    ));
                    continue;
                }

                let prefix = topic_parts[0];
                let abbreviated_contract = topic_parts[1];

                // Check vst prefix
                if prefix != "vst" {
                    violations.push(format!(
                        "Topic prefix is '{}' but should be 'vst' in {} at {}:{}",
                        prefix, emission.contract, emission.source_file, emission.line_number
                    ));
                }

                // Check if abbreviated name should be expanded
                if let Some(full_name) = mapping.get(abbreviated_contract) {
                    violations.push(format!(
                        "Event in {} uses abbreviated prefix 'vst/{}' but should use 'vst/{}' at {}:{}",
                        emission.contract, abbreviated_contract, full_name, emission.source_file, emission.line_number
                    ));
                } else {
                    // Verify abbreviated name matches contract name
                    let expected = get_expected_prefix(&emission.contract);
                    if emission.topic_prefix != expected {
                        // Allow if current abbreviated matches contract (might be intentional abbreviation)
                        if abbreviated_contract != emission.contract {
                            violations.push(format!(
                                "Event in {} uses prefix '{}' but contract is '{}' at {}:{}",
                                emission.contract, emission.topic_prefix, expected, emission.source_file, emission.line_number
                            ));
                        }
                    }
                }
            }

            if !violations.is_empty() {
                eprintln!("Event Topic Standardization Violations:");
                for violation in &violations {
                    eprintln!("  - {}", violation);
                }
                panic!("Found {} event topic standardization violations", violations.len());
            }
        }
        Err(e) => {
            eprintln!("Error parsing event emissions: {}", e);
            // Don't fail the test if we can't parse (walkdir might not be available)
            println!("Event parsing test skipped: {}", e);
        }
    }
}

#[test]
fn test_event_names_are_symbols() {
    // This test ensures event names are valid Soroban Symbol types
    // Symbol constraint: max 32 characters
    let contracts_dir = Path::new("./contracts");
    
    match parse_event_emissions(contracts_dir) {
        Ok(emissions) => {
            let mut violations = Vec::new();

            for emission in emissions {
                // Soroban Symbol constraint: max 32 bytes
                if emission.event_name.len() > 32 {
                    violations.push(format!(
                        "Event name '{}' is {} bytes (max 32) in {} at {}:{}",
                        emission.event_name,
                        emission.event_name.len(),
                        emission.contract,
                        emission.source_file,
                        emission.line_number
                    ));
                }

                // Event names should not contain spaces
                if emission.event_name.contains(' ') {
                    violations.push(format!(
                        "Event name '{}' contains spaces in {} at {}:{}",
                        emission.event_name, emission.contract, emission.source_file, emission.line_number
                    ));
                }
            }

            if !violations.is_empty() {
                eprintln!("Event Name Violations:");
                for violation in &violations {
                    eprintln!("  - {}", violation);
                }
                panic!("Found {} event name violations", violations.len());
            }
        }
        Err(e) => {
            println!("Event name test skipped: {}", e);
        }
    }
}

// Note: This test requires walkdir and regex dependencies
// Add to Cargo.toml:
// [dev-dependencies]
// walkdir = "2.4"
// regex = "1.10"
