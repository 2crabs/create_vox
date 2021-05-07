use crate::voxfile::VoxFile;
use std::fs::File;
use std::io::BufWriter;
use crate::writing::{write_slice, write_string_literal};
use crate::riff::write_chunk;


impl VoxFile{
    pub fn write(&mut self, path: &str){
        //setups nodes for all children
        self.make_nodes();

        let file = File::create(path).expect("failed to create file");
        let mut writer = BufWriter::new(file);
        write_string_literal(&mut writer, "VOX ");
        write_slice(&mut writer, &[0, 0, 0, 0]);

        write_chunk("MAIN", 0, self.get_size() as u32, &mut writer);
        for model in self.models.iter(){
            model.write(&mut writer);
        }
        self.root_node.write_all(&mut writer);
        for layer in self.layers.iter(){
            layer.write(&mut writer);
        }
        write_chunk("RGBA", 1024, 0, &mut writer);
        for color in self.palette.iter() {
            write_slice(&mut writer, &[color.r, color.g, color.b, color.a])
        }
    }
}