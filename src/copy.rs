use crate::node::Transform;
use crate::node::{Node, NodeAttributes, NodeType};

pub struct ModelCopy {
    pub model_id: i32,
    pub position: Option<(i32, i32, i32)>,
    pub rotation: Option<u8>,
    pub layer: Option<i32>,
    pub name: Option<String>,
}

impl ModelCopy {
    pub(crate) fn to_node(&self) -> Node {
        let mut attributes = NodeAttributes::new();
        attributes.name = self.name.clone();
        let mut transform_node = Node::new(NodeType::Transform(self.transform_data()), attributes);
        let shape_node = Node::new(NodeType::Shape(self.model_id), NodeAttributes::new());
        transform_node.add_child(shape_node);

        transform_node
    }

    pub(crate) fn transform_data(&self) -> Transform {
        Transform {
            layer: self.layer.unwrap_or_else(|| 0),
            rotation: match self.rotation {
                None => None,
                Some(rot) => Some(rot as i32),
            },
            translation: self.position,
        }
    }
}
