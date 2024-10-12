# Ethereum Vanity Wallet Generator

This Rust-based tool generates Ethereum wallets with customizable addresses based on user-defined criteria. It's designed for creating vanity addresses or addresses with specific patterns.

## Features

- Multi-threaded wallet generation for improved performance
- Customizable address matching using prefixes, suffixes, or patterns from a file
- Generates BIP39 compliant mnemonic phrases for each wallet
- Supports various selection criteria for wallet addresses

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/hambosto/ethereum-vanity.git
   cd ethereum-vanity-rs
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the program with the following command:

```
cargo run --release -- [OPTIONS]
```

### Options:

- `-w, --workers <NUMBER>`: Set the number of worker threads (default: number of CPU cores minus 1)
- `-i, --input <FILE>`: Use a file containing 4-letter words to match address prefixes and suffixes
- `-p, --prefix <PREFIX>`: Match addresses starting with a specific prefix
- `-s, --suffix <SUFFIX>`: Match addresses ending with a specific suffix

### Examples:

1. Generate wallets with addresses starting with "1337":
   ```
   cargo run --release -- --prefix 1337
   ```

2. Generate wallets with addresses ending with "cafe":
   ```
   cargo run --release -- --suffix cafe
   ```

3. Use a custom word list file for matching:
   ```
   cargo run --release -- --input my_words.txt
   ```

4. Specify the number of worker threads:
   ```
   cargo run --release -- --workers 8 --prefix 0x
   ```

## Output

The program will output matching wallet addresses and their corresponding mnemonic phrases to the console. It also provides periodic updates on the number of wallets checked and the check rate.

## Safety and Security

- This tool is for educational and experimental purposes only.
- Never share your private keys or mnemonic phrases.
- Always verify the security of generated wallets before using them with real funds.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This software is provided "as is", without warranty of any kind. Use at your own risk.
