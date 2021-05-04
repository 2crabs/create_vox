use crate::riff::{LAYR, Dict, VoxString};
use std::io::BufWriter;
use std::fs::File;
use crate::node::bool_to_string;

pub struct Layer{
    id: i32,
    name: Option<String>,
    hidden: Option<bool>,
}

impl  Layer {
    pub fn from_chunk(chunk: LAYR) -> Layer{
        Layer{
            id: chunk.layer_id,
            name: Layer::has_name(&chunk),
            hidden: Layer::is_hidden(&chunk)
        }
    }

    pub fn is_hidden(chunk: &LAYR) -> Option<bool>{
        if !chunk.layer_attributes.pairs.is_empty() {
            for pair in chunk.layer_attributes.pairs.iter(){
                if pair.0.content == *"_hidden" {
                    println!("here");
                    return Some(pair.1.content.parse::<i32>().expect("failed to parse value") == 1)
                }
            }
        }

        None
    }

    pub fn has_name(chunk: &LAYR) -> Option<String>{
        if !chunk.layer_attributes.pairs.is_empty() {
            for pair in chunk.layer_attributes.pairs.iter(){
                if pair.0.content == *"_name" {
                    return Some(pair.1.content.clone());
                }
            }
        }

        None
    }

    pub fn to_chunk(&self) -> LAYR{
        LAYR {
            layer_id: self.id,
            layer_attributes: self.to_dict(),
            reserved_id: -1
        }
    }

    pub fn to_dict(&self) -> Dict{
        let num_pairs: i32;
        if self.hidden.is_some() && self.name.is_some() {
            num_pairs = 2;
        } else if self.hidden.is_some() || self.name.is_some() {
            num_pairs = 1;
        } else {
            num_pairs = 0;
        }

        let mut pairs: Vec<(VoxString, VoxString)> = Vec::new();

        if self.name.is_some() {
            pairs.push((VoxString::new(5, String::from("_name")), VoxString::new(self.name.as_ref().unwrap().len() as i32, self.name.as_ref().unwrap().clone())));
        }

        if self.hidden.is_some(){
            pairs.push((VoxString::new(7, String::from("_hidden")), VoxString::new(1, bool_to_string(self.hidden.unwrap()))));
        }
        Dict{
            num_of_pairs: num_pairs,
            pairs
        }
    }

    pub fn write(&self,  buf_writer: &mut BufWriter<File>){
        self.to_chunk().write(buf_writer);
    }

    pub fn get_size(&self) -> i32{
        self.to_chunk().get_size()
    }
}