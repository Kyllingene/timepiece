use chrono::{DateTime, Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::common::sleep;
use crate::format::{dur, time};
use crate::print::Printer;

pub fn timer(duration: Duration) {
    let mut printer = Printer::new();

    let mut start = Local::now();
    let mut time = Duration::zero();
    let mut elapsed = Duration::zero();

    let mut paused = false;

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        if poll(std::time::Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p') | KeyCode::Char(' '),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if !paused {
                        printer.erase(format!(" {} PAUSED", dur::time(&(time - duration))));
                    }

                    paused = !paused;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    printer.print(format!(
                        "\x07Timer for {} cancelled ({} left)",
                        dur::time(&duration),
                        dur::time(&(time - duration))
                    ));
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right | KeyCode::Char('a'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    time = time - (second * 10);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left | KeyCode::Char('d'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    time = time + (second * 10);
                }
                _ => (),
            }
        }

        if paused {
            start += second;
        } else {
            elapsed = elapsed + second;

            if elapsed >= minute {
                elapsed = Duration::zero();
                time = Local::now() - start;
            }
    
            sleep(1.0);

            time = time + second;
            printer.erase(format!(" {}", dur::time(&(time - duration))));

            if time >= duration {
                printer.print(format!("\x07Timer for {} complete", dur::time(&duration)));
                break;
            }
        }
    }
}

pub fn alarm(stop: DateTime<Local>) {
    let mut printer = Printer::new();

    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        if poll(std::time::Duration::ZERO).unwrap() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) = read().unwrap()
            {
                printer.print(format!(
                    "\x07Alarm for {} cancelled (time left: {})",
                    time::time(&stop),
                    dur::time(&(stop - time))
                ));
                break;
            }
        }

        printer.erase(format!(" {}", dur::time(&(stop - time))));
        sleep(1.0);

        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        if time >= stop {
            elapsed = Duration::zero();
            time = Local::now();

            if time >= stop {
                printer.print(format!("\x07Alarm for {} complete", time::time(&stop)));
                break;
            }
        }
    }
}
