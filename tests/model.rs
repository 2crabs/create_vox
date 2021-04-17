use create_vox::*;
use std::fs::File;
use std::io::BufWriter;

#[test]
fn writing_model(){
    let mut vox = Voxobject::new(10,10,10);

    vox.add_model(1, 3, 4);
    vox.add_voxel_at_pos(1,1,1,1);
    vox.save_as_file("model.vox");
}