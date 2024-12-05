use std::collections::HashMap;

pub struct Mime {
    mime: Option<&'static str>,
}

impl Mime {
    pub fn new(ext: &str) -> Self {
        Mime {
            mime: Self::get_mimetype(ext),
        }
    }

    fn get_mimetype(ext: &str) -> Option<&'static str> {
        match ext {
            "html" => Some("text/html"),
            "css" => Some("text/css"),
            "js" => Some("text/javascript"),
            "txt" => Some("text/plain"),
            "jpg" => Some("image/jpg"),
            "jpeg" => Some("image/jpeg"),
            "png" => Some("image/png"),
            "bmp" => Some("image/bmp"),
            "csv" => Some("text/csv"),
            _ => None,
        }
    }

    pub fn mimetype_to_hashmap(&self) -> Option<HashMap<String, String>> {
        if let Some(mime) = self.mime {
            let mut mime_hashmap = HashMap::new();
            mime_hashmap.insert("Content-Type".to_string(), mime.to_string());
            Some(mime_hashmap)
        } else {
            None
        }
    }
}
