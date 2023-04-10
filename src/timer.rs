use chrono::{DateTime, Duration, Local};
use console::Key;

use crate::common::{sleep, watch_keys};
use crate::format::{dur, time};
use crate::print::{erase, back};

pub fn timer(duration: Duration) {
    let (handle, keys) = watch_keys();

    let mut start = Local::now();
    let mut time = start;
    let mut elapsed = Duration::zero();

    let mut paused = false;

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);

    back();
    loop {
        if let Ok(key) = keys.try_recv() {
            match key {
                Key::Escape | Key::Char('q') => {
                    println!(
                        "\x07Timer for {} cancelled (time left: {})",
                        dur::time(&duration),
                        dur::time(&(time - start - duration))
                    );
                    handle.join().unwrap();
                    return;
                }

                Key::Enter | Key::Char(' ') | Key::Char('p') => {
                    if paused {
                        erase();
                    } else {
                        println!(" {} PAUSED", dur::time(&(time - start - duration)));
                    }

                    paused = !paused;
                }
                _ => {}
            }
        }

        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        sleep(1.0);

        if paused {
            start += second;
        } else {
            println!(" {}", dur::time(&(time - start - duration)));

            if time - start >= duration {
                break;
            }

            erase();
        }
    }

    println!("\x07Timer for {} complete", dur::time(&duration));
}

pub fn alarm(stop: DateTime<Local>) {
    let (handle, keys) = watch_keys();

    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);

    back();
    loop {
        if let Ok(key) = keys.try_recv() {
            match key {
                Key::Escape | Key::Char('q') => {
                    println!(
                        "\x07Alarm for {} cancelled (time left: {})",
                        time::time(&stop),
                        dur::time(&(stop - time))
                    );
                    handle.join().unwrap();
                    return;
                }
                _ => {}
            }
        }

        println!(" {}", dur::time(&(stop - time)));
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
                break;
            }
        }

        erase();
    }

    println!("\x07Alarm for {} complete", time::time(&stop));
}
