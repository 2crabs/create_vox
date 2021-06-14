use crate::copy::ModelCopy;
use crate::layer::Layer;
use crate::model::Model;
use crate::node::{Node, NodeAttributes, NodeType, Transform};
use crate::Color;

/// Struct which holds all data for a .vox file such as models and palette
pub struct VoxFile {
    pub models: Vec<Model>,
    pub palette: [Color; 256],
    pub root_node: Node,
    pub layers: Vec<Layer>,
    pub copies: Vec<ModelCopy>,
}

impl VoxFile {
    //size in bytes when written
    pub(in crate::voxfile) fn get_size(&self) -> i32 {
        let mut size = 1024;
        for model in self.models.iter() {
            size += model.get_size();
        }
        for layer in self.layers.iter() {
            size += layer.get_size()
        }
        size += self.root_node.get_all_size();
        size
    }

    pub(in crate::voxfile) fn make_nodes(&mut self) {
        let mut root_node = Node::new(
            NodeType::Transform(Transform::default()),
            NodeAttributes::new(),
        );
        let mut group_node = Node::new(NodeType::Group, NodeAttributes::new());

        for model in self.models.iter() {
            group_node.add_child(model.to_node());
        }

        for copy in self.copies.iter() {
            group_node.add_child(copy.to_node());
        }
        root_node.add_child(group_node);
        self.root_node = root_node
    }

    //takes data from nodes and applies it to models
    pub(crate) fn get_node_data(&mut self) {
        let mut used_model_ids = Vec::new();
        self.root_node
            .clone()
            .get_child_data_to_models(self, &mut used_model_ids)
    }

    //(id, pos, layer, rot, name)
    pub(crate) fn check_transform(
        transform_node: &Node,
    ) -> Option<(
        i32,
        Option<(i32, i32, i32)>,
        Option<i32>,
        Option<u8>,
        Option<String>,
    )> {
        let id: i32;
        let pos: Option<(i32, i32, i32)>;
        let layer: Option<i32>;
        let rot: Option<u8>;

        match transform_node.node_type {
            NodeType::Transform(ref trans) => {
                pos = trans.translation;
                layer = Some(trans.layer);
                rot = match trans.rotation {
                    None => None,
                    Some(rot) => Some(rot as u8),
                };
            }
            _ => return None,
        }

        if transform_node.has_child_shape() {
            match transform_node.children[0].node_type {
                NodeType::Shape(shape_id) => {
                    id = shape_id;
                }
                _ => return None,
            }
        } else {
            return None;
        }

        let name = transform_node.attributes.name.clone();

        Some((id, pos, layer, rot, name))
    }

    //finds model by id and edits it with given data
    pub(crate) fn change_model_data(
        &mut self,
        id: i32,
        pos: Option<(i32, i32, i32)>,
        layer: Option<i32>,
        rot: Option<u8>,
        name: Option<String>,
    ) {
        for model in self.models.iter_mut() {
            if model.id == id {
                model.position = pos;
                model.layer = layer;
                model.rotation = rot;
                model.name = name.clone();
            }
        }
    }

    pub(crate) fn add_copy(
        &mut self,
        id: i32,
        pos: Option<(i32, i32, i32)>,
        layer: Option<i32>,
        rot: Option<u8>,
        name: Option<String>,
    ) {
        self.copies.push(ModelCopy {
            model_id: id,
            position: pos,
            rotation: rot,
            layer,
            name,
        })
    }

    /// creates a new voxfile with one model with the size given.
    ///
    /// # Example
    /// ```
    /// use create_vox::VoxFile;
    ///
    /// let vox = VoxFile::new(30, 10, 10);
    /// assert_eq!(vox.models[0].size, (30, 10, 10));
    /// ```
    pub fn new(size_x: u16, size_y: u16, size_z: u16) -> VoxFile {
        if size_x > 256 || size_y > 256 || size_z > 256 {
            panic!("size can not be greater than 256")
        }
        VoxFile {
            models: vec![Model::new(size_x, size_y, size_z)],
            palette: [Color {
                r: 75,
                g: 75,
                b: 75,
                a: 255,
            }; 256],
            root_node: Node::new(NodeType::Group, NodeAttributes::new()),
            layers: vec![],
            copies: vec![],
        }
    }

    pub fn save(&mut self, file_path: &str) {
        self.write(file_path);
    }

    /// Add a copy of a model at a certain position. The model id is which model in the array of models to use.
    ///
    /// # Example
    /// ```
    /// use create_vox::VoxFile;
    ///
    /// let mut vox = VoxFile::new(10, 10, 10);
    /// vox.add_model_copy(0, 5, 10, 5);
    /// ```
    pub fn add_model_copy(&mut self, model_id: i32, x: i32, y: i32, z: i32) {
        self.add_copy(model_id, Some((x, y, z)), None, None, None);
    }

    /// Creates a new layer and returns the id that it has.
    ///
    /// # Example
    /// ```
    /// use create_vox::VoxFile;
    ///
    /// let mut vox = VoxFile::new(10, 10, 10);
    /// vox.models[0].layer = Some(vox.add_layer(String::from("my layer"), false));
    /// ```
    pub fn add_layer(&mut self, name: String, hidden: bool) -> i32{
        self.layers.push(Layer::new(name, hidden, self.layers.len() as i32));
        self.layers.len() as i32
    }
}
