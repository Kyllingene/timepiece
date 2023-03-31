use chrono::{Local, Duration};

use crate::common::sleep;
use crate::format;
use crate::print::erase;

pub fn time() {
    let now = Local::now();
    println!("{}", format::time::time(&now));
}

pub fn date() {
    let now = Local::now();
    println!("{}", format::time::date(&now));
}

pub fn now() {
    let now = Local::now();
    println!("{} {}", format::time::date(&now), format::time::time(&now));
}

pub fn clock() {
    let mut time = Local::now();
    let mut elapsed = Duration::zero();

    let second = Duration::seconds(1);
    let minute = Duration::minutes(1);
    loop {
        println!("{} {}", format::time::date(&time), format::time::time(&time));
        sleep(1.0);
        time += second;
        elapsed = elapsed + second;

        if elapsed >= minute {
            elapsed = Duration::zero();
            time = Local::now();
        }

        erase();
    }
}
