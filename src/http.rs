pub struct HttpResponse {
    pub status: HttpStatus,
    pub length: usize,
    pub content: String,
}

impl HttpResponse {
    pub fn new(status: HttpStatus, content: String) -> HttpResponse {
        HttpResponse {
            status,
            length: content.len(),
            content,
        }
    }

    pub fn format(self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.code, self.status.text, self.length, self.content
        )
    }
}

pub struct HttpStatus {
    pub code: u16,
    pub text: String,
}

impl HttpStatus {
    pub fn new(code: u16, text: String) -> HttpStatus {
        HttpStatus { code, text }
    }
}
