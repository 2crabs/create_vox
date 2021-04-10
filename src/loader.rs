use std::fs::*;
use std::io::Read;
use std::convert::TryInto;
use crate::{Voxobject, Voxel};
use std::ops::Add;

pub fn load_voxobject(file_path: &str) -> Voxobject{
    let mut file = File::open(file_path).expect("failed to open file");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents);
    //let contents = *contents.as_slice();
    /*
    let VOX_chunk: [u8; 4] = contents[0..4].try_into().expect("failed to read");
    let MAIN_chunk: [u8; 4] = contents[8..12].try_into().expect("failed to read");
    let MAIN_children: [u8; 4] = contents[16..20].try_into().expect("failed to read");//26719 in loaded.vox
    let SIZE_chunk: [u8; 4] = contents[20..24].try_into().expect("failed to read");

    let SIZE_size: u32 = u32::from_le_bytes(contents[24..28].try_into().expect("failed to read"));
    */
    let SIZE_x: u16 = u16::from_le_bytes(contents[32..34].try_into().expect("failed to read"));
    let SIZE_y: u16 = u16::from_le_bytes(contents[36..38].try_into().expect("failed to read"));
    let SIZE_z: u16 = u16::from_le_bytes(contents[40..42].try_into().expect("failed to read"));
    let XYZI_chunk: [u8; 4] = contents[44..48].try_into().expect("failed to read");
    let XYZI_size: u32 = u32::from_le_bytes(contents[48..52].try_into().expect("failed to read"));
    let num_voxels = u32::from_le_bytes(contents[56..60].try_into().expect("failed to read"));
    /*
    assert_eq!(String::from("VOX ").as_bytes(), VOX_chunk);
    assert_eq!(String::from("MAIN").as_bytes(), MAIN_chunk);
    assert_eq!(String::from("SIZE").as_bytes(), SIZE_chunk);
    assert_eq!(12, SIZE_size);
    assert_eq!(10, SIZE_x);
    assert_eq!(15, SIZE_y);
    assert_eq!(19, SIZE_z);
    assert_eq!(String::from("XYZI").as_bytes(), XYZI_chunk);
    assert_eq!(56, XYZI_size);
    assert_eq!(num_voxels, 13);
    */

    let size = (SIZE_x,SIZE_y,SIZE_z);
    //the voxobject that is returned
    let mut voxobject = Voxobject::new(size.0,size.1,size.2);


    for i in 0..num_voxels{
        let voxel_info = contents[((60+(i*4)) as usize)..((64+(i*4)) as usize)].try_into().expect("failed to read");
        voxobject.add_voxel(array_to_voxel(voxel_info));
    };

    voxobject
}

fn array_to_voxel(array: [u8; 4]) -> Voxel{
    Voxel::new(array[0],array[1],array[2],array[3] + 1)
}