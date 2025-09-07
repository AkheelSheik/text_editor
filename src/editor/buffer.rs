
 pub struct Buffer {
    pub line: Vec<String>,
}

impl Default for Buffer {
    fn default()-> Self {
        Self {
            line: Vec::new(),
        }
    }
}

impl Buffer {
    pub fn is_empty(&self) ->bool {
    if let Some(element) = self.line.get(1) {
        return true;
    } else {
        return false;
    } 
    }
}