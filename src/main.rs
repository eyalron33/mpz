// This program collects Fq elements from the user, constructs a Merkle tree using the provided inputs,
// and outputs the Merkle tree's root hash and proof paths either to the console or as a JSON file.
// The hash we it uses is Pedersen hash as implemented by Aztec.

use crate::cli::Cli;
use crate::hashing::{fq_to_str_hex, get_fq_element};

use clap::Parser;
use inquire;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

mod cli;
mod consts;
mod hashing;
mod merkle;

// Type alias for a boxed dynamic error trait object, accessible within the crate.
pub(crate) type Error = Box<dyn std::error::Error>;

// Struct to represent a Merkle member (input with index and proof path)
#[derive(Serialize, Deserialize)]
struct MerkleMember {
    leaf: String, // Fq element in hex
    index: usize,
    path: Vec<String>, // Merkle proof path in hex
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occurred: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Collect inputs from the user
    let mut inputs = Vec::new();

    // Logic to gather inputs...
    loop {
        let input = inquire::Text::new("Enter an Fq element (or press Enter to stop):").prompt()?;
        if input.trim().is_empty() {
            break;
        }
        match get_fq_element(&input) {
            Ok(fq) => inputs.push(fq),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Check if inputs were provided
    if inputs.is_empty() {
        eprintln!("No inputs were provided.");
        return Ok(());
    }

    // Create Merkle tree with inputs
    match merkle::create_tree_with_pedersen(&inputs) {
        Ok((root_hash, proof_paths, indices)) => {
            if cli.json {
                // Prepare JSON data in the required format
                let members: Vec<MerkleMember> = inputs
                    .iter()
                    .enumerate()
                    .map(|(i, leaf)| {
                        let path: Vec<String> =
                            proof_paths[i].iter().map(|fq| fq_to_str_hex(fq)).collect();
                        MerkleMember {
                            leaf: fq_to_str_hex(leaf),
                            index: indices[i] as usize,
                            path,
                        }
                    })
                    .collect();

                let tree_json = serde_json::json!({
                    "root": fq_to_str_hex(&root_hash),
                    "leaves": members
                });

                // Determine the output filename
                let output_filename = cli.output.as_deref().unwrap_or(consts::DEFAULT_JSON_OUTPUT);

                // Write the JSON output to the specified file or the default file
                let mut file = File::create(output_filename)?;
                file.write_all(tree_json.to_string().as_bytes())?;
                println!("Merkle tree data saved to {}", output_filename);
            } else {
                // Output to console
                println!("Merkle tree root hash: {:?}", fq_to_str_hex(&root_hash));
                for (i, path) in proof_paths.iter().enumerate() {
                    let hex_path: Vec<String> = path.iter().map(|fq| fq_to_str_hex(fq)).collect();
                    println!("Leaf {}: Proof path: {:?}", i, hex_path);
                }
                println!("Indices: {:?}", indices);
            }
        }
        Err(e) => eprintln!("Error creating Merkle tree: {}", e),
    }

    Ok(())
}
