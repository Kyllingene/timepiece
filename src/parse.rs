use chrono::{Duration, Local, DateTime, Datelike, TimeZone};

pub fn time(t: &str) -> Option<DateTime<Local>> {
    let (hours, t) = t.split_once(':')?;
    let (minutes, t) = t.split_once(':')?;
    let (seconds, pm) = t.split_once(' ')?;

    let pm: u32 = match pm.to_lowercase().as_str() {
        "am" | "a" => 0,
        "pm" | "p" => 12,

        _ => return None
    };

    let hours: u32 = hours.parse().ok()?;
    let minutes: u32 = minutes.parse().ok()?;
    let seconds: u32 = seconds.parse().ok()?;

    let now = Local::now();
    Some(Local.with_ymd_and_hms(
        now.year(),
        now.month(),
        now.day(),
        hours + pm,
        minutes,
        seconds
    ).earliest()?)
}

pub fn dur(t: &str) -> Option<Duration> {
    let increments = t.split(":").collect::<Vec<&str>>();

    if increments.len() > 3 {
        return None;
    }

    let hours = increments.get(0).unwrap_or(&"0");
    let minutes = increments.get(1).unwrap_or(&"0");
    let seconds = increments.get(2).unwrap_or(&"0");

    let hours: i64 = hours.parse().ok()?;
    let minutes: i64 = minutes.parse().ok()?;
    let seconds: i64 = seconds.parse().ok()?;

    Some(Duration::seconds((hours * 3600) + (minutes * 60) + seconds))
}