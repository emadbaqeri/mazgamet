# Mazgamet

[![CI](https://github.com/emadbaqeri/mazgamet/actions/workflows/ci.yml/badge.svg)](https://github.com/emadbaqeri/mazgamet/actions/workflows/ci.yml)
[![Security audit](https://github.com/emadbaqeri/mazgamet/actions/workflows/audit.yml/badge.svg)](https://github.com/emadbaqeri/mazgamet/actions/workflows/audit.yml)
[![Crates.io](https://img.shields.io/crates/v/lexer.svg)](https://crates.io/crates/lexer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lexical analyzer and parsing tools library for building compilers and interpreters.

## Features

- Fast and memory-efficient lexical analysis
- Customizable tokenization rules
- Comprehensive error reporting
- Simple API for integration with parsers

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
lexer = "0.1.0"
```

## Example Usage

```rust
// Coming soon!
```

## Development

### Prerequisites

- Rust 1.76 or later
- Cargo

### Building

```bash
cargo build --workspace
```

### Testing

```bash
cargo test --workspace
```

### Running Lints

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/) format
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to update tests as appropriate and follow the code style guidelines.

## Release Process

1. Update version numbers in all Cargo.toml files
2. Update CHANGELOG.md following the Keep a Changelog format
3. Create a new git tag with the version number (e.g., `v0.1.0`)
4. Push the tag to GitHub
5. The CI/CD pipeline will automatically create a GitHub release and publish to crates.io

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Thanks to all contributors and supporters