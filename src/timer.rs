use chrono::{DateTime, Duration, Local};

use crate::common::sleep;
use crate::format::{self, dur};
use crate::print::erase;

pub fn timer(duration: Duration) {
    let start = Local::now();

    loop {
        let now = Local::now();
        if now - start >= duration {
            break;
        }

        erase();
        println!("  {}", dur::time(&(now - start - duration)));
        sleep(0.5);
    }

    erase();
    println!("\x07Timer complete");
}

pub fn alarm(time: DateTime<Local>) {
    loop {
        let now = Local::now();
        if now >= time {
            break;
        }

        erase();
        println!("  {}", dur::time(&(time - now)));
        sleep(0.5);
    }

    erase();
    println!("\x07Alarm for {} complete", format::time::time(&time));
}
