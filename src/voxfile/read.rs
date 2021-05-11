use crate::layer::Layer;
use crate::model::Model;
use crate::riff;
use crate::riff::{num_of_chunks, LAYR};
use crate::voxfile::VoxFile;
use crate::Color;
use std::fs::File;
use std::io::Read;

impl VoxFile {
    pub fn load(path: &str) -> VoxFile {
        //read file
        let mut file = File::open(path).expect("failed to open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("failed to read contents");

        //gets models
        let num_of_models = riff::num_of_chunks(&contents, String::from("SIZE"));
        let mut models = Vec::new();

        for i in 1..(num_of_models + 1) {
            let mut chunk_pos = riff::find_chunk(&contents, String::from("SIZE"), i)
                .expect("could not find SIZE chunk") as i32;
            models.push(Model::read(&contents, &mut chunk_pos, i - 1));
        }

        //palette
        let mut palette: [Color; 256] = [Color::new(0, 0, 0, 0); 256];
        let current_pos = riff::find_chunk(&contents, String::from("RGBA"), 1).unwrap() + 12;
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
            let mut chunk_pos = riff::find_chunk(&contents, String::from("LAYR"), i)
                .expect("could not find SIZE chunk") as i32;
            layers.push(Layer::from_chunk(LAYR::read(&contents, &mut chunk_pos)));
        }

        let mut voxfile = VoxFile {
            models,
            palette,
            root_node: riff::nodes_from_chunks(&contents),
            layers,
            copies: vec![]
        };

        voxfile.get_node_data();

        voxfile
    }
}
