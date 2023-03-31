use chrono::{Local, Duration};

use crate::common::sleep;
use crate::format::dur;
use crate::print::{erase, up};

pub fn stopwatch() {
    let mut time = Duration::zero();
    
    let mut start = Local::now();
    loop {
        time = time + (Local::now() - start);
        start = Local::now();

        up();
        erase();
        println!("  {}", dur::accurate(&time));
        sleep(0.05);
    }
}