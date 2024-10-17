// This module implements a Merkle tree construction using Pedersen hashes as implemented by Aztec.

use crate::hashing::{fq_to_str_hex, hash_two_fq};
use crate::Error;
pub use ark_bn254::Fr as Fq; // Fr (scalar field) of BN254 is the Fq (base field) of Babyjubjub

/// Creates a Merkle tree using the provided Fq elements and returns the root, proof paths, and indices.
pub fn create_tree_with_pedersen(inputs: &Vec<Fq>) -> Result<(Fq, Vec<Vec<Fq>>, Vec<u64>), Error> {
    if inputs.is_empty() {
        return Err("No inputs provided.".into());
    }

    // Initialize the current level with the input Fq elements
    let mut current_level: Vec<Fq> = inputs.clone();
    let mut proof_paths: Vec<Vec<Fq>> = vec![Vec::new(); inputs.len()]; // To store paths for each member
    let mut index_bits: Vec<Vec<u8>> = vec![Vec::new(); inputs.len()]; // To store indices for each member

    // Initialize leaf indices, where each leaf initially corresponds to itself
    let mut orig_leaf_indices: Vec<Vec<usize>> = (0..inputs.len()).map(|i| vec![i]).collect();

    let mut level = 0;

    println!("Merkle tree construction:");

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        let mut next_orig_leaf_indices = Vec::new(); // For tracking the next level's leaf indices
        let mut i = 0;

        println!("\nLevel {}:", level);

        // Process pairs of elements
        while i + 1 < current_level.len() {
            // Hash the current pair of nodes
            let hash = hash_two_fq(current_level[i], current_level[i + 1])?;
            println!(
                "Leaf {} and Leaf {} -> Hash {}",
                i,
                i + 1,
                fq_to_str_hex(&hash)
            );
            next_level.push(hash);

            // For each original leaf that contributed to the current pair, add the sibling and the direction (left or right) to the proof path and index
            for leaf_index in &orig_leaf_indices[i] {
                proof_paths[*leaf_index].push(current_level[i + 1].clone()); // The sibling node
                index_bits[*leaf_index].insert(0, 0); // Current node is on the left, sibling is on the right
            }
            for leaf_index in &orig_leaf_indices[i + 1] {
                proof_paths[*leaf_index].push(current_level[i].clone()); // The sibling node
                index_bits[*leaf_index].insert(0, 1); // Current node is on the right, sibling is on the left
            }

            // Merge the leaf indices from the two nodes and store them for the next level
            let mut merged_indices = orig_leaf_indices[i].clone();
            merged_indices.extend_from_slice(&orig_leaf_indices[i + 1]);
            next_orig_leaf_indices.push(merged_indices);

            i += 2
        }

        // If there's an odd number of elements, propagate the last node and its indices unchanged
        if i < current_level.len() {
            println!(
                "Leaf {} (unpaired) -> Moves up as {}",
                i,
                fq_to_str_hex(&current_level[i])
            );
            next_level.push(current_level[i]);

            // Just propagate the original leaf index to the next level
            next_orig_leaf_indices.push(orig_leaf_indices[i].clone());
        }

        // Move to the next level
        current_level = next_level;
        orig_leaf_indices = next_orig_leaf_indices;
        level += 1;
    }

    // Convert bit vectors to integers using the provided bits_to_integer function
    let index: Vec<u64> = index_bits.into_iter().map(bits_to_integer).collect();

    // The last remaining element is the Merkle root
    println!("\nMerkle root is: {}", fq_to_str_hex(&current_level[0]));

    Ok((current_level[0], proof_paths, index))
}

/// Converts a vector of bits to a u64 integer representation.
fn bits_to_integer(bits: Vec<u8>) -> u64 {
    let mut result = 0u64;

    // Iterate over the bits and convert them to an integer
    for (i, &bit) in bits.iter().rev().enumerate() {
        if bit == 1 {
            result += 1 << i;
        }
    }

    result
}
