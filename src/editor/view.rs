use crate::editor::terminal::Terminal;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} {VERSION}");
        let terminal_width = Terminal::size()?.width;
        let msg_length = welcome_message.len();
        let pad_length = (terminal_width - msg_length) / 2;

        let spaces_to_print = " ".repeat(pad_length - 1);
        welcome_message = format!("~{spaces_to_print}{NAME} {VERSION}");
        welcome_message.truncate(terminal_width);

        Terminal::print(welcome_message.as_str())?;
        Ok(())
    }

    pub fn render() -> Result<(), Error> {
        let height = Terminal::size()?.height;
        for row in 0..height {
            Terminal::clear_line()?;
            if row == 0 {
                Terminal::print("~ Hello Atto")?;
            } else if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
