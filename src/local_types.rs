/**
 * to_u32
 * Credit to https://www.reddit.com/r/rust/comments/36ixl0/converting_a_vector_of_bits_to_an_integer/crehkpw/
 */
pub fn to_u32(slice: &Vec<u8>) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

pub fn to_u32le(slice: &Vec<u8>) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc.rotate_left(8) + b as u32)
}

pub fn to_u16(slice: &Vec<u8>) -> u16 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u16)
}

pub fn to_u16le(slice: &Vec<u8>) -> u16 {
    slice.iter().rev().fold(0, |acc, &b| acc.rotate_left(8) + b as u16)
}
