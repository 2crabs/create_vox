#[test]
fn write(){
    let mut vox = create_vox::voxfile::VoxFile::load("magicavoxel.vox");
    vox.write("voxfile.vox");
    let mut vox2 = create_vox::voxfile::VoxFile::load("voxfile.vox");
    vox2.write("voxfile2.vox");
}