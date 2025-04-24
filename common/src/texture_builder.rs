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
pub fn generate_default_texture_view_and_sampler(
    texture: Texture,
    device: &wgpu::Device,
) -> (TextureView, Sampler) {
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        // u: The horizontal axis of the texture (similar to the x axis in 2D space).
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        // v: The vertical axis of the texture (similar to the y axis in 2D space).
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        // w: The depth axis, used for 3D textures or volumetric textures.
        // The value ClampToEdge means the texture will use the color of the edge pixels
        // when sampling outside the valid range.
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        // Specifies that when the texture is magnified (scaled up), linear filtering is used.
        // This blends the colors of nearby texels to produce smoother results.
        mag_filter: wgpu::FilterMode::Linear,
        // Specifies that when the texture is minified (scaled down),
        // nearest-neighbor filtering is used. This selects the nearest texel,
        // resulting in a pixelated look.
        min_filter: wgpu::FilterMode::Nearest,
        // Specifies that when mipmaps are used,
        // the nearest mipmap level is selected without blending between levels.
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    (texture_view, sampler)
}
