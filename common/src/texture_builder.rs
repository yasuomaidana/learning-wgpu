#[macro_export]
macro_rules! get_rgba_image_and_dimensions {
    ($filename:expr) => {{
        // Embeds the image files as a byte array at compile time, making it part of the binary
        let bytes = include_bytes!($filename);
        // Decodes the byte array into an image object using the image crate
        let image = image::load_from_memory(bytes).expect("Failed to load image");
        // Converts the image object into a texture format that WGPU can use (RGBA8)
        (image.to_rgba8(), image.dimensions())
    }};
}

// pub fn create_bind_group() -> BindGroup{
//     // Create a bind group with the specified layout and values
//     let bind_group = BindGroup {
//         layout: &wgpu::BindGroupLayout::default(),
//         entries: &[],
//         label: None,
//     };
//     bind_group
// }
