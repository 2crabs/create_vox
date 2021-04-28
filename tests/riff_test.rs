use create_vox::riff::{VoxString};
use std::fs::File;
use std::io::Read;

#[test]
fn riff_string(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(&content.to_vec(), &mut 0).content;

    assert_eq!(String::from("hi"), my_string);
}

#[test]
#[should_panic]
fn riff_string_fail(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(&content.to_vec(), &mut 0).content;

    assert_eq!(String::from("HI"), my_string);
}

#[test]
fn chunk_read(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    //start of first chunk
    let mut pos = create_vox::riff::find_chunk(&contents, String::from("nTRN"), 2).unwrap() as i32;
    let chunk = create_vox::riff::nTRN::read(&contents, &mut pos);

    println!("{:?}", chunk);
    println!("\n");
    println!("node id: {}", chunk.node_id);
    println!("node attributes: {:?}", chunk.node_attributes);
    println!("child node id: {}", chunk.child_node_id);
    println!("reserved id: {}", chunk.reserved_id);
    println!("layer id: {}", chunk.layer_id);
    println!("number of frames: {}", chunk.num_of_frames);
    println!("frame attributes: {:?}", chunk.frame_attributes);
    println!("size: {}", chunk.get_size())
}

#[test]
fn ngrp_read(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut cursor = create_vox::riff::find_chunk(&contents, String::from("nGRP"), 1).unwrap() as i32;
    let chunk = create_vox::riff::nGRP::read(&contents, &mut cursor);

    println!("{:?}", chunk);
}

#[test]
fn nshp_read(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut cursor = create_vox::riff::find_chunk(&contents, String::from("nSHP"), 1).unwrap() as i32;
    let chunk = create_vox::riff::nSHP::read(&contents, &mut cursor);

    println!("{:?}", chunk);
    println!("size: {}", chunk.get_size());
}

#[test]
fn matl_read(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut cursor = create_vox::riff::find_chunk(&contents, String::from("MATL"), 1).unwrap() as i32;

    let chunk = create_vox::riff::MATL::read(&contents, &mut cursor);
    println!("{:?}", chunk)
}

#[test]
fn num_chunk(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    println!("transform chunks: {}", create_vox::riff::num_of_chunks(&contents, String::from("nTRN")));
    println!("group chunks: {}", create_vox::riff::num_of_chunks(&contents, String::from("nGRP")));
    println!("shape chunks: {}", create_vox::riff::num_of_chunks(&contents, String::from("nSHP")));
}

#[test]
fn chunk_to_node(){
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    //create_vox::riff::nodes_from_chunks(&contents);
}