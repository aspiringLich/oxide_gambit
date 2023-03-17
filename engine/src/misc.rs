pub fn u8_to_char(u: u8) -> char {
    (match u {
        0..=9 => '0' as u8 + u,
        10..=35 => 'a' as u8 + u - 10,
        36..=61 => 'A' as u8 + u - 36,
        62 => '+' as u8,
        63 => '/' as u8,
        _ => ' ' as u8,
    }) as char
}
