pub const GREEN_BLOCK: &str = "\x1b[32m■\x1b[0m";
pub const GREEN_EMPTY_BLOCK: &str = "\x1b[32m☐\x1b[0m";

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

