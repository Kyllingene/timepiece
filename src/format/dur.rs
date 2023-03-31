use chrono::Duration;

pub fn time(t: &Duration) -> String {
    format!(
        "{}:{:02}:{:02}",
        t.num_hours().abs(),
        t.num_minutes().abs() - t.num_hours().abs() * 60,
        t.num_seconds().abs() - t.num_minutes().abs() * 60
    )
}

pub fn accurate(t: &Duration) -> String {
    format!(
        "{}:{:03}",
        time(t),
        t.num_milliseconds() - (t.num_seconds() * 1000)
    )
}
