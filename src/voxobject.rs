use crate::color::*;
use crate::model::Model;

/// Holds all the information needed to create a vox file.
#[derive(Clone)]
//#[deprecated]
pub struct Voxobject {
    size: (u16, u16, u16),
    models: Vec<Model>,
    pub(crate) palette: [Color; 256],
}

impl Voxobject {
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
}

//used for gradient
fn get_middle(a: u8, b: u8, point_between: f32) -> u8 {
    ((((b as i16) - (a as i16)) as f32 * point_between) + a as f32) as u8
}
