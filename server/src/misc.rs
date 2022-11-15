use std::ops::Rem;

use yauc::prelude::*;

// /// write a ColString to a formatter or whatever idk
// pub trait WriteColString {
//     fn write_cstr(&mut self, string: ColString) -> Result<(), std::fmt::Error>;
// }

// impl WriteColString for std::fmt::Formatter<'_> {
//     fn write_cstr(&mut self, string: ColString) -> std::result::Result<(), std::fmt::Error> {
//         self.write_str(&string.to_string())
//     }
// }

/// this isnt actually base 64 its my own bad version
pub fn base64<T>(digit: T) -> char
where
    T: Into<u32>,
{
    let digit = digit.into();
    match digit {
        0..=9 => (digit + '0' as u32) as u8 as char,
        10..=35 => (digit - 10 + 'A' as u32) as u8 as char,
        36..=61 => (digit - 36 + 'a' as u32) as u8 as char,
        62 => '+',
        63 => '/',
        _ => panic!("Invalid digit"),
    }
}

pub fn format_base64<T>(x: T, radix: u32) -> String
where
    T: Into<u32>,
{
    let mut x = x.into();
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;
        result.push(base64(m));
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
