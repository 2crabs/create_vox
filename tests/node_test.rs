use create_vox::node::*;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

#[test]
fn node_add() {
    let mut node = Node {
        node_type: NodeType::Transform(Transform {
            layer: 0,
            rotation: Some(0),
            translation: Some((0, 0, 0)),
        }),
        id: 0,
        children_ids: vec![],
        attributes: NodeAttributes {
            name: None,
            hidden: None,
        },
        children: Vec::new(),
    };

    node.add_child(Node::new(NodeType::Group, NodeAttributes::new()));
    assert_eq!(node.children.len(), 1)
}

#[test]
fn make_tree() {
    let mut file = File::open("magicavoxel2.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut node = create_vox::riff::nodes_from_chunks(&contents);

    println!("node is: {:?}", node);
    println!(
        "number of nodes: {}",
        create_vox::riff::num_of_chunks(&contents, String::from("nTRN"))
            + create_vox::riff::num_of_chunks(&contents, String::from("nGRP"))
            + create_vox::riff::num_of_chunks(&contents, String::from("nSHP"))
    );
    //println!("bench: {}", easybench::bench(|| {create_vox::riff::nodes_from_chunks(&contents);}))

    //recursion thing ¯\_(ツ)_/¯
    //println!("shallow children: {}", node.children[0].children.len());
    println!("type: {:?}", node.node_type);
    println!("number of children is: {}", node.num_children());

    //if I delete these it breaks
    {
        let new_file = File::create("Nodes.vox").unwrap();
        let mut writer = BufWriter::new(new_file);
        writer.write(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        node.write_all(&mut writer);
    }

    let mut node_file = File::open("Nodes.vox").unwrap();
    let mut node_contents = Vec::new();
    node_file
        .read_to_end(&mut node_contents)
        .expect("failed to read file contents");

    let new_node = create_vox::riff::nodes_from_chunks(&node_contents);

    assert_eq!(node, new_node);

    println!(
        "bench: {}",
        easybench::bench(|| {
            create_vox::riff::nodes_from_chunks(&contents);
        })
    );

    println!("size is: {}", node.get_all_size());
}
