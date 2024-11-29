pub struct HttpRequest {
    pub method: Option<String>,
    pub uri: Option<String>,
    pub http_version: Option<String>,
}

impl HttpRequest {
    pub fn new(data: &[u8]) -> Self {
        let mut request = HttpRequest {
            method: None,
            uri: None,
            http_version: None,
        };

        request.parse(data);
        request
    }
    fn parse(&mut self, data: &[u8]) {
        let lines: Vec<&[u8]> = data
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .collect();

        let request_line = lines[0];
        let words: Vec<&[u8]> = request_line.split(|&b| b == b' ').collect();

        self.method = Some(std::str::from_utf8(words[0]).unwrap_or("").to_string());

        if words.len() > 1 {
            // This is in if because sometimes browsers don't send uri for homepages
            self.uri = Some(std::str::from_utf8(words[1]).unwrap_or("").to_string());
        }

        if words.len() > 2 {
            self.http_version = Some(std::str::from_utf8(words[2]).unwrap_or("").to_string());
        }
    }
}
