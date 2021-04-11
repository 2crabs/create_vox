use create_vox::Voxobject;

#[test]
fn load(){
    let mut new_vox = Voxobject::load("loaded.vox");
    new_vox.save_as_file("loaded_copy.vox");
}