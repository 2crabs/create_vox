use create_vox::{Color, Voxel, Voxobject, VoxFile};
#[test]
//#[ignore]
fn it_works() {
    // let mut my_vox = Voxobject::new(256, 256, 256);
    // my_vox.set_all_palette_color(255, 0, 0, 255);
    // my_vox.add_voxel(Voxel::new(0, 0, 0, 1)).unwrap();
    // let color1 = Color::new(255, 255, 0, 255);
    // let color2 = Color::new(0, 255, 255, 255);
    // my_vox.add_gradient(1, 100, color1, color2);
    // my_vox.add_cube(0, 0, 0, 255, 255, 255, 70).unwrap();
    // my_vox.save_as_file("myvox.vox");


    let mut my_vox = VoxFile::new(256, 256, 256);
    my_vox.models[0].add_cube(0, 0, 0, 255, 255, 255, 70).unwrap();
    my_vox.save("tester.vox");
}

#[test]
#[should_panic]
fn size_too_big() {
    //Voxobject::new(254, 300, 10);
}

#[test]
#[should_panic]
fn incorrect_index() {
    //let mut test_vox = Voxobject::new(10, 10, 10);
    //test_vox.set_palette_color(0, 255, 255, 255, 255);
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
