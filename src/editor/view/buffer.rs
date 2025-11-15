use std::fs::read_to_string;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Might wanna call this Buffer::from as this makes more idiomatic sense
    pub fn load(path: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(path)?;

        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line))
        }
        Ok(Self { lines })
    }
}
