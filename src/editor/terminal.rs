use std::io::{Write, stdout};

use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use std::io::Error;

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

/// Repreents the Terminal
/// Terminal spans max rows/columns of usize::MAX or u16::MAX, whichever is smaller
/// (crossterm uses u16 so we're kinda limited here)
/// If you try to set the caret out of these bounds, it will be truncated
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Moves the caret to the given position
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        queue!(
            stdout(),
            MoveTo(
                // #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
                position.col as u16,
                position.row as u16
            )
        )?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {
            height: height as usize,
            width: width as usize,
        })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
