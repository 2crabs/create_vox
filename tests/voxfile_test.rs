use std::io::Read;

#[test]
fn write() {
    let mut vox = create_vox::VoxFile::load("magicavoxel.vox");
    vox.write("voxfile.vox");

    //println!("speed 1: {}", easybench::bench(|| {
    let mut file = std::fs::File::open("voxfile.vox").expect("failed to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read contents");
    //}));
    println!(
        "speed 2: {}",
        easybench::bench(|| {
            let mut vox2 = create_vox::VoxFile::load("voxfile.vox");
            vox2.write("voxfile.vox");
        })
    );
}
