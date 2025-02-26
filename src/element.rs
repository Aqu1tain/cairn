use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Represents a decoded element from a Celeste map file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecodedElement {
    #[serde(rename = "__name")]
    pub name: String,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
    #[serde(rename = "__children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DecodedElement>>,
}

impl DecodedElement {
    /// Creates a new DecodedElement with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: HashMap::new(),
            children: None,
        }
    }

    /// Collect all string keys for lookup table
    pub fn collect_keys(&self, seen: &mut HashSet<String>) {
        seen.insert(self.name.clone());
        
        for (key, value) in &self.attributes {
            if !key.starts_with("__") {
                seen.insert(key.clone());
            }
            
            if let Value::String(s) = value {
                seen.insert(s.clone());
            }
        }
        
        if let Some(children) = &self.children {
            for child in children {
                child.collect_keys(seen);
            }
        }
    }
}