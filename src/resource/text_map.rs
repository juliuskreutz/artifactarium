//! Parse a SRData text map
//!
//! Text maps are indexed using a hash, refer to [`TextMapEntry`].

use std::collections::HashMap;
use serde::Deserialize;
use crate::resource::TextMapEntry;

#[derive(Deserialize)]
pub struct TextMap(pub HashMap<u32, String>);

impl TextMap {
    pub fn get(&self, text_map_string: &TextMapEntry) -> Option<&str> {
        self.0.get(&text_map_string.0)
            .map(|s| s.as_str())
    }
}