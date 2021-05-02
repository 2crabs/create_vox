use crate::{Color, Voxel, Voxobject};
use std::convert::TryInto;
use std::fs::*;
use std::io::Read;

#[deprecated]
pub(crate) fn load_voxobject(file: &mut File) -> Voxobject {
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let size_x: u16 = u16::from_le_bytes(contents[32..34].try_into().expect("failed to read"));
    let size_y: u16 = u16::from_le_bytes(contents[36..38].try_into().expect("failed to read"));
    let size_z: u16 = u16::from_le_bytes(contents[40..42].try_into().expect("failed to read"));

    let num_voxels = u32::from_le_bytes(contents[56..60].try_into().expect("failed to read"));

    //finds the start of the palette
    let end_of_voxels = 60 + (num_voxels * 4);
    let mut chunk_name = String::new();
    let mut chunk_size: u32;
    let mut current_pos = end_of_voxels;

    while chunk_name != String::from("RGBA") {
        //gets name of chunk
        chunk_name = String::from_utf8(
            contents[(current_pos as usize)..((current_pos + 4) as usize)].to_vec(),
        )
        .expect("failed to create string");
        if chunk_name == String::from("RGBA") {
            break;
        }
        current_pos += 4;
        chunk_size = u32::from_le_bytes(
            contents[(current_pos as usize)..((current_pos + 4) as usize)]
                .try_into()
                .expect("failed to read"),
        );
        current_pos += chunk_size + 8;
    }
    current_pos += 12;
    //current pos should now at the start of the contents of RGBA
    let mut palette: [Color; 256] = [Color::new(0, 0, 0, 0); 256];

    for i in 0..256 {
        //gets the color data
        let r: u8 = contents[(current_pos + (i * 4)) as usize];
        let g: u8 = contents[(current_pos + (i * 4) + 1) as usize];
        let b: u8 = contents[(current_pos + (i * 4) + 2) as usize];
        let a: u8 = contents[(current_pos + (i * 4) + 3) as usize];
        palette[i as usize] = Color::new(r, g, b, a);
    }
    let size = (size_x, size_y, size_z);

    //the voxobject that is returned
    let mut voxobject = Voxobject::new(size.0, size.1, size.2);
    voxobject.palette = palette;
    for i in 0..num_voxels {
        let voxel_info = contents[((60 + (i * 4)) as usize)..((64 + (i * 4)) as usize)]
            .try_into()
            .expect("failed to read");
        voxobject.add_voxel(array_to_voxel(voxel_info)).unwrap();
    }

    voxobject
}

fn array_to_voxel(array: [u8; 4]) -> Voxel {
    Voxel::new(array[0], array[1], array[2], array[3])
}
