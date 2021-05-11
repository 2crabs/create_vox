use std::io::Read;

#[test]
fn write() {
    let mut vox = create_vox::VoxFile::load("magicavoxel.vox");
    vox.write("voxfile.vox");

    println!(
        "speed 2: {}",
        easybench::bench(|| {
            let mut vox2 = create_vox::VoxFile::load("voxfile.vox");
            vox2.write("voxfile.vox");
        })
    );
}
