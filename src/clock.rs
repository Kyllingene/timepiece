use chrono::{Duration, Local};
use console::Key;

use crate::common::{sleep, watch_keys};
use crate::format::time;
use crate::print::erase;

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
    let (handle, keys) = watch_keys();

    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let mut lap: u32 = 0;

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        if let Ok(key) = keys.try_recv() {
            match key {
                Key::Escape | Key::Char('q') => {
                    println!(
                        "{} {}",
                        time::date(&time),
                        time::time(&time)
                    );
                    break;
                }
                Key::Enter | Key::Char(' ') | Key::Char('l') => {
                    println!(
                        " == LAP {lap}: {} {}",
                        time::date(&time),
                        time::time(&time)
                    );
                    lap += 1;
                }
                _ => {}
            }
        }

        println!(
            "{} {}",
            time::date(&time),
            time::time(&time)
        );
        sleep(1.0);
        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        erase();
    }

    handle.join().unwrap();
}
