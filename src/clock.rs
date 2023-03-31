use chrono::Local;

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
    loop {
        erase();
        now();
        sleep(0.5);
    }
}
