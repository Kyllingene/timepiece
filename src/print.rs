#[inline]
pub fn erase() {
    print!("\x1b[F\x1b[K");
}
