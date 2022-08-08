use create_vox::{Color, VoxFile, Voxel};
#[test]
//#[ignore]
fn it_works() {
    let mut vox = VoxFile::new(10, 10, 10);
    vox.models[0].add_cube(0, 0, 0, 5, 5, 5, 2).unwrap();
    vox.set_palette_color(2, 255, 100, 0, 255);
    vox.add_model_copy(0, 10, 10, 10);
    vox.add_layer(String::from("cool layer"), false);
    vox.models[0].layer = Some(1);
    vox.save("tester.vox");
}

#[test]
#[should_panic]
fn size_too_big() {
    VoxFile::new(254, 300, 10);
}

#[test]
fn voxel_at_pos() {
    let mut test_vox = VoxFile::new(10, 10, 10);
    test_vox.models[0].add_voxel_at_pos(3, 4, 2, 1);
    assert_eq!(true, test_vox.models[0].is_voxel_at_pos(3, 4, 2));
}
#[test]
fn adding_cube(){
    let mut test_vox = VoxFile::new(50, 1, 15);
    test_vox.models[0].add_cube(0,0,0, 40, 1,15,1).expect("Failed to add cube");
}
