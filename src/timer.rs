use chrono::{Local, DateTime, Duration};

use crate::common::sleep;
use crate::format::{dur, self};
use crate::print::{erase, up};

pub fn timer(duration: Duration) {
    let start = Local::now();

    loop {
        let now = Local::now();
        if now - start >= duration {
            break;
        }

        up();
        erase();
        println!("  {}", dur::time(&(now - start - duration)));
        sleep(0.5);
    }

    up();
    erase();
    println!("\x07Timer complete");
}

pub fn alarm(time: DateTime<Local>) {
    loop {
        let now = Local::now();
        if now >= time {
            break;
        }

        up();
        erase();
        println!("  {}", dur::time(&(time - now)));
        sleep(0.5);
    }

    up();
    erase();
    println!("\x07Alarm for {} complete", format::time::time(&time));
}