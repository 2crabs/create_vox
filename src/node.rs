use crate::riff::{Dict, nTRN};

pub enum NodeType{
    Transform(Transform),
    Group,
    //shape with id of model.
    Shape(i32)
}

//need to convert the chunks into a tree
//add first chunk
//get child id and find it from other chunks
//add the chunk under the child property after converting it
pub struct Node{
    pub node_type: NodeType,
    //need to change this. make a struct for attributes
    pub attributes: NodeAttributes,
    pub child: Option<Vec<NodeType>>
}

impl Node{
    pub fn add_child(&mut self, node_type: NodeType){
        match self.child {
            Some(_) => {self.child.as_mut().unwrap().push(node_type)},
            None => {
                self.child = Some(vec![node_type])
            }
        }
    }
}

pub struct NodeAttributes{
    pub name: Option<String>,
    pub hidden: Option<String>
}

pub struct Transform{
    pub layer: i32,
    //need to make rotation type
    pub rotation: u8,
    pub translation: (i32, i32, i32)
}

impl Transform{
    fn from_chunk(chunk: nTRN){

    }
}