pub struct Options {
    content: Vec<String>,
}

impl Options {
    pub fn new() -> Options {
        return Options {content: Vec::new()}
    }

    pub fn push(&mut self, content: String) {
        self.content.push(content);
    }

    pub fn display(self) -> String {
        return self.content.join("\n");
    }
}