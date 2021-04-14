use std::ops::Add;

/// A single voxel.
#[derive(Clone)]
pub struct Voxel{
    pub position: (u8, u8, u8),
    pub colorindex: u8
}

impl Voxel{
    /// Creates new voxel.
    ///
    /// # Example
    /// ```
    /// use create_vox::Voxel;
    ///
    /// let voxel = Voxel::new(5,0,0,1);
    /// ```
    pub fn new(x: u8, y: u8, z: u8,  colorindex_value: u8) -> Voxel{
        if colorindex_value == 0 {
            panic!("index needs to be between 1 and 255");
        }
        Voxel{
            position: (x, y, z),
            colorindex: colorindex_value
        }
    }

    pub fn from_bytes(voxel_bytes: [u8; 4]) -> Voxel{
        if voxel_bytes[3] == 0 {
            panic!("could not parse voxel");
        }
        Voxel::new(voxel_bytes[0],voxel_bytes[1],voxel_bytes[2],voxel_bytes[3])
    }
}

impl PartialEq for Voxel{
    fn eq(&self, other: &Voxel) -> bool{
        self.position == other.position &&
            self.colorindex == other.colorindex
    }
}

impl Add for Voxel{
    type Output = Vec<Voxel>;

    fn add(self, other: Voxel) -> Vec<Voxel>{
        vec![self, other]
    }
}
