#[inline]
fn up() {
    print!("{}[A", 27 as char);
}

#[inline]
fn back() {
    print!("\r");
}

#[inline]
pub fn erase() {
    up();
    print!("{}[2K", 27 as char);
    back();
}
