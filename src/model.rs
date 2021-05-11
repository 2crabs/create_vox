use crate::convert::*;
use crate::node::{Node, NodeAttributes, NodeType, Transform};
use crate::riff::write_chunk;
use crate::writing::*;
use crate::*;
use std::fs::File;
use std::io::BufWriter;

#[derive(Clone)]
pub struct Model {
    pub size: (u16, u16, u16),
    pub(crate) voxels: Vec<Voxel>,
    pub position: Option<(i32, i32, i32)>,
    pub rotation: Option<u8>,
    pub layer: Option<i32>,
    pub name: Option<String>,
    pub id: i32,
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Model {
    pub fn new(x: u16, y: u16, z: u16) -> Model {
        Model {
            size: (x, y, z),
            voxels: Vec::new(),
            position: None,
            rotation: None,
            layer: None,
            name: None,
            id: 0,
        }
    }

    pub fn write(&self, writer: &mut BufWriter<File>) {
        let size_slice: &[u8] = &[
            u16_to_array(self.size.0)[0],
            u16_to_array(self.size.0)[1],
            0,
            0,
            u16_to_array(self.size.1)[0],
            u16_to_array(self.size.1)[1],
            0,
            0,
            u16_to_array(self.size.2)[0],
            u16_to_array(self.size.2)[1],
            0,
            0,
        ];
        write_chunk("SIZE", 12, 0, writer);
        //writes the slice for size
        write_slice(writer, size_slice);

        write_chunk("XYZI", ((self.voxels.len() as u32) * 4) + 4, 0, writer);
        //number voxels in the voxobject
        write_slice(writer, &u32_to_array(self.voxels.len() as u32));
        //writes all of the voxels
        self.write_voxels(writer);
    }

    fn write_voxels(&self, buf_writer: &mut BufWriter<File>) {
        for i in 0..self.voxels.len() {
            buf_writer
                .write(&[
                    self.voxels[i].position.0,
                    self.voxels[i].position.1,
                    self.voxels[i].position.2,
                    self.voxels[i].colorindex,
                ])
                .expect("failed to write voxels");
        }
    }

    //start at size chunk
    pub(crate) fn read(input: &Vec<u8>, cursor: &mut i32, id: i32) -> Model {
        use crate::riff::i32_from_vec;
        *cursor += 12;
        let size_x = i32_from_vec(input, cursor) as u16;
        *cursor += 4;
        let size_y = i32_from_vec(input, cursor) as u16;
        *cursor += 4;
        let size_z = i32_from_vec(input, cursor) as u16;
        *cursor += 16;

        let num_of_voxels = i32_from_vec(input, cursor);
        *cursor += 4;
        let mut voxels = Vec::new();
        for i in 0..num_of_voxels {
            let x = input[(*cursor + 4 * i) as usize];
            let y = input[(*cursor + 1 + 4 * i) as usize];
            let z = input[(*cursor + 2 + 4 * i) as usize];
            let index = input[(*cursor + 3 + 4 * i) as usize];
            voxels.push(Voxel::new(x, y, z, index))
        }

        Model {
            size: (size_x, size_y, size_z),
            voxels,
            position: None,
            rotation: None,
            layer: None,
            name: None,
            id,
        }
    }

    pub fn to_node(&self) -> Node {
        let mut attributes = NodeAttributes::new();
        attributes.name = self.name.clone();
        let mut transform_node = Node::new(NodeType::Transform(self.transform_data()), attributes);
        let shape_node = Node::new(NodeType::Shape(self.id), NodeAttributes::new());
        transform_node.add_child(shape_node);

        transform_node
    }

    //puts data into Transform struct
    pub fn transform_data(&self) -> Transform {
        Transform {
            layer: self.layer.unwrap_or_else(|| 0),
            rotation: match self.rotation {
                None => None,
                Some(rot) => Some(rot as i32),
            },
            translation: self.position,
        }
    }

    pub(crate) fn get_size(&self) -> i32 {
        self.voxels.len() as i32 * 4 + 4
    }
}
