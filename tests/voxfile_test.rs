#[test]
fn write(){
    let vox = create_vox::voxfile::VoxFile::load("magicavoxel.vox");
    vox.write("voxfile.vox");
    let vox2 = create_vox::voxfile::VoxFile::load("voxfile.vox");
    vox2.write("voxfile2.vox");
}