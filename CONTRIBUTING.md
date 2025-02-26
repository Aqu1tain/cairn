# Contributing to Cairn

Thank you for your interest in contributing to Cairn! This document provides guidelines and instructions for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/cairn.git
   cd cairn
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL-OWNER/cairn.git
   ```
4. **Create a new branch** for your feature or bug fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Environment

### Prerequisites

- **Rust** (latest stable version recommended)
- **Cargo** (comes with Rust)

You can install Rust and Cargo using [rustup](https://rustup.rs/).

### Building the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

## Code Style Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
- Use `cargo fmt` to format your code before submitting
- Run `cargo clippy` to check for common mistakes and style issues
- Write meaningful commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) format
- Add documentation comments (`///`) to public functions and types

## Pull Request Process

1. **Update your fork** with the latest changes from upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run tests** to ensure your changes don't break existing functionality:
   ```bash
   cargo test
   ```

3. **Update documentation** if needed, including:
   - Code comments and documentation strings
   - README.md for user-facing changes
   - Update CHANGELOG.md if applicable

4. **Submit a pull request** to the main repository:
   - Provide a clear description of the changes
   - Reference any related issues
   - Explain how your changes benefit the project

5. **Address review feedback** promptly and make requested changes.

## Documentation Guidelines

- Use [rustdoc](https://doc.rust-lang.org/rustdoc/) comments for API documentation
- Include examples in documentation when appropriate
- Document both the "what" and the "why" of your code
- Keep documentation up-to-date with code changes

Example of good documentation:

```rust
/// Converts a binary Celeste map file to JSON format.
///
/// This function reads a binary map file, decodes its structure, and
/// writes the result as a formatted JSON file.
///
/// # Arguments
///
/// * `bin_path` - Path to the binary map file
/// * `json_path` - Path where the JSON output should be written
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `io::Error` if reading, decoding, or writing fails.
///
/// # Examples
///
/// ```no_run
/// use cairn::bin_to_json;
/// 
/// fn main() -> std::io::Result<()> {
///     bin_to_json("1-ForsakenCity.bin", "1-ForsakenCity.json")?;
///     Ok(())
/// }
/// ```
pub fn bin_to_json<P: AsRef<Path>, Q: AsRef<Path>>(bin_path: P, json_path: Q) -> io::Result<()> {
    // Implementation...
}
```

## Bug Reports and Feature Requests

- Use the GitHub issue tracker to report bugs or request features
- Provide detailed information for bug reports:
  - Steps to reproduce
  - Expected vs. actual behavior
  - Version of Cairn you're using
  - Operating system and version
- For feature requests, explain the use case and benefits

## Code of Conduct

- Be respectful and inclusive in your interactions
- Focus on constructive feedback
- Help maintain a positive community environment

## License

By contributing to Cairn, you agree that your contributions will be licensed under the same license as the project (MIT).