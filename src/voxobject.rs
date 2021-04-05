use crate::voxel::*;
use crate::color::*;
use crate::*;

/// Holds all the information needed to create a vox file.
pub struct Voxobject{
    size: (u16, u16, u16),
    voxels: Vec<Voxel>,
    palette: [Color; 256]
}

impl Voxobject{
    ///Creates a new voxobject with the given size. Size needs to be between 1-255 on all axis.
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxobject;
    ///
    /// let mut my_vox = Voxobject::new(100,150,100);
    /// ```
    pub fn new(size_x: u16, size_y: u16, size_z: u16) -> Voxobject{
        if size_x > 256 || size_y > 256 || size_z > 256 {
            panic!("size can not be greater than 256")
        }
        Voxobject{
            size: (size_x, size_y, size_z),
            voxels: Vec::new(),
            palette: [Color {r:75,g:75,b:75,a:255};256]
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
    pub fn add_gradient(&mut self, index1: u8, index2: u8, color1: Color, color2: Color){
        for i in index1..index2 {
            let fraction_between = ((i-index1) as f32) / ((index2-index1) as f32);
            self.set_palette_color(i,
                                   get_middle(color1.r,color2.r, fraction_between),
                                   get_middle(color1.g,color2.g, fraction_between),
                                   get_middle(color1.b,color2.b, fraction_between),
                                   get_middle(color1.a,color2.a, fraction_between)
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
    pub fn add_voxel(&mut self, new_voxel: Voxel) -> Result<(), &str>{
        if (new_voxel.position.0 + 1) as u16 > self.size.0 ||
            (new_voxel.position.1 + 1) as u16 > self.size.1 ||
            (new_voxel.position.2 + 1) as u16 > self.size.2 {
            return Err("Voxel position greater than Voxobject size");
        }
        self.voxels.push(new_voxel);
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
    pub fn add_voxel_at_pos(&mut self, x: u8, y: u8, z: u8, voxel_index: u8) -> Result<(), &str>{
        if (x + 1) as u16 > self.size.0 ||
            (y + 1) as u16 > self.size.1 ||
            (z + 1) as u16 > self.size.2 {
            return Err("Position greater than Voxobject size");
        }
        self.voxels.push(Voxel::new(x, y, z, voxel_index));
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
    pub fn clear_voxels(&mut self){
        self.voxels.clear();
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
    pub fn reset_palette(&mut self){
        self.palette = [Color {r:75,g:75,b:75,a:255};256];
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
    pub fn num_of_voxels(&self) -> i32{
        self.voxels.len() as i32
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
    pub fn set_size(&mut self, x: u16, y: u16, z: u16){
        if x > 256 || y > 256 || z > 256 {
            panic!("size can not be greater than 256");
        }
        self.size = (x, y, z);
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
    pub fn set_palette_color(&mut self,index: u8,r: u8,g: u8,b: u8,a: u8){
        if index == 0 {
            panic!("index needs to be between 1 and 255");
        }
        let new_index = index - 1;
        self.palette[new_index as usize].r = r;
        self.palette[new_index as usize].g = g;
        self.palette[new_index as usize].b = b;
        self.palette[new_index as usize].a = a;
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
    pub fn set_all_palette_color(&mut self,r: u8,g: u8,b: u8,a: u8){
        for i in 0..255{
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
    pub fn add_cube(&mut self,startx: u8,starty: u8,startz: u8,endx: u8,endy: u8,endz: u8,colorindex: u8) -> Result<(), &str>{
        if endx as u16 > self.size.0 || endx as u16 > self.size.0 || endx as u16 > self.size.0 {
            return Err("Cube too large");
        }
        for currentx in startx..endx{
            for currenty in starty..endy{
                for currentz in startz..endz{
                    self.add_voxel(Voxel::new(currentx, currenty, currentz, colorindex)).unwrap();
                }
            }
        }

        Ok(())
    }


    fn write_voxels(&self, buf_writer: &mut std::io::BufWriter<std::fs::File>){
        for i in 0..self.voxels.len(){
            buf_writer.write(&[self.voxels[i].position.0,self.voxels[i].position.1,self.voxels[i].position.2,self.voxels[i].colorindex]).expect("failed to write voxels");
        }
    }

    ///Creates a file and saves the voxobject to it.
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
    pub fn save_as_file(&mut self,name: &str){

        let empty_slice: &[u8] = &[0,0,0,0];
        let size_slice: &[u8] = &[
            u16_to_array(self.size.0)[0],u16_to_array(self.size.0)[1],0,0,
            u16_to_array(self.size.1)[0],u16_to_array(self.size.1)[1],0,0,
            u16_to_array(self.size.2)[0],u16_to_array(self.size.2)[1],0,0
        ];
        let file = std::fs::File::create(name).expect("Error");
        let mut buf_writer = std::io::BufWriter::new(file);

        let number_of_voxels = self.voxels.len() as u32;

        write_string_literal(&mut buf_writer, "VOX ");
        write_slice(&mut buf_writer, empty_slice);

        write_string_literal(&mut buf_writer, "MAIN");
        write_slice(&mut buf_writer, empty_slice);
        write_slice(&mut buf_writer, &i32_to_array((number_of_voxels*4)+41));

        write_string_literal(&mut buf_writer, "SIZE");
        write_slice(&mut buf_writer, &[12,0,0,0]);
        write_slice(&mut buf_writer, empty_slice);
        write_slice(&mut buf_writer, size_slice);

        write_string_literal(&mut buf_writer, "XYZI");
        write_slice(&mut buf_writer, &i32_to_array((number_of_voxels*4)+4));
        write_slice(&mut buf_writer, empty_slice);
        write_slice(&mut buf_writer, &i32_to_array(number_of_voxels));
        self.write_voxels(&mut buf_writer);

        write_string_literal(&mut buf_writer, "RGBA");
        write_slice(&mut buf_writer, &[0,4,0,0]);
        write_slice(&mut buf_writer, empty_slice);
        //max value of i is 255
        for i in 0..256{
            write_slice(&mut buf_writer, &[self.palette[i].r,self.palette[i].g,self.palette[i].b,self.palette[i].a]);
        }

    }
}