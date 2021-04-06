use std::ops::Add;

/// Color containing 4 bytes for red, green, blue, and alpha.
#[derive(Copy, Clone)]
pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8
}

impl Color {
    /// Creates new color.
    ///
    /// # Example
    /// ```
    /// use create_vox::Color;
    ///
    /// let yellow = Color::new(255,255,0,255);
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a
        }
    }
}

impl PartialEq for Color{
    fn eq(&self, other: &Color) -> bool{
        self.r == other.r &&
            self.g == other.g &&
            self.b == other.b &&
            self.a == other.a
    }
}

impl Add for Color{
    type Output = Color;

    fn add(self, other: Color) -> Color{
        let r = (self.r + other.r) / 2;
        let g = (self.g + other.g) / 2;
        let b = (self.b + other.b) / 2;
        let a = (self.a + other.a) / 2;

        Color::new(r,g,b,a)
    }
}