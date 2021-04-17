use std::io::BufWriter;
use std::fs::File;
use crate::writing::*;
use crate::convert::*;

pub fn write_chunk(name: &str, size: u32, children_size: u32, writer: &mut BufWriter<File>){
    write_string_literal(writer, name);
    write_slice(writer, &i32_to_array(size));
    write_slice(writer, &i32_to_array(children_size));
}