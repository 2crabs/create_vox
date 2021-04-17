use crate::*;
use crate::convert::*;
use crate::writing::*;
use std::io::BufWriter;
use std::fs::File;
use crate::riff::write_chunk;

#[derive(Clone)]
pub struct Model{
    size: (u16, u16, u16),
    pub(crate) voxels: Vec<Voxel>
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Model{

    pub fn new(x: u16, y: u16, z: u16) -> Model{
        Model{
            size: (x, y, z),
            voxels: Vec::new(),
        }
    }
    pub fn write(&self, writer: &mut BufWriter<File>){
        let size_slice: &[u8] = &[
            u16_to_array(self.size.0)[0],
            u16_to_array(self.size.0)[1],
            0,
            0,
            u16_to_array(self.size.1)[0],
            u16_to_array(self.size.1)[1],
            0,
            0,
            u16_to_array(self.size.2)[0],
            u16_to_array(self.size.2)[1],
            0,
            0,
        ];
        write_chunk("SIZE", 12, 0, writer);
        //writes the slice for size
        write_slice(writer, size_slice);

        write_chunk("XYZI", ((self.voxels.len() as u32) * 4) + 4, 0, writer);
        //number voxels in the voxobject
        write_slice(writer, &i32_to_array(self.voxels.len() as u32));
        //writes all of the voxels
        self.write_voxels(writer);
    }

    fn write_voxels(&self, buf_writer: &mut BufWriter<File>) {
        for i in 0..self.voxels.len() {
            buf_writer
                .write(&[
                    self.voxels[i].position.0,
                    self.voxels[i].position.1,
                    self.voxels[i].position.2,
                    self.voxels[i].colorindex,
                ])
                .expect("failed to write voxels");
        }
    }
}