use chrono::{DateTime, Datelike, Duration, Local, TimeZone};

pub fn time(t: &str) -> Option<DateTime<Local>> {
    let (hours, t) = t.split_once(':')?;
    let (minutes, t) = t.split_once(':')?;
    let (seconds, pm) = t.split_once(' ')?;

    let pm: u32 = match pm.to_lowercase().as_str() {
        "am" | "a" => 0,
        "pm" | "p" => 12,

        _ => return None,
    };

    let hours: u32 = hours.parse().ok()?;
    let minutes: u32 = minutes.parse().ok()?;
    let seconds: u32 = seconds.parse().ok()?;

    let now = Local::now();
    Local
        .with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            hours + pm,
            minutes,
            seconds,
        )
        .earliest()
}

pub fn dur(t: &str) -> Option<Duration> {
    let increments = t.split(':').collect::<Vec<&str>>();

    let hours;
    let minutes;
    let seconds;
    match increments.len() {
        0 => {
            hours = "0";
            minutes = "0";
            seconds = "0";
        }

        1 => {
            hours = "0";
            minutes = "0";
            seconds = increments[0];
        }

        2 => {
            hours = "0";
            minutes = increments[0];
            seconds = increments[1];
        }

        3 => {
            hours = increments[0];
            minutes = increments[1];
            seconds = increments[2];
        }

        _ => return None,
    }

    let hours: i64 = hours.parse().ok()?;
    let minutes: i64 = minutes.parse().ok()?;
    let seconds: i64 = seconds.parse().ok()?;

    Some(Duration::seconds((hours * 3600) + (minutes * 60) + seconds))
}
