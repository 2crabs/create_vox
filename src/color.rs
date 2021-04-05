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