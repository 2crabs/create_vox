use create_vox::riff::{VoxString, find_chunk, nTRN};
use std::fs::File;
use std::io::Read;

#[test]
fn riff_string(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(content).content;

    assert_eq!(String::from("hi"), my_string);
}

#[test]
#[should_panic]
fn riff_string_fail(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(content).content;

    assert_eq!(String::from("HI"), my_string);
}

#[test]
fn chunk_read(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let a = find_chunk(contents, String::from("nTRN")).unwrap();
    println!("result was: {}", a);
}