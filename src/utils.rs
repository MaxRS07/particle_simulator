use vek::*;

pub fn enccol(r: u8, g: u8, b: u8) -> u32 {
    return ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
}
