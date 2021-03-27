# Creating vox files in rust
A Rust library for easily creating vox files. Includes all functionality for creating a simple vox file.

## Example
creates a vox file with a red cube in the center.
```rust
let mut voxel_vox = Voxobject::new(20,20,20);
voxel_vox.set_palette_color(255,255,0,0,255);
voxel_vox.add_voxel(Voxel::new(0,0,0,255));
voxel_vox.save_as_file("voxel.vox");
```
