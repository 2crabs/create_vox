use crate::{Color, VoxFile};

impl VoxFile {
    /// sets the color of a index on the palette. The index used can not be 0.
    ///
    /// # Example
    /// ```
    /// use create_vox::VoxFile;
    ///
    /// let mut vox = VoxFile::new(10, 10, 10);
    ///
    /// //sets index 1 to be red
    /// vox.set_palette_color(1, 255, 0, 0, 255);
    /// //sets index 5 to be white
    /// vox.set_palette_color(5, 255, 255, 255, 255);
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

    pub fn reset_palette(&mut self) {
        self.palette = [Color {
            r: 75,
            g: 75,
            b: 75,
            a: 255,
        }; 256];
    }

    pub fn get_palette_color(&self, index: u8) -> Color {
        if index == 0 {
            panic!("index needs to be between 1 and 255");
        }
        let new_index = index - 1;

        self.palette[new_index as usize]
    }

    /// Like set_palette_color() but sets the color of all indexes on palette
    ///
    /// # Example
    /// ```
    /// use create_vox::VoxFile;
    ///
    /// let mut vox = VoxFile::new(10, 10, 10);
    /// //sets all indexes on the palette to be red.
    /// vox.set_all_palette_color(255, 0, 0, 255);
    /// ```
    pub fn set_all_palette_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for i in 0..255 {
            self.palette[i as usize].r = r;
            self.palette[i as usize].g = g;
            self.palette[i as usize].b = b;
            self.palette[i as usize].a = a;
        }
    }
}

fn get_middle(a: u8, b: u8, point_between: f32) -> u8 {
    ((((b as i16) - (a as i16)) as f32 * point_between) + a as f32) as u8
}
