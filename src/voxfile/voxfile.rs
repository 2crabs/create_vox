use crate::model::Model;
use crate::Color;
use crate::node::Node;
use crate::riff;
use std::fs::File;
use std::io::{Read, BufWriter};
use crate::writing::{write_slice, write_string_literal};
use crate::riff::{write_chunk, LAYR, num_of_chunks};

pub struct VoxFile{
    pub models: Vec<Model>,
    pub palette: [Color; 256],
    pub root_node: Node,
    pub layers: Vec<LAYR>
}

impl VoxFile{
    //size in bytes when written
    pub fn get_size(&self) -> i32{
        let mut size = 1024;
        for model in self.models.iter(){
            size += model.get_size();
        }
        for layer in self.layers.iter(){
            size += layer.get_size()
        }
        size += self.root_node.get_all_size();
        size
    }
}