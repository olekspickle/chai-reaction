pub fn try_subtract(a: u32, b: u32) -> Option<u32> {
    if a >= b { Some(a - b) } else { None }
}
