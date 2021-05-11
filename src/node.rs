use crate::riff::{nGRP, nSHP, nTRN, Dict, VoxString};
use crate::VoxFile;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Transform(Transform),
    Group,
    //shape with id of model.
    Shape(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub id: i32,
    pub children_ids: Vec<i32>,
    pub attributes: NodeAttributes,
    pub children: Vec<Node>,
}

impl Node {
    pub fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn new(node_type: NodeType, attribs: NodeAttributes) -> Node {
        Node {
            node_type,
            id: 0,
            children_ids: vec![],
            attributes: attribs,
            children: Vec::new(),
        }
    }

    pub fn number_nodes(&mut self, start: i32) -> i32 {
        let mut id = start;
        self.id = start;
        for child in self.children.iter_mut() {
            id += 1;
            id = child.number_nodes(id);
        }
        id
    }

    pub fn number_children_ids(&mut self) {
        for child in self.children.iter_mut() {
            self.children_ids.push(child.id);
            child.number_children_ids();
        }
    }

    //number of nodes below this node
    pub fn num_children(&self) -> i32 {
        let mut num = 0;
        num += self.children.len() as i32;
        for child in self.children.iter() {
            num += child.num_children();
        }

        num
    }
    pub fn write(&self, buf_writer: &mut BufWriter<File>) {
        match &(*self).node_type {
            NodeType::Transform(trans) => {
                nTRN {
                    node_id: self.id,
                    node_attributes: self.attributes.to_dict(),
                    child_node_id: self.children_ids[0],
                    reserved_id: -1,
                    layer_id: trans.layer,
                    num_of_frames: 1,
                    frame_attributes: trans.to_dict(),
                }
                .write(buf_writer);
            }

            NodeType::Group => nGRP {
                node_id: self.id,
                node_attributes: self.attributes.to_dict(),
                num_of_children_nodes: self.children.len() as i32,
                child_id: self.children_ids.clone(),
            }
            .write(buf_writer),

            NodeType::Shape(model_id) => nSHP {
                node_id: self.id,
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

    pub fn write_children(&self, buf_writer: &mut BufWriter<File>) {
        for child in self.children.iter() {
            child.write(buf_writer);
            child.write_children(buf_writer);
        }
    }

    pub fn write_all(&mut self, buf_writer: &mut BufWriter<File>) {
        self.number_nodes(0);
        self.number_children_ids();
        self.write(buf_writer);
        self.write_children(buf_writer)
    }

    pub fn get_size(&self) -> i32 {
        match &(*self).node_type {
            NodeType::Transform(trans) => nTRN {
                node_id: 0,
                node_attributes: self.attributes.to_dict(),
                child_node_id: 0,
                reserved_id: 0,
                layer_id: -1,
                num_of_frames: 1,
                frame_attributes: trans.to_dict(),
            }
            .get_size(),

            NodeType::Group => nGRP {
                node_id: 0,
                node_attributes: self.attributes.to_dict(),
                num_of_children_nodes: self.children.len() as i32,
                child_id: vec![0; self.children.len()],
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
            }
            .get_size(),
        }
    }

    pub fn get_children_size(&self, size: &mut i32) {
        for child in self.children.iter() {
            *size += child.get_size();
            child.get_children_size(size);
        }
    }

    pub fn get_all_size(&self) -> i32 {
        let mut size = self.get_size();
        self.get_children_size(&mut size);
        size
    }

    pub fn has_child_shape(&self) -> bool {
        for child in self.children.iter() {
            match child.node_type {
                NodeType::Shape(_) => return true,
                _ => {}
            }
        }
        false
    }

    pub fn get_child_data_to_models(&self, voxfile: &mut VoxFile, used_ids: &mut Vec<i32>) {
        self.make_model_data(voxfile, used_ids);
        for child in self.children.iter() {
            child.get_child_data_to_models(voxfile, used_ids);
        }
    }

    pub fn make_model_data(&self, voxfile: &mut VoxFile, used_ids: &mut Vec<i32>) {
        let data = VoxFile::check_transform(self);
        if data.is_some() {
            if used_ids.contains(&data.as_ref().unwrap().0) {
                voxfile.add_copy(
                    data.as_ref().unwrap().0,
                    data.as_ref().unwrap().1,
                    data.as_ref().unwrap().2,
                    data.as_ref().unwrap().3,
                    data.as_ref().unwrap().4.clone(),
                );
            } else {
                voxfile.change_model_data(
                    data.as_ref().unwrap().0,
                    data.as_ref().unwrap().1,
                    data.as_ref().unwrap().2,
                    data.as_ref().unwrap().3,
                    data.as_ref().unwrap().4.clone(),
                );
                used_ids.push(data.as_ref().unwrap().0);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

        match self.hidden {
            Some(value) => pairs.push((
                VoxString::new(7, String::from("_hidden")),
                VoxString::new(1, bool_to_string(value)),
            )),
            None => {}
        };

        pairs
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transform {
    pub layer: i32,
    //need to make rotation type
    pub rotation: Option<i32>,
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
                //it was here
                VoxString::new(rot.to_string().len() as i32, rot.to_string()),
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

    pub fn default() -> Transform {
        Transform {
            layer: 0,
            rotation: None,
            translation: None,
        }
    }
}

pub fn bool_to_string(value: bool) -> String {
    if value {
        String::from("1")
    } else {
        String::from("0")
    }
}
