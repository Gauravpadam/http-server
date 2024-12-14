use std::collections::HashMap;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    Unknown,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: Option<String>,
    pub http_version: Option<String>,
    pub request_headers: HashMap<String, String>,
    pub request_body: String,
}

impl HttpRequest {
    pub fn new(data: &[u8]) -> Self {
        let method;
        let uri;
        let http_version;
        let request_headers;
        let request_body;

        (request_headers, request_body, method, uri, http_version) = Self::parse(data);

        HttpRequest {
            method,
            uri,
            http_version,
            request_headers,
            request_body,
        }
    }

    fn make_request_parts(request: &str) -> (String, HashMap<String, String>, String) {
        let mut request_line = String::new();
        let mut request_headers: HashMap<String, String> = HashMap::new();
        let mut start_body: usize = 0;
        let mut request_body = String::new();

        let request_parts: Vec<&str> = request.split("\r\n").collect();

        request_line = request_parts[0].to_string();

        for i in 1..request_parts.len() {
            if let Some((key, value)) = request_parts[i].split_once(": ") {
                request_headers.insert(key.to_string(), value.to_string());
            } else {
                start_body = i;
                break;
            }
        }

        for j in start_body..request_parts.len() {
            request_body += request_parts[j];
        }

        (request_line, request_headers, request_body)
    }

    fn read_request_type(request_line: String) -> HttpMethod {
        match request_line.split_whitespace().next() {
            Some("GET") => HttpMethod::GET,
            Some("POST") => HttpMethod::POST,
            Some("PATCH") => HttpMethod::PATCH,
            Some("PUT") => HttpMethod::PUT,
            Some("DELETE") => HttpMethod::DELETE,
            _ => HttpMethod::Unknown,
        }
    }

    fn parse(
        bytestream: &[u8],
    ) -> (
        HashMap<String, String>,
        String,
        HttpMethod,
        Option<String>,
        Option<String>,
    ) {
        // println!("Bytestream: {:?}", std::str::from_utf8(data));
        let request: &str =
            std::str::from_utf8(bytestream).expect("Expected a valid uf8 bytestream");

        let request_line: String;
        let request_headers: HashMap<String, String>;
        let request_body: String;
        let method: HttpMethod;
        let mut uri: Option<String> = None;
        let mut http_version: Option<String> = None;

        (request_line, request_headers, request_body) = Self::make_request_parts(request);

        // Debug:
        // println!("Request line: {}", request_line);
        // println!("Request headers: {:?}", request_headers);
        // println!("Request body: {}", request_body);

        let words: Vec<&str> = request_line.split(" ").collect();

        method = Self::read_request_type(words[0].to_string());

        if words.len() > 1 {
            uri = Some(words[1].to_string());
        }

        if words.len() > 2 {
            http_version = Some(words[2].to_string());
        }

        (request_headers, request_body, method, uri, http_version)
    }
}
