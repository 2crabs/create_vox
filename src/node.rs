use crate::riff::{Dict, nTRN};

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct NodeAttributes{
    pub name: Option<String>,
    pub hidden: Option<String>
}

impl NodeAttributes{
    pub fn new() -> NodeAttributes{
        NodeAttributes{ name: None, hidden: None }
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
    fn from_chunk(chunk: nTRN){

    }
}