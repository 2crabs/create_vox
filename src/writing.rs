use std::io::Write;

pub fn write_string_literal(inputfile: &mut std::io::BufWriter<std::fs::File>, string: &str){
    inputfile.write(string.as_bytes()).expect("failed");
}

pub fn write_slice(inputfile: &mut std::io::BufWriter<std::fs::File>, slice: &[u8]){
    inputfile.write(slice).expect("failed");
}
