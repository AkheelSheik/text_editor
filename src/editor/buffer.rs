
 pub struct Buffer {
    pub line: Vec<String>,
}

impl Default for Buffer {
    fn default()-> Self {
        Self {
            line: vec!["Hello World!".to_string()]
        }
    }
}