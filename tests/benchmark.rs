use std::fs::File;
use std::io::Read;

#[test]
fn bench() {
    let mut file = File::open("magicavoxel.vox").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut chunk_pos =
        create_vox::riff::find_chunk(&contents, String::from("nTRN"), 1).unwrap() as i32;
    let chunk = create_vox::riff::nTRN::read(&contents, &mut chunk_pos);
    println!("bench: {}", easybench::bench(|| { chunk.to_node() }))
}
