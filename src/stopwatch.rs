use chrono::{Duration, Local};

use crate::common::sleep;
use crate::format::dur;
use crate::print::erase;

pub fn stopwatch() {
    let mut time = Duration::zero();

    let mut start = Local::now();
    loop {
        time = time + (Local::now() - start);
        start = Local::now();

        println!("  {}", dur::accurate(&time));
        sleep(0.05);
        erase();
    }
}
