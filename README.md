# Creating vox files in Rust
A Rust library for easily creating and loading magicavoxel files. Includes all functionality for creating and reading a simple vox file.

## Example
creating a file:
```rust
//creates a vox file with a red cube in the center.
let mut cube_vox = Voxfile::new(100,100,100);
cube_vox.set_palette_color(255,255,0,0,255);
cube_vox.models[0].add_cube(25,25,25,75,75,75,255).unwrap();
cube_vox.save("red_cube.vox");
```

loading a file:
```rust
//opens a file and then auto scales it
let mut new_vox = VoxFile::load("my_vox.vox");
new_vox.models[0].auto_size();
new_vox.savee("new_vox.vox");
```
