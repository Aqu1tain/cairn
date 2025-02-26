# Cairn - Celeste Map Encoder/Decoder

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
git clone https://github.com/Aqu1tain/cairn.git
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

## How It Works

Cairn converts between Celeste's binary map format and JSON by implementing the custom binary format specification used by the game.

### Celeste Map Format Overview

Celeste's map format consists of:

1. **Header** - "CELESTE MAP" string identifier
2. **Package Name** - The map's package identifier
3. **String Lookup Table** - A lookup table for deduplicating strings
4. **Hierarchical Element Tree** - The actual map data

Each element in the map structure has:
- A name
- Attributes (key-value pairs)
- Child elements

### Binary Format Details

Values in the binary format are encoded with type codes:

| Type Code | Data Type | Description |
|-----------|-----------|-------------|
| 0 | Boolean | True/False value |
| 1 | UInt8 | Single byte integer |
| 2 | Int16 | 2-byte signed integer |
| 3 | Int32 | 4-byte signed integer |
| 4 | Float32 | 4-byte floating point number |
| 5 | String Reference | Index into the string lookup table |
| 6 | Raw String | Length-prefixed string |
| 7 | Run-length Encoded String | Compressed string, primarily for tile data |

The encoder chooses the most efficient representation for each value. For example, small integers use the UInt8 type, while larger ones use Int16 or Int32. Strings that appear multiple times are stored in the lookup table and referenced by index.

### Variable Length Integer Encoding

For run-length encoded strings and raw strings, the length is encoded as a variable-length integer, where:
- Each byte uses 7 bits for data
- The high bit indicates if more bytes follow
- This allows efficient encoding of both small and large values

## Project Structure

```
cairn/
├── src/
│   ├── main.rs         # Command-line interface
│   ├── lib.rs          # Public API and module exports
│   ├── element.rs      # DecodedElement struct definition
│   ├── binary.rs       # Binary encoding/decoding utilities
│   └── map.rs          # Map conversion functions
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── CONTRIBUTING.md     # Contribution guidelines
```

### File Descriptions

#### src/element.rs

Contains the `DecodedElement` struct definition, which represents a single element in the Celeste map hierarchy. This is the central data structure used throughout the application.

```rust
pub struct DecodedElement {
    pub name: String,
    pub attributes: HashMap<String, Value>,
    pub children: Option<Vec<DecodedElement>>,
}
```

#### src/binary.rs

Implements the low-level binary encoding and decoding utilities:
- Reading and writing variable-length integers
- String encoding and decoding
- Run-length encoding and decoding
- Value type handling
- Element tree serialization and deserialization

The functions in this file handle the binary format details without being concerned with file I/O or the higher-level map structure.

#### src/map.rs

Contains high-level map operations that use the binary utilities:
- `decode_map()` - Reads a binary map file and converts it to a DecodedElement
- `encode_map()` - Writes a DecodedElement to a binary map file
- `bin_to_json()` - Converts a binary map file to JSON
- `json_to_bin()` - Converts a JSON file to a binary map

These functions handle file I/O and the overall conversion process.

#### src/lib.rs

Defines the public API for the library by re-exporting the relevant types and functions. This is what other Rust projects will use when depending on Cairn.

#### src/main.rs

Implements the command-line interface, parsing arguments and calling the appropriate library functions.

## Acknowledgments

Cairn was inspired by [Maple](https://github.com/CelestialCartographers/Maple), the original Celeste map encoder/decoder written in Julia by the Celestial Cartographers team. Many thanks to their work for documenting and implementing the Celeste map format, which made this Rust implementation possible.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed information on how to contribute to Cairn.

## License

This project is licensed under the MIT License - see the LICENSE file for details.