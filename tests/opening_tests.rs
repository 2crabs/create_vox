use create_vox::Voxobject;
use std::fs::File;

#[test]
fn load() {
    let mut new_vox = Voxobject::load("loaded.vox").expect("failed to open in opening_test");
    new_vox.save_as_file("loaded_copy.vox");
}

#[test]
fn loading_from_file() {
    let mut file = File::open("loaded.vox").expect("failed to load file");
    let mut new_vox = Voxobject::load_from_file(&mut file);
    new_vox.save_as_file("loaded_copy.vox");
}
