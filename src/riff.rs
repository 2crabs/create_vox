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

//maybe use const generic for this
pub struct VoxString{
    pub size: i32,
    pub content: String
}

impl VoxString{
    pub fn read(input: &[u8]) -> VoxString{
        let size = i32::from_le_bytes(input[0..4].try_into().expect("failed to read"));
        let string = String::from_utf8(input[4..((4 + size) as usize)].to_vec()).unwrap();

        VoxString::new(size, string)

    }

    pub fn new(size: i32, content: String) -> VoxString{
        VoxString{
            size,
            content
        }
    }
}

pub struct Dict{
    num_of_pairs: i32,
    //(key, value)
    pairs: Vec<(VoxString, VoxString)>
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
pub struct nTRN {
    node_id: i32,
    //need to figure out dict for node_attributes
    child_node_id: i32,
    reserved_id: i32,
    //must be -1
    layer_id: i32,
    //must be 1
    num_of_frames: i32,
    // for each frame
    // DICT	: frame attributes
    // (_r : int8) ROTATION, see (c)
    // (_t : int32x3) translation
    // }xN
    frame_attributes: Dict
}

//group node chunk
#[allow(non_camel_case_types)]
pub struct nGRP{
    node_id: i32,
    //need to figure out dict for node_attributes
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
    //need to figure out dict for node_attributes
    //must be 1
    num_of_models: i32,
    // for each model
    // {
    // int32	: model id
    // DICT	: model attributes : reserved
    // }xN
    model_id: i32
}