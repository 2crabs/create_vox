use std::io::BufWriter;
use std::fs::File;
use crate::writing::*;
use crate::convert::*;
use std::convert::TryInto;

pub fn write_chunk(name: &str, size: u32, children_size: u32, writer: &mut BufWriter<File>){
    write_string_literal(writer, name);
    write_slice(writer, &i32_to_array(size));
    write_slice(writer, &i32_to_array(children_size));
}

#[derive(Debug)]
pub struct VoxString{
    pub size: i32,
    pub content: String
}

impl VoxString{
    pub fn read(input: &Vec<u8>, cursor: &mut i32) -> VoxString{
        let size = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        let string = String::from_utf8(input[(4 + *cursor as usize)..((4 + size + *cursor) as usize)].to_vec()).unwrap();
        *cursor = *cursor + 4 + size;

        VoxString::new(size, string)
    }

    pub fn new(size: i32, content: String) -> VoxString{
        VoxString{
            size,
            content
        }
    }
}

#[derive(Debug)]
pub struct Dict{
    pub num_of_pairs: i32,
    //(key, value)
    pub pairs: Vec<(VoxString, VoxString)>
}

impl Dict{
    //todo change input to Vec<u8>
    pub fn read(input: &Vec<u8>, cursor: &mut i32) -> Dict{
        let mut pairs = Vec::new();

        let size = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;
        for i in 0..size {
            let key = VoxString::read(input, cursor);
            let value = VoxString::read(input, cursor);
            pairs.push((key, value))
        }

        Dict{
            num_of_pairs: size,
            pairs
        }
    }
}

pub struct Rotation {
    //store a row-major rotation in the bits of a byte
    // bit | value
    // 0-1 : 1 : index of the non-zero entry in the first row
    // 2-3 : 2 : index of the non-zero entry in the second row
    // 4   : 0 : the sign in the first row (0 : positive; 1 : negative)
    // 5   : 1 : the sign in the second row (0 : positive; 1 : negative)
    // 6   : 1 : the sign in the third row (0 : positive; 1 : negative)
    value: u8,
}

//transform node chunk
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct nTRN {
    pub node_id: i32,
    pub node_attributes: Dict,
    pub child_node_id: i32,
    pub reserved_id: i32,
    //must be -1
    pub layer_id: i32,
    //must be 1
    pub num_of_frames: i32,
    // for each frame
    // DICT	: frame attributes
    // (_r : int8) ROTATION, see (c)
    // (_t : int32x3) translation
    // }xN
    pub frame_attributes: Dict
}

impl nTRN{
    pub fn read(input:  &Vec<u8>, cursor: &mut i32) -> nTRN{
        *cursor += 12;
        //need to make function for reading i32
        let node_id = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;
        let node_attributes = Dict::read(input, cursor);
        let child_node_id = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;
        let reserved_id = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;
        let layer_id = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;
        let num_of_frames = i32::from_le_bytes(input[(*cursor as usize)..(4 + *cursor as usize)].try_into().expect("failed to read"));
        *cursor += 4;

        let frame_attributes = Dict::read(input, cursor);

        nTRN{
            node_id,
            node_attributes,
            child_node_id,
            reserved_id,
            layer_id,
            num_of_frames,
            frame_attributes
        }
    }
}

//group node chunk
#[allow(non_camel_case_types)]
pub struct nGRP{
    node_id: i32,
    node_attributes: Dict,
    num_of_children_nodes: i32,
    // for each child
    // {
    // int32	: child node id
    // }xN
    child_id: Vec<i32>
}

//shape node chunk
#[allow(non_camel_case_types)]
pub struct nSHP{
    node_id: i32,
    node_attributes: Dict,
    //must be 1
    num_of_models: i32,
    // for each model
    // {
    // int32	: model id
    // DICT	: model attributes : reserved
    // }xN
    model_id: i32
}

//returns starting index. number 1 should return 1st chunk
pub fn find_chunk(contents: &Vec<u8>, name: String, number: i32) -> Result<usize, ()>{

    //currently breaks if can not find name
    let mut chunk_name = String::new();
    let mut chunk_size: u32;
    let mut current_pos = 8;

    let mut num_chunk = 1;

    while chunk_name != name || num_chunk != (number + 1) {
        //gets name of chunk
        chunk_name = String::from_utf8(
            contents[(current_pos as usize)..((current_pos + 4) as usize)].to_vec(),
        )
            .expect("failed to create string");
        if chunk_name == name{
            if num_chunk == number {
                return Ok(current_pos as usize)
            }
            num_chunk += 1;
        }
        current_pos += 4;
        chunk_size = u32::from_le_bytes(
            contents[(current_pos as usize)..((current_pos + 4) as usize)]
                .try_into()
                .expect("failed to read"),
        );
        current_pos += chunk_size + 8;
    };

    Err(())
}