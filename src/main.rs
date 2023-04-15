mod clock;
mod common;
mod format;
mod parse;
mod print;
mod stopwatch;
mod timer;

use std::env;
use std::process::exit;

fn main() {
    let mut args = env::args();
    let binary = args.next().unwrap_or("rssg".to_string());
    let mut args: Vec<String> = args.collect();

    if args.is_empty() || args[0].as_str() == "help" {
        println!("usage: {binary} [options] <command>");
        vec![
            "commands:",
            "  help             : print this help message",
            "  time             : print the current time",
            "  date             : print the current date",
            "  now              : print the current date + time",
            "  clock            : continuously print the current date + time",
            "  timer <duration> : set a timer for <duration>",
            "                     format: [[[<hours>:]<minutes>:]<seconds>]",
            "                     rings the terminal BEL when the timer stops",
            "  alarm <time>     : set an alarm for <time> in the present day",
            "                     format: [[[<hours>:]<minutes>:]<seconds>] AM|PM",
            "                     rings the terminal BEL when the timer stops",
            "  stopwatch        : start a stopwatch",
            "                     press Ctrl+C to stop, Enter to lap",
        ]
        .iter()
        .for_each(|s| println!("{s}"));
        return;
    }

    match args[0].as_str() {
        "time" => clock::time(),
        "date" => clock::date(),
        "now" => clock::now(),
        "clock" => clock::clock(),

        "timer" => {
            if args.len() != 2 {
                eprintln!("Expected 1 argument to timer, got {}", args.len() - 1);
                exit(1);
            }

            let duration = match parse::dur(&args[1]) {
                Some(t) => t,
                None => {
                    eprintln!("Invalid duration: `{}`", args[1]);
                    exit(1);
                }
            };

            timer::timer(duration);
        }

        "alarm" => {
            if args.len() != 3 {
                eprintln!("Expected 2 arguments to alarm, got {}", args.len() - 1);
                exit(1);
            }

            args = args.into_iter().skip(1).collect();
            let time = match parse::time(&args.join(" ")) {
                Some(t) => t,
                None => {
                    eprintln!("Invalid time: `{}`", args.join(" "));
                    exit(1);
                }
            };

            timer::alarm(time);
        }

        "stopwatch" => stopwatch::stopwatch(),

        s => {
            eprintln!("Unknown command: `{s}`");
            exit(1);
        }
    }
}
