use chrono::{DateTime, Duration, Local};

use crate::common::sleep;
use crate::format::{dur, time};
use crate::print::erase;

pub fn timer(duration: Duration) {
    let start = Local::now();
    let mut time = start;
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        println!("  {}", dur::time(&(time - start - duration)));
        sleep(0.5);

        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        if time - start >= duration {
            break;
        }

        erase();
    }

    println!("\x07Timer for {} complete", dur::time(&duration));
}

pub fn alarm(stop: DateTime<Local>) {
    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        println!("  {}", dur::time(&(stop - time)));
        sleep(0.5);

        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        if time >= stop {
            break;
        }

        erase();
    }

    println!("\x07Alarm for {} complete", time::time(&stop));
}
