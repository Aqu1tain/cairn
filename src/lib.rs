// Main library module that re-exports public API

mod binary;
mod element;
mod map;

// Re-export the primary types and functions
pub use element::DecodedElement;
pub use map::{bin_to_json, decode_map, encode_map, json_to_bin};

// Lib crate version of the package
pub const VERSION: &str = env!("CARGO_PKG_VERSION");