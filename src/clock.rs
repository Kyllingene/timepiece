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
    let second = Duration::seconds(1);
    loop {
        erase();
        println!("{} {}", format::time::date(&time), format::time::time(&time));
        sleep(1.0);
        time += second;
    }
}
