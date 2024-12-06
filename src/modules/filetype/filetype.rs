use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct FileType {
    extension: String,
    buffer_reader: Option<Box<BufReader<File>>>,
    mime: Option<&'static str>,
}

impl FileType {
    pub fn new(ext: &str, file_path: String) -> Self {
        FileType {
            extension: ext.to_string(),
            buffer_reader: Self::get_buffer_reader(file_path),
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

    fn get_buffer_reader(file_path: String) -> Option<Box<BufReader<File>>> {
        match File::open(file_path) {
            Ok(file) => Some(Box::new(BufReader::new(file))),
            Err(_) => None,
        }
    }

    pub fn read_file(&mut self) -> Option<Vec<u8>> {
        if let Some(reader) = self.buffer_reader.as_mut() {
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer) {
                Ok(_) => Some(buffer),
                Err(_) => None,
            }
        } else {
            None
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
