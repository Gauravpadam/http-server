use std::collections::HashMap;

pub fn mimetype_to_hashmap(mimetype: &str) -> Option<HashMap<String, String>> {
    if mimetype.is_empty() {
        // Can be a more practical condiditon of error
        None
    } else {
        let mut mime_hashmap = HashMap::new();
        mime_hashmap.insert("Content-Type".to_string(), mimetype.to_string());
        Some(mime_hashmap)
    }
}
