# Merkle Tree CLI Tool

This CLI tool generates Merkle trees using the Pedersen hash based on the Aztec implementation. The hashing process is executed through a Noir program, utilizing the `nargo` tool to calculate the hash and retrieve the output.

## Features

- Generates Merkle trees from user-provided Fq elements.
- Computes hashes using the Pedersen hash implementation in Noir.
- Outputs the Merkle tree root and proof paths to the console or as a JSON file.

## Dependencies

This tool has been tested with the following versions:
- `nargo`: 0.35.0
- `noirc`: compatible with the above nargo version.

Make sure to have these installed to use the CLI tool effectively.

## Usage

### Command-Line Options

- `--json` or `-j`: Outputs the Merkle tree data as a JSON file instead of printing it to the console.
- `--output <filename>`: Specifies the output filename for the JSON data. If not provided, it defaults to `merkle_tree_output.json`.

### Example Commands

To run the program, use the following command in your terminal:

```bash
cargo run -- --json --output my_output.json
```

##This command will collect Fq elements from the user, generate a Merkle tree, and save the output as a JSON file named my_output.json.

## How the Merkle Tree is Built

The program constructs a Merkle tree by iterating through the input Fq elements in pairs. It calculates the hash for each pair using the Pedersen hash and adds the resulting hash to the next level of the tree. If there is an odd number of elements, the last unpaired element is propagated up to the next level unchanged. This ensures that all elements contribute to the final Merkle root.

## Implementation Details

The hashing process utilizes the Noir programming language, as implementing the Pedersen hash directly in Rust posed significant challenges. The Aztec implementation of the Pedersen hash lacked detailed specifications, particularly regarding the choice of generators used in their hash function. As reverse engineering the implementation proved to be too complex and time-consuming, wrapping the Noir implementation was chosen as a more practical solution.

## Example Noir Program
An example Noir program to verify the Merkle tree is given in `noir_pedersen` folder.

## License

This project is licensed under the MIT License - see the LICENSE file for details.