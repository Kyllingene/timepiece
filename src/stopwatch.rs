use std::io::{stdin, Read};

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

pub fn stopwatch_notatty() {
    let mut stdin = stdin().lock();
    let mut buf = String::new();

    let start = Local::now();
    loop {
        if let Ok(0) = stdin.read_to_string(&mut buf) {
            break;
        }

        println!("{buf}");
        buf.clear();
    }

    let now = Local::now();
    println!("\n\n\x07Finished in {} seconds", dur::accurate(&(now - start)));
}
