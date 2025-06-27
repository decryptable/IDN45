# IDN45

[![Build & Release IDN45](https://github.com/decryptable/IDN45/actions/workflows/build.yml/badge.svg)](https://github.com/decryptable/IDN45/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**IDN45** is an educational implementation of a modern cryptographic hash function built in Rust.

This project serves as a practical exploration of cryptographic principles, demonstrating how to build a performant, cross-platform CLI tool and library in Rust.

---

## 🛑 CRITICAL SECURITY WARNING

This implementation is for **educational and demonstrative purposes ONLY**. It has not undergone the rigorous, public, and academic peer review that standard algorithms like SHA-256 or SHA-3 have.

**DO NOT USE THIS IN A PRODUCTION ENVIRONMENT** for any security-critical application. The shorter hash formats (`uuid` and `short`) are especially insecure against collision attacks. For real-world use cases, always use established, well-vetted cryptographic libraries like the `sha2` or `sha3` crates.

---

## Features

- **Modern Cryptographic Design**: Implements a Sponge Construction similar to Keccak/SHA-3.
- **Flexible Output Formats**:
  - `standard`: A secure 256-bit hash (64 hex characters).
  - `uuid`: A 96-bit hash formatted for unique IDs (`IDN45-xxxxxxxx-xxxxxxxx-xxxxxxxx`).
  - `short`: A 44-bit hash for decorative purposes (`xxxx:xxxxxxx`).
- **Powerful CLI**: A full-featured command-line interface for hashing and validating text, files, and piped data.
- **Cross-Platform**: The GitHub Actions workflow automatically builds binaries for Linux, macOS (Intel & Apple Silicon), and Windows.
- **Usable as a Library**: Can be easily integrated into other Rust projects.

## Installation

### From Source

Ensure you have the Rust toolchain installed. Then, clone the repository and build the project:

```sh
git clone https://github.com/decryptable/IDN45.git
cd IDN45
cargo build --release
```

The binary will be located at `target/release/idn45`. You can copy this file to a directory in your system's `PATH` (e.g., `/usr/local/bin` or `~/.cargo/bin`).

### From Pre-compiled Binaries

You can download the latest pre-compiled executables for your operating system from the **[Actions tab](https://www.google.com/search?q=https://github.com/decryptable/IDN45/actions)**. Look for the latest successful run of the "Build & Release IDN45" workflow and download the appropriate artifact.

## CLI Usage

The CLI is structured with subcommands for hashing and validating.

```sh
# Display the main help message
idn45 --help

# Display help for the 'hash' subcommand
idn45 hash --help
```

### Hashing

```sh
# Hash a string with the default standard format (256-bit)
idn45 hash --text "Hello, Rust!"

# Hash a file with the UUID format
idn45 hash --file ./Cargo.toml --format uuid

# Hash a string with the short format and a salt
idn45 hash --text "secret_password" --format short --salt "my-secret-salt"

# Hash data from stdin (piping)
echo "Piped data" | idn45 hash
```

### Validation

```sh
# 1. First, generate a hash
HASH=$(idn45 hash --text "my data to validate")

# 2. Then, validate it
idn45 validate --hash "$HASH" --text "my data to validate"
# Output: Validation successful: Hash matches the input data.

# 3. An incorrect validation will produce an error
idn45 validate --hash "$HASH" --text "wrong data"
# Output: Error: Validation failed: Hash does not match the input data.
```

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.
