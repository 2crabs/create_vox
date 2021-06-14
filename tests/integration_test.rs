use create_vox::{Color, VoxFile, Voxel};
#[test]
//#[ignore]
fn it_works() {
    let mut vox = VoxFile::new(10, 10, 10);
    vox.models[0].add_cube(0,0,0,5,5,5, 2).unwrap();
    vox.set_palette_color(2, 255, 100, 0, 255);
    vox.add_model_copy(0, 10,10,10);
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
    test_vox.models[0].add_voxel_at_pos(3,4,2,1);
    assert_eq!(true, test_vox.models[0].is_voxel_at_pos(3, 4, 2));
}

#[test]
fn red_cube() {
    //let mut cube_vox = Voxobject::new(100, 100, 100);
    //cube_vox.set_palette_color(255, 255, 0, 0, 255);
    //cube_vox.add_cube(25, 25, 25, 75, 75, 75, 255).unwrap();
    //cube_vox.save_as_file("red_cube.vox");
}

#[test]
fn one_voxel() {
    //let mut voxel_vox = Voxobject::new(20, 20, 20);
    //voxel_vox.set_palette_color(255, 255, 0, 0, 255);
    //voxel_vox.add_voxel(Voxel::new(0, 0, 0, 255)).unwrap();
    //voxel_vox.save_as_file("voxel.vox");
}
#[test]
fn out_of_range_voxel() {
    // let mut vox = Voxobject::new(3, 3, 3);
    // let voxel = Voxel::new(2, 1, 5, 1);
    // assert_eq!(
    //     Err("Voxel position greater than Voxobject size"),
    //     vox.add_voxel(voxel)
    // );
}

#[test]
fn comparing() {
    // let color1 = Color::new(255, 10, 40, 255);
    // let color2 = Color::new(255, 13, 40, 255);
    // let are_equal = color1 == color2;
    // assert_eq!(false, are_equal);
}

#[test]
fn addition() {
    // let mut vox1 = Voxobject::new(256, 256, 256);
    // let vox2 = Voxobject::new(256, 256, 256);
    //currently does not work
    //vox1 += vox2;
}

#[test]
fn large_cube() {
    // let mut vox = Voxobject::new(256, 256, 256);
    //
    // vox.add_voxel_at_pos(5, 10, 4, 3).unwrap();
    // vox.add_voxel_at_pos(5, 13, 4, 3).unwrap();
    // vox.add_voxel_at_pos(1, 23, 2, 3).unwrap();
    // vox.add_cube(40, 40, 55, 60, 44, 60, 1).unwrap();
    // vox.auto_size();
    //
    // vox.save_as_file("large.vox");
}

#[test]
fn retain() {
    // let mut new_vox = Voxobject::new(10, 10, 10);
    // new_vox.add_voxel_at_pos(1, 1, 1, 6).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 1, 5).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 1, 6).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 1, 7).unwrap();
    //
    // new_vox.retain_voxels(|voxel| voxel.colorindex == 6);
    //
    // assert_eq!(2, new_vox.num_of_voxels());
}

#[test]
fn voxel_change() {
    // let mut new_vox = Voxobject::new(10, 10, 10);
    // new_vox.add_voxel_at_pos(1, 1, 1, 6).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 2, 5).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 3, 6).unwrap();
    // new_vox.add_voxel_at_pos(1, 1, 4, 7).unwrap();
    //
    // new_vox.change_voxels(|voxel| voxel.colorindex = 3);
    // new_vox.save_as_file("changed_voxels.vox");
}
