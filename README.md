# cairn - Celeste Map Encoder/Decoder

A Rust library and command-line tool for converting Celeste map files between binary (.bin) and JSON formats.

## Features

- Convert binary Celeste maps to human-readable JSON
- Convert JSON back to binary format compatible with Celeste
- Standalone command-line utility
- Library API for integration into other Rust projects

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Aquitain/cairn.git
cd cairn

# Build in release mode
cargo build --release

# The binary will be in target/release/cairn
```

## Usage

### Command Line Interface

```bash
# Convert a binary map to JSON
cairn bin2json path/to/map.bin path/to/output.json

# Convert a JSON file to binary map format
cairn json2bin path/to/map.json path/to/output.bin
```

If you don't specify an output file, the tool will use the input filename with the appropriate extension:

```bash
# Will output to 1-ForsakenCity.json
cairn bin2json 1-ForsakenCity.bin

# Will output to mymap.bin
cairn json2bin mymap.json
```

### Using the Library in Your Rust Projects

Add this to your `Cargo.toml`:

```toml
[dependencies]
cairn = "0.1.0"
```

Then in your code:

```rust
use cairn::{bin_to_json, json_to_bin, decode_map, encode_map, DecodedElement};

fn main() -> std::io::Result<()> {
    // Convert bin to json
    bin_to_json("input.bin", "output.json")?;

    // Convert json to bin
    json_to_bin("input.json", "output.bin")?;

    // Or work with the map structure directly
    let map = decode_map("input.bin")?;
    
    // Modify the map...
    
    // Save back to binary
    encode_map(&map, "modified.bin")?;
    
    Ok(())
}
```

## File Format

Celeste map files use a custom binary format that includes:

- A string lookup table for efficient storage
- Type-encoded values (boolean, integer, float, string)
- Run-length encoding for efficient tile storage
- Hierarchical structure of elements with attributes and children

## License

This project is licensed under the MIT License.