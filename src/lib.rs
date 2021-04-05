mod voxobject;
mod writing;
mod color;
mod voxel;

use std::io::Write;
use writing::*;
pub use color::*;
pub use voxel::*;
pub use voxobject::*;


fn i32_to_array(a: u32) -> [u8;4]{
    a.to_ne_bytes()
}

fn u16_to_array(a: u16) -> [u8;2]{
    a.to_ne_bytes()
}

fn get_middle(a: u8, b: u8, point_between: f32) -> u8{
    ((((b as i16)-(a as i16)) as f32 * point_between)+ a as f32) as u8
}

#[cfg(test)]
mod tests {
}
