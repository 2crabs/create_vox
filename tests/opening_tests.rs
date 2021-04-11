use create_vox::{Color, Voxobject, Voxel};
use std::fs::File;

#[test]
fn load(){
    let mut new_vox = Voxobject::load("loaded.vox");
    new_vox.save_as_file("loaded_copy.vox");
}