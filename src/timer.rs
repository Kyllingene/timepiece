use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Local};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use notify_rust::Notification;

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
                    "\x07Timer for {dur} cancelled (time left: {})",
                    dur::time(left),
                ));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Timer cancelled")
                        .body(&format!(
                            "Timer for {dur} cancelled\nTime left: {}",
                            dur::time(left)
                        ))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

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
                state
                    .printer
                    .print(format!("\x07Timer for {} complete", dur));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Timer complete")
                        .body(&format!(
                            "Timer for {dur} complete\nFinished at {}",
                            time::time(&Local::now())
                        ))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

                state.cancel = true;
                break;
            }
        }
    }

    print!("Timer finished at ");
    crate::clock::time();
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
                let left = dur::time(&(stop - time));
                let time = time::time(&stop);
                printer.print(format!(
                    "\x07Alarm for {time} cancelled (time left: {left})",
                ));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Alarm cancelled")
                        .body(&format!("Alarm for {time} cancelled\nTime left: {left}"))
                        .show()
                    {
                        eprintln!("Failed to send notification: {e}");
                    }
                }

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
                let time = time::time(&stop);
                printer.print(format!("\x07Alarm for {time} complete"));

                #[cfg(feature = "notify")]
                {
                    if let Err(e) = Notification::new()
                        .summary("Alarm complete")
                        .body(&format!("Alarm for {time} complete"))
                        .show()
                    {
                        eprintln!("Failed to show notification: {e}");
                    }
                }

                break;
            }
        }
    }
}
