/// Color containing 4 bytes for red, green, blue, and alpha.
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
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