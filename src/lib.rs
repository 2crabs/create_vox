use std::io::Write;
//VOXOBJECT
pub struct Voxobject{
    size: (u16, u16, u16),
    voxels: Vec<Voxel>,
    palette: [Color; 256]
}

impl Voxobject{
    //needs to be created
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
    pub fn add_voxel(&mut self,new_voxel: Voxel){
        self.voxels.push(new_voxel);
    }

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

    pub fn set_all_palette_color(&mut self,r: u8,g: u8,b: u8,a: u8){
        for i in 0..255{
            self.palette[i as usize].r = r;
            self.palette[i as usize].g = g;
            self.palette[i as usize].b = b;
            self.palette[i as usize].a = a;
        }
    }

    pub fn add_cube(&mut self,startx: u8,starty: u8,startz: u8,endx: u8,endy: u8,endz: u8,colorindex: u8){
        for currentx in startx..endx{
            for currenty in starty..endy{
                for currentz in startz..endz{
                    self.add_voxel(Voxel::new(currentx, currenty, currentz, colorindex));
                }
            }
        }
    }

    pub fn write_voxels(&self, buf_writer: &mut std::io::BufWriter<std::fs::File>){
        for i in 0..self.voxels.len(){
            buf_writer.write(&[self.voxels[i].position.0,self.voxels[i].position.1,self.voxels[i].position.2,self.voxels[i].colorindex]).expect("failed to write voxels");
        }
    }

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

//VOXEL
pub struct Voxel{
    position: (u8, u8, u8),
    colorindex: u8
}

impl Voxel{
    fn write_to_file(&self, file: &mut std::fs::File){
        file.write(&[self.position.0,self.position.1,self.position.2, self.colorindex]).expect("failed to write to file");
    }

    pub fn new(x: u8, y: u8, z: u8,  colorindex_value: u8) -> Voxel{
        if colorindex_value == 0 {
            panic!("index needs to be between 1 and 255");
        }
        Voxel{
            position: (x, y, z),
            colorindex: colorindex_value
        }
    }
}

//COLOR
#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a
        }
    }
}

//writing functions
fn write_string_literal(inputfile: &mut std::io::BufWriter<std::fs::File>, string: &str){
    inputfile.write(string.as_bytes()).expect("failed");
}

fn write_slice(inputfile: &mut std::io::BufWriter<std::fs::File>, slice: &[u8]){
    inputfile.write(slice).expect("failed");
}

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
    use super::*;
    #[test]
    fn it_works() {
        let mut my_vox = Voxobject::new(256,256,256);
        my_vox.set_all_palette_color(255,0,0,255);
        my_vox.add_voxel(Voxel::new(0,0,0,1));
        let color1 = Color::new(255,255,0,255);
        let color2 = Color::new(0,255,255,255);
        my_vox.add_gradient(1,100,color1,color2);
        my_vox.add_cube(0,0,0,255,255,255,70);
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
        cube_vox.add_cube(25,25,25,75,75,75,255);
        cube_vox.save_as_file("red_cube.vox");
    }

    #[test]
    fn one_voxel(){
        let mut voxel_vox = Voxobject::new(20,20,20);
        voxel_vox.set_palette_color(255,255,0,0,255);
        voxel_vox.add_voxel(Voxel::new(0,0,0,255));
        voxel_vox.save_as_file("voxel.vox");
    }
}
