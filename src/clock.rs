use chrono::{Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::common::sleep;
use crate::format::time;
use crate::print::Printer;

pub fn time() {
    let now = Local::now();
    println!("{}", time::time(&now));
}

pub fn date() {
    let now = Local::now();
    println!("{}", time::date(&now));
}

pub fn now() {
    let now = Local::now();
    println!("{} {}", time::date(&now), time::time(&now));
}

pub fn clock() {
    let mut printer = Printer::new();

    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let mut lap: u32 = 0;

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        if poll(std::time::Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' ') | KeyCode::Char('l'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    printer.print(format!(
                        " == LAP {lap}: {} {}",
                        time::date(&time),
                        time::time(&time)
                    ));
                    lap += 1;
                }
                _ => {}
            }
        }

        printer.erase(format!("{} {}", time::date(&time), time::time(&time)));
        sleep(1.0);
        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }
    }
}
