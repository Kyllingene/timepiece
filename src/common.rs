use std::sync::mpsc::Receiver;

pub fn sleep(secs: f32) {
    std::thread::sleep(std::time::Duration::from_secs_f32(secs));
}

pub fn watch_keys() -> (std::thread::JoinHandle<()>, Receiver<console::Key>) {
    let (tx, rx) = std::sync::mpsc::channel();
    (std::thread::spawn(move || {
        let term = console::Term::stdout();
        while let Ok(key) = term.read_key() {
            if matches!(key, console::Key::Escape | console::Key::Char('q')) {
                tx.send(key).unwrap();
                break;
            } else {
                tx.send(key).unwrap();
            }
        }
    }), rx)
}
