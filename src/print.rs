use std::io::{Stdout, stdout};

use crossterm::cursor::{MoveToColumn, MoveToNextLine};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;

pub struct Printer {
    stdout: Stdout
}

impl Printer {
    pub fn new() -> Self {
        let stdout = stdout();
        enable_raw_mode().unwrap();
        Self {
            stdout
        }
    }

    /// Clear the line and print.
    pub fn erase(&mut self, s: String) {
        execute!(
            self.stdout,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            Print(s)
        ).unwrap();
    }

    /// Clear the line, print, and move down.
    pub fn print(&mut self, s: String) {
        execute!(
            self.stdout,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            Print(s),
            MoveToNextLine(1)
        ).unwrap();
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}