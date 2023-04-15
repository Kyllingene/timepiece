use chrono::{Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::format::dur;
use crate::print::Printer;

pub fn stopwatch() {
    let mut printer = Printer::new();
    
    let mut time = Duration::zero();
    let mut start = Local::now();
    
    let mut lap: u32 = 0;
    loop {
        time = time + (Local::now() - start);
        start = Local::now();
        if poll(std::time::Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    printer.print(format!(" {}", dur::accurate(&time)));
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' ') | KeyCode::Char('l'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    printer.print(format!(" == LAP {lap}: {}", dur::accurate(&time)));
                    lap += 1;
                }
                _ => {}
            }
        }

        printer.erase(format!(" {}", dur::accurate(&time)));
    }

    println!();
}
