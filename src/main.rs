use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use idn45::{hasher::HashFormat, validate_hash, IDN45};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author = "Gemini")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Defines the main subcommands for the CLI tool.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Hashes input text or file content.
    Hash(HashArgs),
    /// Validates a given hash against input text or file content.
    Validate(ValidateArgs),
}

/// Arguments for the `hash` subcommand.
#[derive(Args, Debug)]
struct HashArgs {
    /// Input text to hash. Use this or --file.
    #[arg(long)]
    text: Option<String>,

    /// Path to the file to hash. Use this or --text.
    #[arg(long, value_name = "FILE_PATH")]
    file: Option<PathBuf>,

    /// The desired output format for the hash.
    #[arg(long, short, value_enum, default_value_t = CliHashFormat::Standard)]
    format: CliHashFormat,

    /// An optional salt to use for the hash, enhancing security.
    #[arg(long, short)]
    salt: Option<String>,
}

/// Arguments for the `validate` subcommand.
#[derive(Args, Debug)]
struct ValidateArgs {
    /// The full hash string to validate.
    #[arg(long, short)]
    hash: String,

    /// The original input text. Use this or --file.
    #[arg(long)]
    text: Option<String>,

    /// Path to the original file. Use this or --text.
    #[arg(long, value_name = "FILE_PATH")]
    file: Option<PathBuf>,

    /// The salt that was used to create the original hash.
    #[arg(long, short)]
    salt: Option<String>,
}

/// An enum for CLI options, decoupling it from the library's internal types.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum CliHashFormat {
    /// Standard 256-bit secure hash (64 hex characters).
    Standard,
    /// 96-bit hash formatted as a UUID.
    Uuid,
    /// 12-character/44-bit insecure hash for decorative purposes.
    Short,
}

/// Helper function to get input data from either text, file, or stdin.
fn get_input_data(text: Option<String>, file: Option<PathBuf>, from_stdin: bool) -> Result<Vec<u8>> {
    match (text, file, from_stdin) {
        (Some(t), None, false) => Ok(t.into_bytes()),
        (None, Some(f), false) => fs::read(&f).with_context(|| format!("Failed to read file: {}", f.display())),
        (None, None, true) => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            Ok(buffer)
        }
        _ => Err(anyhow!("Invalid input: Please provide input via --text, --file, or stdin (for hashing only), but not multiple at once.")),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // --- HASH COMMAND LOGIC ---
        Commands::Hash(args) => {
            // If neither --text nor --file is provided, read from stdin.
            let use_stdin = args.text.is_none() && args.file.is_none();
            let input_data = get_input_data(args.text, args.file, use_stdin)
                .context("Failed to get input for hashing")?;

            let salt_bytes = args.salt.as_deref().map(str::as_bytes);

            let lib_format = match args.format {
                CliHashFormat::Standard => HashFormat::Standard,
                CliHashFormat::Uuid => HashFormat::Uuid,
                CliHashFormat::Short => HashFormat::Short,
            };

            let hasher = IDN45::new(Some(&input_data), salt_bytes);
            let final_hash = hasher.hexdigest(lib_format);

            println!("{}", final_hash);
        }
        // --- VALIDATE COMMAND LOGIC ---
        Commands::Validate(args) => {
            let input_data = get_input_data(args.text, args.file, false)
                .context("Failed to get input for validation. You must provide --text or --file.")?;
            
            let salt_bytes = args.salt.as_deref().map(str::as_bytes);

            if validate_hash(&input_data, &args.hash, salt_bytes) {
                println!("Validation successful: Hash matches the input data.");
            } else {
                // Using anyhow to return a user-friendly error that exits with a non-zero status code.
                return Err(anyhow!("Validation failed: Hash does not match the input data."));
            }
        }
    }

    Ok(())
}
