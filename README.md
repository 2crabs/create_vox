# Creating vox files in rust
A Rust library for easily creating magicavoxel files. Includes all functionality for creating a simple vox file.

## Example
creates a vox file with a red cube in the center.
```rust
let mut cube_vox = Voxobject::new(100,100,100);
cube_vox.set_palette_color(255,255,0,0,255);
cube_vox.add_cube(25,25,25,75,75,75,255).unwrap();
cube_vox.save_as_file("red_cube.vox");
```
