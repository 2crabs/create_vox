use crate::riff::{Dict, nTRN, VoxString};
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub enum NodeType{
    Transform(Transform),
    Group,
    //shape with id of model.
    Shape(i32)
}

impl NodeType{
    pub fn write(&self, id: Option<i32>, children: Option<Vec<i32>>, buf_writer: &mut BufWriter<File>){
        
    }
}

#[derive(Debug)]
pub struct Node{
    pub node_type: NodeType,
    pub attributes: NodeAttributes,
    pub child: Vec<Node>
}

impl Node{
    pub fn add_child(&mut self, node: Node){
        self.child.push(node);
    }

    pub fn new(node_type: NodeType) -> Node{
        Node{
            node_type,
            attributes: NodeAttributes::new(),
            child: Vec::new()
        }
    }

    //node id
    pub fn num_node(&self){

    }

    //number of nodes below this node
    pub fn num_children(&self) -> i32{
        let mut num = 0;
        num += self.child.len() as i32;
        for child in self.child.iter(){
            num += child.num_children();
        }

        num
    }

    pub fn write(&self, id: i32, children: Option<Vec<i32>>, buf_writer: &mut BufWriter<File>){
        match &(*self).node_type {
            NodeType::Transform(trans) => {
                nTRN{
                    node_id: id,
                    node_attributes: self.attributes.to_dict(),
                    child_node_id: children.unwrap()[0],
                    reserved_id: 0,
                    layer_id: 0,
                    num_of_frames: 0,
                    frame_attributes: trans.to_dict()
                }.write(buf_writer);

            },

            NodeType::Group => {

            },

            NodeType::Shape(_) => {

            }

        }
    }
    
    //pub fn write_children(){
    //}
    
    pub fn write_all(){
        
    }
}

#[derive(Debug)]
pub struct NodeAttributes{
    pub name: Option<String>,
    pub hidden: Option<bool>
}

impl NodeAttributes{
    pub fn new() -> NodeAttributes{
        NodeAttributes{ name: None, hidden: None }
    }
    
    pub fn to_dict(&self) -> Dict{
        Dict{
            num_of_pairs: self.num_of_pairs(),
            pairs: self.to_pairs()
        }
    }

    pub fn num_of_pairs(&self) -> i32{
        if self.name.is_some() && self.hidden.is_some(){
            2
        } else if self.name.is_some() || self.hidden.is_some()
        {
            1
        } else {
            0
        }
    }

    pub fn to_pairs(&self) -> Vec<(VoxString, VoxString)>{
        let mut pairs: Vec<(VoxString, VoxString)> = Vec::new();

        match self.name.clone() {
            Some(name) => pairs.push((VoxString::new(5, String::from("_name")), VoxString::new(name.len() as i32, name))),
            None => {}
        };

        match self.hidden.clone() {
            Some(value) => pairs.push((VoxString::new(7, String::from("_hidden")), VoxString::new(1, bool_to_string(value)))),
            None => {}
        };

        pairs
    }
}

#[derive(Debug)]
pub struct Transform{
    pub layer: i32,
    //need to make rotation type
    pub rotation: u8,
    pub translation: (i32, i32, i32)
}

impl Transform{
    pub fn to_dict(&self) -> Dict{
        Dict{
            num_of_pairs: 0,
            pairs: vec![]
        }
    }
}

pub fn bool_to_string(value: bool) -> String{
    if value {
        String::from("1")
    } else {
        String::from("0")
    }
}