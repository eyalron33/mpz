// This module provides functionality to hash two Fq elements using nargo,
// generating a Prover.toml file and executing the necessary narog commands.
// You must have nargo installed in order to use it.

use crate::Error;
use ark_bn254::Fr as Fq; // Fr (scalar field) of BN254 is the Fq (base field) of Babyjubjub
use ark_std::str::FromStr; // import to use from_str in structs
use num::{BigUint, Num};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

/// Hashes two Fq elements using nargo execution.
pub fn hash_two_fq(input1: Fq, input2: Fq) -> Result<Fq, Error> {
    // Step 1: Create the Prover.toml file
    create_prover_toml(input1, input2)?;

    // Step 2: Execute nargo and retrieve the hash
    let hash_str_hex = execute_nargo_and_get_hash()?;

    // Step 3: transfer the string hash to Fq elemenet
    let hash = hex_to_fq(&hash_str_hex)?;

    Ok(hash)
}

/// Creates a Prover.toml file with the given Fq inputs to hash.
fn create_prover_toml(input1: Fq, input2: Fq) -> io::Result<()> {
    // Convert Fq elements to strings
    let input1_str = input1.to_string();
    let input2_str = input2.to_string();

    // Create and write the Prover.toml file
    let mut file = File::create("noir-src/Prover.toml")?;
    writeln!(file, "input1 = \"{}\"", input1_str)?;
    writeln!(file, "input2 = \"{}\"", input2_str)?;

    Ok(())
}

/// Executes the nargo command and retrieves the hash output.
fn execute_nargo_and_get_hash() -> io::Result<String> {
    // Execute the "nargo execute" command
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir("noir-src") // Set the working directory
        .stdout(Stdio::piped()) // Capture stdout to avoid printing to terminal
        .stderr(Stdio::null()) // Suppress stderr output
        .spawn()? // Spawn the command
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture stdout"))?;

    // Read the first line from the command's output
    let reader = BufReader::new(output);
    if let Some(first_line) = reader.lines().next() {
        return first_line;
    }

    Err(io::Error::new(io::ErrorKind::Other, "No output from nargo"))
}

/// Converts a hexadecimal string to an Fq element.
fn hex_to_fq(hex_string: &str) -> Result<Fq, Error> {
    // Strip '0x' prefix if present
    let hex_str = if hex_string.starts_with("0x") || hex_string.starts_with("0X") {
        &hex_string[2..] // Remove the prefix
    } else {
        hex_string // Use the original string if no prefix
    };

    // Convert hex string to BigUint
    let x_decimal = BigUint::from_str_radix(hex_str, 16)?;

    // Convert BigUint to Fq
    let x = Fq::from(x_decimal);

    Ok(x)
}

/// Checks if a string is a valid hexadecimal format.
fn is_hex(s: &str) -> bool {
    s.starts_with("0x") // Check for hexadecimal prefix
        || s.starts_with("0X")
        || s.chars()
            .any(|c| c.is_ascii_hexdigit() && c.is_alphabetic()) // Check for presence of hex letters
}

/// Converts a string input to an Fq element, interpreting hex and decimal formats.
pub fn get_fq_element(input: &str) -> Result<Fq, Error> {
    if is_hex(input) {
        hex_to_fq(input)
    } else {
        Fq::from_str(input).map_err(|_| "Invalid decimal input".into())
    }
}

/// Converts an Fq element to a hexadecimal string representation.
pub fn fq_to_str_hex(fq: &Fq) -> String {
    // Convert Fq to a decimal string
    let fq_string = fq.to_string();

    // Parse the decimal string into a hex
    let fq_decimal = BigUint::parse_bytes(fq_string.as_bytes(), 10).unwrap();

    // Return the hex string formatted with leading zeros
    format!("{:0>64x}", fq_decimal)
}
