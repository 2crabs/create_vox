mod voxobject;
mod writing;
mod color;
mod voxel;

use std::io::Write;
pub use writing::*;
pub use color::*;
pub use voxel::*;
pub use voxobject::*;


fn i32_to_array(a: u32) -> [u8;4]{
    a.to_ne_bytes()
}

fn u16_to_array(a: u16) -> [u8;2]{
    a.to_ne_bytes()
}

fn get_middle(a: u8, b: u8, point_between: f32) -> u8{
    ((((b as i16)-(a as i16)) as f32 * point_between)+ a as f32) as u8
}

#[cfg(test)]
mod tests {
    use crate::writing::*;
    use crate::color::*;
    use crate::voxel::*;
    use crate::voxobject::*;

    #[test]
    fn it_works() {
        let mut my_vox = Voxobject::new(256,256,256);
        my_vox.set_all_palette_color(255,0,0,255);
        my_vox.add_voxel(Voxel::new(0,0,0,1)).unwrap();
        let color1 = Color::new(255,255,0,255);
        let color2 = Color::new(0,255,255,255);
        my_vox.add_gradient(1,100,color1,color2);
        my_vox.add_cube(0,0,0,255,255,255,70).unwrap();
        my_vox.save_as_file("myvox.vox");
    }

    #[test]
    #[should_panic]
    fn size_too_big(){
        let test_vox = Voxobject::new(254,300,10);
    }

    #[test]
    #[should_panic]
    fn incorrect_index(){
        let mut test_vox = Voxobject::new(10,10,10);
        test_vox.set_palette_color(0,255,255,255,255);
    }

    #[test]
    fn red_cube(){
        let mut cube_vox = Voxobject::new(100,100,100);
        cube_vox.set_palette_color(255,255,0,0,255);
        cube_vox.add_cube(25,25,25,75,75,75,255).unwrap();
        cube_vox.save_as_file("red_cube.vox");
    }

    #[test]
    fn one_voxel(){
        let mut voxel_vox = Voxobject::new(20,20,20);
        voxel_vox.set_palette_color(255,255,0,0,255);
        voxel_vox.add_voxel(Voxel::new(0,0,0,255)).unwrap();
        voxel_vox.save_as_file("voxel.vox");
    }
    #[test]
    fn out_of_range_voxel(){
        let mut vox = Voxobject::new(3,3,3);
        let voxel = Voxel::new(2,1,5,1);
        assert_eq!(Err("Voxel position greater than Voxobject size"), vox.add_voxel(voxel));
    }
}
