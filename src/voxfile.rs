use crate::model::Model;
use crate::Color;
use crate::node::Node;
use crate::riff;
use std::fs::File;
use std::io::{Read, BufWriter};
use crate::writing::{write_slice, write_string_literal};
use crate::riff::{write_chunk, LAYR, num_of_chunks};

pub struct VoxFile{
    models: Vec<Model>,
    palette: [Color; 256],
    root_node: Node,
    layers: Vec<LAYR>
}

impl VoxFile{
    pub fn load(path: &str) -> VoxFile{
        //read file
        let mut file = File::open(path).expect("failed to open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("failed to read contents");

        //gets models
        let num_of_models = riff::num_of_chunks(&contents, String::from("SIZE"));
        let mut models = Vec::new();

        for i in 1..(num_of_models + 1) {
            let mut chunk_pos = riff::find_chunk(&contents, String::from("SIZE"), i).expect("could not find SIZE chunk") as i32;
            models.push(Model::read(&contents, &mut chunk_pos));
        }


        //palette
        let mut palette: [Color; 256] = [Color::new(0, 0, 0, 0); 256];
        assert_eq!(1, riff::num_of_chunks(&contents, String::from("RGBA")));
        let current_pos = riff::find_chunk(&contents, String::from("RGBA"), 1).unwrap();
        for i in 0..256 {
            //gets the color data
            let r: u8 = contents[(current_pos + (i * 4)) as usize];
            let g: u8 = contents[(current_pos + (i * 4) + 1) as usize];
            let b: u8 = contents[(current_pos + (i * 4) + 2) as usize];
            let a: u8 = contents[(current_pos + (i * 4) + 3) as usize];
            palette[i as usize] = Color::new(r, g, b, a);
        }

        let mut layers = Vec::new();
        for i in 1..(num_of_chunks(&contents, String::from("LAYR")) + 1) {
            let mut chunk_pos = riff::find_chunk(&contents, String::from("LAYR"), i).expect("could not find SIZE chunk") as i32;
            layers.push(LAYR::read(&contents, &mut chunk_pos));
        }

        VoxFile{
            models,
            palette: [Color {
                r: 75,
                g: 75,
                b: 75,
                a: 255,
            }; 256],
            root_node: riff::nodes_from_chunks(&contents),
            layers
        }
    }

    pub fn write(&mut self, path: &str){
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

    pub fn get_size(&self) -> i32{
        let mut size = 1024;
        for model in self.models.iter(){
            size += model.get_size();
        }
        for layer in self.layers.iter(){
            size += layer.get_size()
        }
        size += self.root_node.get_all_size();
        size
    }
}