use crate::riff::{nGRP, nSHP, nTRN, Dict, VoxString};
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Transform(Transform),
    Group,
    //shape with id of model.
    Shape(i32),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub id: i32,
    pub children: Vec<i32>,
    pub attributes: NodeAttributes,
    pub child: Vec<Node>,
}

impl Node {
    pub fn add_child(&mut self, node: Node) {
        self.child.push(node);
    }

    pub fn new(node_type: NodeType, attribs: NodeAttributes) -> Node {
        Node {
            node_type,
            id: 0,
            children: vec![],
            attributes: attribs,
            child: Vec::new(),
        }
    }

    //node id
    pub fn num_node(&self) {}

    //number of nodes below this node
    pub fn num_children(&self) -> i32 {
        let mut num = 0;
        num += self.child.len() as i32;
        for child in self.child.iter() {
            num += child.num_children();
        }

        num
    }
    pub fn write(&self, id: &mut i32, children: Vec<i32>, buf_writer: &mut BufWriter<File>) {
        match &(*self).node_type {
            NodeType::Transform(trans) => {
                nTRN {
                    node_id: *id,
                    node_attributes: self.attributes.to_dict(),
                    child_node_id: children[0],
                    reserved_id: 0,
                    layer_id: -1,
                    num_of_frames: 1,
                    frame_attributes: trans.to_dict(),
                }
                .write(buf_writer);
            }

            NodeType::Group => nGRP {
                node_id: *id,
                node_attributes: self.attributes.to_dict(),
                num_of_children_nodes: self.child.len() as i32,
                child_id: children,
            }
            .write(buf_writer),

            NodeType::Shape(model_id) => nSHP {
                node_id: *id,
                node_attributes: self.attributes.to_dict(),
                num_of_models: 1,
                model_id: *model_id,
                model_attributes: Dict {
                    num_of_pairs: 0,
                    pairs: vec![],
                },
            }
            .write(buf_writer),
        }
    }

    pub fn get_child_ids(&self, id: &mut i32) -> Vec<i32> {
        let mut child_ids = Vec::new();
        let mut new_id = (*id).clone();
        for _i in self.child.iter() {
            new_id += 1;
            child_ids.push(new_id);
        }

        child_ids
    }
    pub fn write_children(&self, buf_writer: &mut BufWriter<File>, id: &mut i32) {
        for child in self.child.iter() {
            child.write(&mut (*id).clone(), child.get_child_ids(id), buf_writer);
            *id += 1;
            child.write_children(buf_writer, id);
        }
    }

    pub fn write_all(&self, buf_writer: &mut BufWriter<File>) {
        let mut id = 1;
        self.write(&mut id.clone(), self.get_child_ids(&mut id), buf_writer);
        id += 1;
        self.write_children(buf_writer, &mut id)
    }


    pub fn get_size(&self) -> i32{
        match &(*self).node_type {
            NodeType::Transform(trans) => {
                nTRN {
                    node_id: 0,
                    node_attributes: self.attributes.to_dict(),
                    child_node_id: 0,
                    reserved_id: 0,
                    layer_id: -1,
                    num_of_frames: 1,
                    frame_attributes: trans.to_dict(),
                }
                    .get_size()
            }

            NodeType::Group => nGRP {
                node_id: 0,
                node_attributes: self.attributes.to_dict(),
                num_of_children_nodes: self.child.len() as i32,
                child_id: vec![0; self.child.len()],
            }
                .get_size(),

            NodeType::Shape(model_id) => nSHP {
                node_id: 0,
                node_attributes: self.attributes.to_dict(),
                num_of_models: 1,
                model_id: *model_id,
                model_attributes: Dict {
                    num_of_pairs: 0,
                    pairs: vec![],
                },
            }.get_size()
        }
    }

    pub fn get_children_size(&self, size: &mut i32){
        for child in self.child.iter(){
            *size += child.get_size();
            child.get_children_size(size);
        };
    }

    pub fn get_all_size(&self) -> i32{
        let mut size = self.get_size();
        self.get_children_size(&mut size);
        size
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeAttributes {
    pub name: Option<String>,
    pub hidden: Option<bool>,
}

impl NodeAttributes {
    pub fn new() -> NodeAttributes {
        NodeAttributes {
            name: None,
            hidden: None,
        }
    }

    pub fn to_dict(&self) -> Dict {
        Dict {
            num_of_pairs: self.num_of_pairs(),
            pairs: self.to_pairs(),
        }
    }

    pub fn num_of_pairs(&self) -> i32 {
        if self.name.is_some() && self.hidden.is_some() {
            2
        } else if self.name.is_some() || self.hidden.is_some() {
            1
        } else {
            0
        }
    }

    pub fn to_pairs(&self) -> Vec<(VoxString, VoxString)> {
        let mut pairs: Vec<(VoxString, VoxString)> = Vec::new();

        match self.name.clone() {
            Some(name) => pairs.push((
                VoxString::new(5, String::from("_name")),
                VoxString::new(name.len() as i32, name),
            )),
            None => {}
        };

        match self.hidden.clone() {
            Some(value) => pairs.push((
                VoxString::new(7, String::from("_hidden")),
                VoxString::new(1, bool_to_string(value)),
            )),
            None => {}
        };

        pairs
    }
}

#[derive(Debug, PartialEq)]
pub struct Transform {
    pub layer: i32,
    //need to make rotation type
    pub rotation: Option<u8>,
    pub translation: Option<(i32, i32, i32)>,
}

impl Transform {
    pub fn to_dict(&self) -> Dict {
        Dict {
            num_of_pairs: self.num_of_pairs(),
            pairs: self.get_pairs(),
        }
    }

    pub fn num_of_pairs(&self) -> i32 {
        if self.rotation.is_some() && self.translation.is_some() {
            2
        } else if self.rotation.is_some() || self.translation.is_some() {
            1
        } else {
            0
        }
    }

    pub fn get_pairs(&self) -> Vec<(VoxString, VoxString)> {
        let mut pairs = Vec::new();
        match self.rotation {
            Some(rot) => pairs.push((
                VoxString::new(2, String::from("_r")),
                VoxString::new(1, rot.to_string()),
            )),
            None => {}
        }

        match self.translation {
            Some(_) => pairs.push((
                VoxString::new(2, String::from("_t")),
                VoxString::new(
                    self.translation_to_string().len() as i32,
                    self.translation_to_string(),
                ),
            )),
            None => {}
        }

        pairs
    }

    pub fn translation_to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.translation.unwrap().0,
            self.translation.unwrap().1,
            self.translation.unwrap().2
        )
    }

}

pub fn bool_to_string(value: bool) -> String {
    if value {
        String::from("1")
    } else {
        String::from("0")
    }
}
