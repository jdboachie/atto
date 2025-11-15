mod buffer;

use buffer::Buffer;
use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    size: Size,

    should_rerender: bool,
}

impl Default for View {
    fn default() -> Self {
        Self {
            should_rerender: true,
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        // might need to subscribe to resize events rather than juggling this manually
        self.should_rerender = true;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} {VERSION}");
        let msg_length = welcome_message.len();

        if width <= msg_length {
            return "~".to_string();
        }
        let pad_length = width.saturating_sub(msg_length).saturating_sub(1) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(pad_length.saturating_sub(1)), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn render(&mut self) {
        if !self.should_rerender {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                // let truncated_line = line.get(0..width).unwrap_or(line);
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line);
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }

        self.should_rerender = false;
    }

    pub fn load(&mut self, path: &String) {
        if let Ok(buffer) = Buffer::load(path) {
            self.buffer = buffer;
            self.should_rerender = true;
        }
    }
}
