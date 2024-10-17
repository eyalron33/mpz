use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mpz")]
#[command(version = "1.0")]
#[command(
    about = "CLI tool for creating a Merkle tree using the Pedersen Hash (Aztec implementation)."
)]
pub struct Cli {
    /// Output the Merkle tree in JSON format.
    #[arg(short = 'j', long = "json")]
    pub json: bool,

    /// Specify the output file for the JSON data.
    /// If not provided, it uses the default from `consts.rs`.
    #[arg(short = 'o', long = "output", requires = "json")]
    pub output: Option<String>,
}
