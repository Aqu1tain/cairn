use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, Read, Write};

use crate::element::DecodedElement;

/// Read variable-length integer from byte stream
pub fn read_var_length<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut result = 0;
    let mut count = 0;
    
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        
        result += ((byte[0] & 0x7F) as u32) << (count * 7);
        count += 1;
        
        if (byte[0] >> 7) == 0 {
            break;
        }
    }
    
    Ok(result)
}

/// Write variable-length integer to byte stream
pub fn write_var_length<W: Write>(writer: &mut W, mut n: u32) -> io::Result<()> {
    let mut bytes = Vec::new();
    
    while n > 0x7F {
        bytes.push((n as u8 & 0x7F) | 0x80);
        n >>= 7;
    }
    
    bytes.push(n as u8);
    
    writer.write_all(&bytes)
}

/// Read string from byte stream
pub fn read_string<R: Read>(reader: &mut R) -> io::Result<String> {
    let length = read_var_length(reader)? as usize;
    let mut bytes = vec![0u8; length];
    reader.read_exact(&mut bytes)?;
    
    String::from_utf8(bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Write string to byte stream
pub fn write_string<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    write_var_length(writer, s.len() as u32)?;
    writer.write_all(s.as_bytes())
}

/// Read run-length encoded string from byte stream
pub fn read_run_length_encoded<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut byte_count = [0u8; 2];
    reader.read_exact(&mut byte_count)?;
    let byte_count = u16::from_le_bytes(byte_count) as usize;
    
    let mut data = vec![0u8; byte_count];
    reader.read_exact(&mut data)?;
    
    let mut result = String::new();
    
    for i in (0..byte_count).step_by(2) {
        let times = data[i] as usize;
        let character = data[i + 1] as char;
        result.push_str(&character.to_string().repeat(times));
    }
    
    Ok(result)
}

/// Encode string using run-length encoding
pub fn encode_run_length(s: &str) -> Option<Vec<u8>> {
    // Only allow run length encoding if the string contains only single-byte characters
    if s.chars().any(|c| c as u32 > 0xFF) {
        return None;
    }
    
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    
    if bytes.is_empty() {
        return Some(result);
    }
    
    let mut count: u8 = 1;
    let mut current = bytes[0];
    
    for &byte in &bytes[1..] {
        if byte != current || count == 255 {
            result.push(count);
            result.push(current);
            
            count = 1;
            current = byte;
        } else {
            count += 1;
        }
    }
    
    result.push(count);
    result.push(current);
    
    Some(result)
}

/// Decode value from byte stream based on type code
pub fn decode_value<R: Read>(type_byte: u8, lookup: &[String], reader: &mut R) -> io::Result<Value> {
    match type_byte {
        0 => {
            let mut value = [0u8; 1];
            reader.read_exact(&mut value)?;
            Ok(Value::Bool(value[0] != 0))
        }
        1 => {
            let mut value = [0u8; 1];
            reader.read_exact(&mut value)?;
            Ok(Value::Number(value[0].into()))
        }
        2 => {
            let mut value = [0u8; 2];
            reader.read_exact(&mut value)?;
            Ok(Value::Number(i16::from_le_bytes(value).into()))
        }
        3 => {
            let mut value = [0u8; 4];
            reader.read_exact(&mut value)?;
            Ok(Value::Number(i32::from_le_bytes(value).into()))
        }
        4 => {
            let mut value = [0u8; 4];
            reader.read_exact(&mut value)?;
            let float = f32::from_le_bytes(value);
            
            // Handle JSON serialization of floating point values
            if float.is_finite() {
                Ok(json!(float))
            } else {
                Ok(Value::Null)
            }
        }
        5 => {
            let mut index = [0u8; 2];
            reader.read_exact(&mut index)?;
            let index = u16::from_le_bytes(index) as usize;
            
            if index < lookup.len() {
                Ok(Value::String(lookup[index].clone()))
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid lookup index"))
            }
        }
        6 => {
            let s = read_string(reader)?;
            Ok(Value::String(s))
        }
        7 => {
            let s = read_run_length_encoded(reader)?;
            Ok(Value::String(s))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid value type"))
    }
}

/// Encode value to byte stream with appropriate type code
pub fn encode_value<W: Write>(writer: &mut W, _key: &str, value: &Value, lookup: &HashMap<String, usize>) -> io::Result<()> {
    match value {
        Value::Bool(b) => {
            writer.write_all(&[0])?;
            writer.write_all(&[*b as u8])?;
        }
        Value::Number(n) => {
            if let Some(n_u8) = n.as_u64().and_then(|n| u8::try_from(n).ok()) {
                writer.write_all(&[1])?;
                writer.write_all(&[n_u8])?;
            } else if let Some(n_i16) = n.as_i64().and_then(|n| i16::try_from(n).ok()) {
                writer.write_all(&[2])?;
                writer.write_all(&n_i16.to_le_bytes())?;
            } else if let Some(n_i32) = n.as_i64().and_then(|n| i32::try_from(n).ok()) {
                writer.write_all(&[3])?;
                writer.write_all(&n_i32.to_le_bytes())?;
            } else if let Some(n_f32) = n.as_f64().and_then(|n| {
                if n >= f32::MIN as f64 && n <= f32::MAX as f64 {
                    Some(n as f32)
                } else {
                    None
                }
            }) {
                writer.write_all(&[4])?;
                writer.write_all(&n_f32.to_le_bytes())?;
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Number out of range"));
            }
        }
        Value::String(s) => {
            if let Some(&index) = lookup.get(s) {
                writer.write_all(&[5])?;
                writer.write_all(&(index as u16).to_le_bytes())?;
            } else if let Some(encoded) = encode_run_length(s) {
                let encoded_len = encoded.len();
                
                if encoded_len < s.len() && encoded_len <= u16::MAX as usize {
                    writer.write_all(&[7])?;
                    writer.write_all(&(encoded_len as u16).to_le_bytes())?;
                    writer.write_all(&encoded)?;
                } else {
                    writer.write_all(&[6])?;
                    write_string(writer, s)?;
                }
            } else {
                writer.write_all(&[6])?;
                write_string(writer, s)?;
            }
        }
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported value type"))
    }
    
    Ok(())
}

/// Decode element from byte stream
pub fn decode_element<R: Read>(reader: &mut R, lookup: &[String]) -> io::Result<DecodedElement> {
    let mut index = [0u8; 2];
    reader.read_exact(&mut index)?;
    let name_index = u16::from_le_bytes(index) as usize;
    
    if name_index >= lookup.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid element name index"));
    }
    
    let name = lookup[name_index].clone();
    
    let mut attribute_count = [0u8; 1];
    reader.read_exact(&mut attribute_count)?;
    let attribute_count = attribute_count[0] as usize;
    
    let mut attributes = HashMap::new();
    
    for _ in 0..attribute_count {
        let mut key_index = [0u8; 2];
        reader.read_exact(&mut key_index)?;
        let key_index = u16::from_le_bytes(key_index) as usize;
        
        if key_index >= lookup.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid attribute key index"));
        }
        
        let key = lookup[key_index].clone();
        
        let mut type_byte = [0u8; 1];
        reader.read_exact(&mut type_byte)?;
        
        let value = decode_value(type_byte[0], lookup, reader)?;
        attributes.insert(key, value);
    }
    
    let mut child_count = [0u8; 2];
    reader.read_exact(&mut child_count)?;
    let child_count = u16::from_le_bytes(child_count) as usize;
    
    let children = if child_count > 0 {
        let mut children = Vec::with_capacity(child_count);
        
        for _ in 0..child_count {
            let child = decode_element(reader, lookup)?;
            children.push(child);
        }
        
        Some(children)
    } else {
        None
    };
    
    Ok(DecodedElement {
        name,
        attributes,
        children,
    })
}

/// Encode element to byte stream
pub fn encode_element<W: Write>(writer: &mut W, element: &DecodedElement, lookup: &HashMap<String, usize>) -> io::Result<()> {
    let name_index = lookup.get(&element.name).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Element name not in lookup table")
    })?;
    
    writer.write_all(&(*name_index as u16).to_le_bytes())?;
    
    // Filter out special attributes
    let attributes: HashMap<_, _> = element.attributes.iter()
        .filter(|(k, _)| !k.starts_with("__"))
        .filter(|(_, v)| !v.is_null())
        .collect();
    
    writer.write_all(&[attributes.len() as u8])?;
    
    for (attr, value) in &attributes {
        let attr_index = lookup.get(attr.as_str()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Attribute name {} not in lookup table", attr))
        })?;
        
        writer.write_all(&(*attr_index as u16).to_le_bytes())?;
        encode_value(writer, attr, value, lookup)?;
    }
    
    let children = element.children.as_ref().map(|c| c.as_slice()).unwrap_or(&[]);
    writer.write_all(&(children.len() as u16).to_le_bytes())?;
    
    for child in children {
        encode_element(writer, child, lookup)?;
    }
    
    Ok(())
}