use create_vox::{Color, Voxobject, Voxel};
#[test]
fn load(){
    let mut new_vox = create_vox::loader::load_voxobject("loaded.vox");
    new_vox.save_as_file("loaded_copy.vox");
}