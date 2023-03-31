use chrono::{DateTime, Datelike, Timelike};

pub fn time(t: &DateTime<chrono::Local>) -> String {
    let (pm, hour) = t.hour12();
    format!(
        "{}:{:02}:{:02} {}",
        hour,
        t.minute(),
        t.second(),
        if pm { "PM" } else { "AM" }
    )
}

pub fn month(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

pub fn date(t: &DateTime<chrono::Local>) -> String {
    format!("{} {}, {}", month(t.month()), t.day(), t.year())
}
