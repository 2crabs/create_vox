use std::fs::*;
use std::io::Read;
use std::convert::TryInto;

pub fn load_voxobject(file_path: &str){
    let mut file = File::open(file_path).expect("failed to open file");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents);
    //let contents = *contents.as_slice();

    let MAIN_chunk: [u8; 4] = contents[0..4].try_into().expect("failed to read");

    assert_eq!(String::from("VOX ").as_bytes(), MAIN_chunk);

    let my_array: [u8; 4] = [0,4,0,0];

    let num = u32::from_le_bytes(my_array);

    assert_eq!(1024, num)
}