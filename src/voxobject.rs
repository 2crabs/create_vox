use crate::color::*;
use crate::model::Model;
use crate::riff::write_chunk;
use crate::voxel::*;
use crate::*;
use std::fs::File;
use std::ops::AddAssign;

/// Holds all the information needed to create a vox file.
#[derive(Clone)]
//#[deprecated]
pub struct Voxobject {
    size: (u16, u16, u16),
    models: Vec<Model>,
    pub(crate) palette: [Color; 256],
}

impl Voxobject {
    ///Creates a new voxobject with the given size. Size needs to be between 1-255 on all axis.
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(100,150,100);
    /// ```
    pub fn new(size_x: u16, size_y: u16, size_z: u16) -> Voxobject {
        if size_x > 256 || size_y > 256 || size_z > 256 {
            panic!("size can not be greater than 256")
        }
        Voxobject {
            size: (size_x, size_y, size_z),
            models: vec![Model::new(size_x, size_y, size_z)],
            palette: [Color {
                r: 75,
                g: 75,
                b: 75,
                a: 255,
            }; 256],
        }
    }
    ///Adds a gradient between the 2 indexes in the palette with the 2 colors.
    ///
    /// # Example
    /// ```
    /// use create_vox::{Voxobject, Color};
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// let color1 = Color::new(255,255,0,255);
    /// let color2 = Color::new(0,255,255,255);
    /// my_vox.add_gradient(1,100,color1,color2);
    /// ```
    pub fn add_gradient(&mut self, index1: u8, index2: u8, color1: Color, color2: Color) {
        for i in index1..index2 {
            let fraction_between = ((i - index1) as f32) / ((index2 - index1) as f32);
            self.set_palette_color(
                i,
                get_middle(color1.r, color2.r, fraction_between),
                get_middle(color1.g, color2.g, fraction_between),
                get_middle(color1.b, color2.b, fraction_between),
                get_middle(color1.a, color2.a, fraction_between),
            )
        }
    }

    ///Adds one voxel to the voxobject.
    ///
    /// # Example
    /// ```
    /// use create_vox::{Voxobject, Voxel};
    ///
    /// let mut my_vox = Voxobject::new(20,20,20);
    /// let voxel = Voxel::new(0,0,0,100);
    /// my_vox.add_voxel(voxel).unwrap();
    /// ```
    pub fn add_voxel(&mut self, new_voxel: Voxel) -> Result<(), &str> {
        if (new_voxel.position.0 + 1) as u16 > self.size.0
            || (new_voxel.position.1 + 1) as u16 > self.size.1
            || (new_voxel.position.2 + 1) as u16 > self.size.2
        {
            return Err("Voxel position greater than Voxobject size");
        }
        self.models[0].voxels.push(new_voxel);
        Ok(())
    }

    ///Adds a voxel at the position specified.
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.add_voxel_at_pos(3,7,3,1).unwrap();
    /// ```
    pub fn add_voxel_at_pos(&mut self, x: u8, y: u8, z: u8, voxel_index: u8) -> Result<(), &str> {
        if (x + 1) as u16 > self.size.0
            || (y + 1) as u16 > self.size.1
            || (z + 1) as u16 > self.size.2
        {
            return Err("Position greater than Voxobject size");
        }
        self.models[0].voxels.push(Voxel::new(x, y, z, voxel_index));
        Ok(())
    }

    /// Deletes all voxels in the Voxobject
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.add_voxel_at_pos(3,7,3,1).unwrap();
    /// my_vox.add_voxel_at_pos(3,6,3,2).unwrap();
    /// my_vox.add_voxel_at_pos(3,5,3,3).unwrap();
    /// my_vox.clear_voxels();
    /// ```
    pub fn clear_voxels(&mut self) {
        self.models[0].voxels.clear();
    }

    /// Resets all indexes in the pallete to the default color
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.set_all_palette_color(255, 100, 0, 255);
    /// my_vox.reset_palette();
    /// ```
    pub fn reset_palette(&mut self) {
        self.palette = [Color {
            r: 75,
            g: 75,
            b: 75,
            a: 255,
        }; 256];
    }

    /// Number of voxes in the Voxobject
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.add_voxel_at_pos(3,7,3,1).unwrap();
    /// my_vox.add_voxel_at_pos(3,6,3,2).unwrap();
    /// my_vox.add_voxel_at_pos(3,5,3,3).unwrap();
    /// assert_eq!(3, my_vox.num_of_voxels())
    /// ```
    pub fn num_of_voxels(&self) -> i32 {
        self.models[0].voxels.len() as i32
    }

    /// Sets the size of a Voxobject
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.set_size(5,5,5);
    /// ```
    pub fn set_size(&mut self, x: u16, y: u16, z: u16) {
        if x > 256 || y > 256 || z > 256 {
            panic!("size can not be greater than 256");
        }
        self.size = (x, y, z);
        //remove voxels not inside object when resized
        self.check_voxels_pos();
    }

    /// Changes the size of the voxobject to fit the voxels
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(100,100,100);
    /// my_vox.add_voxel_at_pos(1,23,2,3);
    /// my_vox.add_cube(40,40,55,60,44,60,1);
    /// my_vox.auto_size();
    /// ```
    pub fn auto_size(&mut self) {
        let mut new_size = (1, 1, 1);
        let mut smallest_pos: (u8, u8, u8) = (255, 255, 255);

        //get smallest position of the voxels
        for voxel in self.models[0].voxels.iter() {
            if voxel.position.0 < smallest_pos.0 {
                smallest_pos.0 = voxel.position.0
            }
            if voxel.position.1 < smallest_pos.1 {
                smallest_pos.1 = voxel.position.1
            }
            if voxel.position.2 < smallest_pos.2 {
                smallest_pos.2 = voxel.position.2
            }
        }
        //move voxels
        for voxel in self.models[0].voxels.iter_mut() {
            voxel.position = (
                voxel.position.0 - smallest_pos.0,
                voxel.position.1 - smallest_pos.1,
                voxel.position.2 - smallest_pos.2,
            )
        }

        for voxel in self.models[0].voxels.iter() {
            if (voxel.position.0 as u16) > new_size.0 - 1 {
                new_size.0 = (voxel.position.0 + 1) as u16
            }
            if (voxel.position.1 as u16) > new_size.1 - 1 {
                new_size.1 = (voxel.position.1 + 1) as u16
            }
            if (voxel.position.2 as u16) > new_size.2 - 1 {
                new_size.2 = (voxel.position.2 + 1) as u16
            }
        }

        self.size = new_size
    }

    /// Sets the color of a specific index on the palette
    ///
    /// # Examples
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.set_palette_color(1,255,0,0,255);
    /// ```
    pub fn set_palette_color(&mut self, index: u8, r: u8, g: u8, b: u8, a: u8) {
        if index == 0 {
            panic!("index needs to be between 1 and 255");
        }
        let new_index = index - 1;
        self.palette[new_index as usize].r = r;
        self.palette[new_index as usize].g = g;
        self.palette[new_index as usize].b = b;
        self.palette[new_index as usize].a = a;
    }

    ///Gets the color from a index on the palette of the Voxobject
    ///
    /// #Example
    /// ```
    /// use create_vox::{Voxobject, Color};
    /// let mut vox = Voxobject::new(10,10,10);
    /// vox.set_palette_color(12, 30,0,40,255);
    ///
    /// assert!(Color::new(30, 0, 40, 255) == vox.get_palette_color(12));
    /// ```
    pub fn get_palette_color(&self, index: u8) -> Color {
        if index == 0 {
            panic!("index needs to be between 1 and 255");
        }
        let new_index = index - 1;

        self.palette[new_index as usize]
    }

    /// Sets color for all indexes on the palette
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.set_all_palette_color(0,255,0,255);
    /// ```
    pub fn set_all_palette_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for i in 0..255 {
            self.palette[i as usize].r = r;
            self.palette[i as usize].g = g;
            self.palette[i as usize].b = b;
            self.palette[i as usize].a = a;
        }
    }

    /// Adds a cube of voxels in the voxobject.
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    /// let mut my_vox = Voxobject::new(100,100,100);
    /// my_vox.add_cube(25,25,25,75,75,75,1).unwrap();
    /// ```
    pub fn add_cube(
        &mut self,
        startx: u8,
        starty: u8,
        startz: u8,
        endx: u8,
        endy: u8,
        endz: u8,
        colorindex: u8,
    ) -> Result<(), &str> {
        if endx as u16 > self.size.0 || endx as u16 > self.size.0 || endx as u16 > self.size.0 {
            return Err("Cube too large");
        }
        for currentx in startx..endx {
            for currenty in starty..endy {
                for currentz in startz..endz {
                    self.add_voxel(Voxel::new(currentx, currenty, currentz, colorindex))
                        .unwrap();
                }
            }
        }

        Ok(())
    }

    /// Checks if a voxel is at the position in a Voxobject
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    /// let mut vox = Voxobject::new(10,10,10);
    /// vox.add_voxel_at_pos(4,6,3,1);
    /// assert_eq!(true, vox.is_voxel_at_pos(4,6,3));
    /// ```
    pub fn is_voxel_at_pos(&self, x: u8, y: u8, z: u8) -> bool {
        for voxel in self.models[0].voxels.iter() {
            if voxel.position.0 == x && voxel.position.1 == y && voxel.position.2 == z {
                return true;
            }
        }

        return false;
    }

    pub fn add_model(&mut self, x: u16, y: u16, z: u16) {
        self.models.push(Model::new(x, y, z));
    }

    /// Keeps all of the voxels in the Voxobject that return true with the closure given
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut new_vox = Voxobject::new(10,10,10);
    /// new_vox.add_voxel_at_pos(1,1,1,6).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,2,5).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,3,6).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,4,7).unwrap();
    ///
    /// new_vox.retain_voxels(|voxel| voxel.colorindex == 6);
    ///
    /// assert_eq!(2, new_vox.num_of_voxels());
    /// ```
    pub fn retain_voxels<T>(&mut self, closure: T)
    where
        T: FnMut(&Voxel) -> bool,
    {
        self.models[0].voxels.retain(closure);
    }

    /// Changes all the voxels in the Voxobject with the closure
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut new_vox = Voxobject::new(10,10,10);
    /// new_vox.add_voxel_at_pos(1,1,1,6).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,2,5).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,3,6).unwrap();
    /// new_vox.add_voxel_at_pos(1,1,4,7).unwrap();
    ///
    /// new_vox.change_voxels(|voxel| voxel.colorindex = 3);
    /// ```
    pub fn change_voxels<T>(&mut self, mut closure: T)
    where
        T: FnMut(&mut Voxel) -> (),
    {
        let voxel_iter = self.models[0].voxels.iter_mut();

        for voxel in voxel_iter {
            closure(voxel);
        }
    }

    fn check_voxels_pos(&mut self) {
        let size = self.size;
        self.models[0].voxels.retain(|voxel| {
            (voxel.position.0 as u16) < size.0
                && (voxel.position.1 as u16) < size.1
                && (voxel.position.2 as u16) < size.2
        });
    }

    /// Moves all voxels given amount and removes ones that are no longer inside the Voxobject bounds
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(30,30,30);
    /// my_vox.add_cube(0,0,0,10,10,10,1).unwrap();
    /// my_vox.move_voxels(5, 4 ,5);
    /// ```
    pub fn move_voxels(&mut self, x: u8, y: u8, z: u8) {
        self.change_voxels(|voxel| {
            voxel.position.0 += x;
            voxel.position.1 += y;
            voxel.position.2 += z;
        });
        self.check_voxels_pos();
    }
    ///Creates a file and saves the voxobject to it
    ///
    /// # Example
    /// ```
    /// use create_vox::{Voxobject, Voxel};
    ///
    /// let mut my_vox = Voxobject::new(10,10,10);
    /// my_vox.set_all_palette_color(255,0,0,255);
    /// my_vox.add_voxel(Voxel::new(0,0,0,1)).unwrap();
    /// my_vox.save_as_file("my_vox.vox");
    /// ```
    pub fn save_as_file(&mut self, name: &str) {
        let empty_slice: &[u8] = &[0, 0, 0, 0];
        let file = std::fs::File::create(name).expect("Error");
        let mut buf_writer = std::io::BufWriter::new(file);

        let number_of_voxels = self.models[0].voxels.len() as u32;

        write_string_literal(&mut buf_writer, "VOX ");
        write_slice(&mut buf_writer, empty_slice);

        write_chunk("MAIN", 0, (number_of_voxels * 4) + 41, &mut buf_writer);

        for model in self.models.iter() {
            model.write(&mut buf_writer)
        }

        //the palette
        write_chunk("RGBA", 1024, 0, &mut buf_writer);
        //writes all of the colors in the palette
        for i in 0..256 {
            write_slice(
                &mut buf_writer,
                &[
                    self.palette[i].r,
                    self.palette[i].g,
                    self.palette[i].b,
                    self.palette[i].a,
                ],
            );
        }
    }

    /// Loads a vox file from the string given
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut loaded_vox = Voxobject::load("my_vox.vox").expect("failed to open file");
    /// loaded_vox.set_all_palette_color(255,0,255,255);
    /// loaded_vox.save_as_file("new_vox.vox");
    /// ```
    pub fn load(file_path: &str) -> Result<Voxobject, &str> {
        let mut file = match File::open(file_path) {
            Err(_) => return Err("failed to open file"),
            Ok(file) => file,
        };
        Ok(loader::load_voxobject(&mut file))
    }

    /// Loads a voxobject from the file given
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    /// use std::fs::File;
    ///
    /// let mut vox_file = File::open("my_vox.vox").expect("failed to open file");
    /// let mut loaded_vox = Voxobject::load_from_file(&mut vox_file);
    /// loaded_vox.set_all_palette_color(255,0,255,255);
    /// loaded_vox.save_as_file("new_vox.vox");
    /// ```
    pub fn load_from_file(file: &mut File) -> Voxobject {
        loader::load_voxobject(file)
    }
}

// impl Add for Voxobject {
//     type Output = Voxobject;
//
//     fn add(self, other: Voxobject) -> Voxobject {
//         let mut new_voxobject = Voxobject::new(self.size.0, self.size.1, self.size.2);
//         let mut other_voxels = other.voxels;
//         new_voxobject.voxels = self.models[0].voxels;
//         new_voxobject.palette = self.palette;
//         new_voxobject.voxels.append(&mut other_voxels);
//
//         new_voxobject
//     }
// }

// impl Add<Voxel> for Voxobject {
//     type Output = Voxobject;
//
//     fn add(self, other: Voxel) -> Voxobject {
//         let mut new_voxobject = self.clone();
//         new_voxobject.voxels.push(other);
//
//         new_voxobject
//     }
// }

// impl AddAssign for Voxobject {
//     fn add_assign(&mut self, other: Voxobject) {
//         self.models[0].voxels.append(&mut other.voxels.clone());
//     }
// }

impl AddAssign<Voxel> for Voxobject {
    fn add_assign(&mut self, other: Voxel) {
        self.models[0].voxels.push(other);
    }
}

// impl PartialEq for Voxobject {
//     fn eq(&self, other: &Voxobject) -> bool {
//         self.models[0].voxels == other..models[0].voxels
//     }
// }

//used for gradient
fn get_middle(a: u8, b: u8, point_between: f32) -> u8 {
    ((((b as i16) - (a as i16)) as f32 * point_between) + a as f32) as u8
}
