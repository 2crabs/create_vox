use create_vox::{Color, Voxobject, Voxel};
#[test]
fn load(){
    create_vox::loader::load_voxobject("loaded.vox");
}