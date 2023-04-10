use chrono::{Duration, Local};
use console::Key;

use crate::common::watch_keys;
use crate::format::dur;
use crate::print::erase;

pub fn stopwatch() {
    let (handle, keys) = watch_keys();
    
    let mut time = Duration::zero();
    let mut start = Local::now();
    
    let mut lap: u32 = 0;
    loop {
        time = time + (Local::now() - start);
        start = Local::now();
        if let Ok(key) = keys.try_recv() {
            match key {
                Key::Escape | Key::Char('q') => {
                    println!(" {}", dur::accurate(&time));
                    break;
                }
                Key::Enter | Key::Char(' ') | Key::Char('l') => {
                    println!(" == LAP {lap}: {}", dur::accurate(&time));
                    lap += 1;
                }
                _ => {}
            }
        }

        println!(" {}", dur::accurate(&time));
        erase();
    }

    handle.join().unwrap();
}
