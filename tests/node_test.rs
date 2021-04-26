use create_vox::node::*;

#[test]
fn node_add(){
    let mut node = Node{
        node_type: NodeType::Transform(Transform{
            layer: 0,
            rotation: 0,
            translation: (0, 0, 0)
        }),
        attributes: NodeAttributes{
            name: None,
            hidden: None
        },
        child: None
    };

    node.add_child(NodeType::Group);
    assert_eq!(node.child.unwrap().len(), 1)
}