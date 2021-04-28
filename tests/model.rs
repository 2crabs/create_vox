use create_vox::model::Model;
use create_vox::*;
use std::fs::File;
use std::io::{BufWriter, Read};

#[test]
fn writing_model() {
    let mut vox = Voxobject::new(10, 10, 10);

    vox.add_model(1, 3, 4);
    vox.add_voxel_at_pos(1, 1, 1, 1);
    vox.save_as_file("model.vox");
}

#[test]
fn read_model() {
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    Model::read(
        &contents,
        &mut (riff::find_chunk(&contents, String::from("SIZE"), 1).unwrap() as i32),
    );
}
