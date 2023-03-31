#[inline]
pub fn up() {
    print!("{}[A", 27 as char);
}

#[inline]
pub fn back() {
    print!("\r");
}

#[inline]
pub fn erase() {
    print!("{}[2K", 27 as char);
    back();
}
