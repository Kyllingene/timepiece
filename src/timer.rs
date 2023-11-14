use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::common::sleep;
use crate::format::{dur, time};
use crate::print::Printer;

struct TimerState {
    time: Duration,
    duration: Duration,
    increment: Duration,

    cancel: bool,
    paused: bool,

    printer: Printer,
}

fn read_keys(state: Arc<Mutex<TimerState>>) {
    loop {
        let key = read().unwrap();
        let mut state = state.lock().unwrap();
        match key {
            Event::Key(KeyEvent {
                code: KeyCode::Char('p') | KeyCode::Char(' '),
                kind: KeyEventKind::Press,
                ..
            }) => {
                if !state.paused {
                    let left = &(state.time - state.duration);
                    state.printer.erase(format!(" {} PAUSED", dur::time(left)));
                }

                state.paused = !state.paused;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let left = &(state.time - state.duration);
                let dur = dur::time(&state.duration);
                state.printer.print(format!(
                    "\x07Timer for {} cancelled ({} left)",
                    dur,
                    dur::time(left),
                ));
                state.cancel = true;
                return;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right | KeyCode::Char('a'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                state.time = state.time - (state.increment * 5);
                let left = &(state.time - state.duration);
                state.printer.erase(format!(" {}", dur::time(left)));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left | KeyCode::Char('d'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                state.time = state.time + (state.increment * 5);
                let left = &(state.time - state.duration);
                state.printer.erase(format!(" {}", dur::time(left)));
            }
            _ => (),
        }
    }
}

pub fn timer(duration: Duration) {
    print!("Timer started at ");
    crate::clock::time();

    let state = Arc::new(Mutex::new(TimerState {
        time: Duration::zero(),
        duration,
        increment: Duration::seconds(1),

        paused: false,
        cancel: false,

        printer: Printer::new(),
    }));

    let read_state = state.clone();
    std::thread::spawn(move || read_keys(read_state));

    loop {
        sleep(1.0);
        let mut state = state.lock().unwrap();
        if !state.paused {
            if state.cancel {
                break;
            }

            state.time = state.time + state.increment;
            let left = &(state.time - state.duration);
            state.printer.erase(format!(" {}", dur::time(left)));

            if state.time >= state.duration {
                let dur = dur::time(&state.duration);
                state.printer.print(format!("\x07Timer for {} complete", dur));
                state.cancel = true;
                break;
            }
        }
    }
}

pub fn alarm(stop: DateTime<Local>) {
    print!("Alarm started at ");
    crate::clock::time();

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
