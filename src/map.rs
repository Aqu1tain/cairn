use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::binary::{decode_element, encode_element, read_string, write_string};
use crate::element::DecodedElement;

/// Decode binary Celeste map to structure
pub fn decode_map<P: AsRef<Path>>(path: P) -> io::Result<DecodedElement> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Read header
    let header = read_string(&mut reader)?;
    if header != "CELESTE MAP" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Celeste map file"));
    }
    
    let package = read_string(&mut reader)?;
    
    let mut lookup_length = [0u8; 2];
    reader.read_exact(&mut lookup_length)?;
    let lookup_length = u16::from_le_bytes(lookup_length) as usize;
    
    let mut lookup = Vec::with_capacity(lookup_length);
    for _ in 0..lookup_length {
        let s = read_string(&mut reader)?;
        lookup.push(s);
    }
    
    let mut map = decode_element(&mut reader, &lookup)?;
    map.attributes.insert("package".to_string(), Value::String(package));
    
    Ok(map)
}

/// Encode structure to binary Celeste map
pub fn encode_map<P: AsRef<Path>>(map: &DecodedElement, path: P) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    // Get package from metadata
    let package = match map.attributes.get("package") {
        Some(Value::String(s)) => s.clone(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing package attribute",
            ));
        }
    };
    
    // Collect all strings for lookup table
    let mut seen = HashSet::new();
    map.collect_keys(&mut seen);
    
    let lookup: Vec<_> = seen.into_iter().collect();
    let lookup_map: HashMap<_, _> = lookup.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
    
    // Write header
    write_string(&mut writer, "CELESTE MAP")?;
    write_string(&mut writer, &package)?;
    
    // Write lookup table
    writer.write_all(&(lookup.len() as u16).to_le_bytes())?;
    for s in &lookup {
        write_string(&mut writer, s)?;
    }
    
    // Write map data
    encode_element(&mut writer, map, &lookup_map)?;
    
    Ok(())
}

/// Convert binary map to JSON
pub fn bin_to_json<P: AsRef<Path>, Q: AsRef<Path>>(bin_path: P, json_path: Q) -> io::Result<()> {
    let map = decode_map(bin_path)?;
    let json = serde_json::to_string_pretty(&map)?;
    
    let mut file = File::create(json_path)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

/// Convert JSON to binary map
pub fn json_to_bin<P: AsRef<Path>, Q: AsRef<Path>>(json_path: P, bin_path: Q) -> io::Result<()> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let map: DecodedElement = serde_json::from_reader(reader)?;
    
    encode_map(&map, bin_path)?;
    
    Ok(())
}