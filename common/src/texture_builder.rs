/// A macro to load an image file at compile time and extract its RGBA data and dimensions.
///
/// # Parameters
/// - `$filename`: The path to the image file to be embedded and processed.
///
/// # Returns
/// A tuple containing:
/// - `image::RgbaImage`: The RGBA image data.
/// - `(u32, u32)`: The width and height of the image in pixels.
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

/// Creates a bind group and its corresponding bind group layout for a texture.
///
/// # Parameters
/// - `device`: A reference to the `wgpu::Device` used to create GPU resources.
/// - `queue`: A reference to the `wgpu::Queue` used to submit commands to the GPU.
/// - `texture_label`: An optional label for the texture, useful for debugging.
/// - `dimensions`: A tuple containing the width and height of the texture in pixels.
/// - `rgba_image`: A reference to the RGBA image data to be uploaded to the texture.
/// - `texture_bind_group_layout_label`: An optional label for the bind group layout, useful for debugging.
/// - `bind_group_label`: An optional label for the bind group, useful for debugging.
///
/// # Returns
/// A tuple containing:
/// - `BindGroup`: The bind group that can be used in shaders.
/// - `BindGroupLayout`: The layout of the bind group, describing its structure.
pub fn create_bind_group_and_layout(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_label: Option<&str>,
    dimensions: (u32, u32),
    rgba_image: &image::RgbaImage,
    texture_bind_group_layout_label: Option<&str>,
    bind_group_label: Option<&str>,
) -> (BindGroup, BindGroupLayout) {
    let texture_size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        // To determine the depth of your texture, you need to know the number of layers in your
        // 3D texture. If you are working with a 3D texture file, the depth is typically specified in the file's metadata or format.
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: texture_label,
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        // Most images are stored using sRGB, so we need to reflect that here.
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
        // COPY_DST means that we want to copy data to this texture
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        // This is the same as with the SurfaceConfig. It
        // specifies what texture formats can be used to
        // create TextureViews for this texture. The base
        // texture format (Rgba8UnormSrgb in this case) is
        // always supported. Note that using a different
        // texture format is not supported on the WebGL2
        // backend.
        view_formats: &[],
    });

    queue.write_texture(
        // Tells wgpu where to copy the pixel data
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // The pixel data to copy. This is a slice of bytes
        &rgba_image,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            // Defines the number of bytes in a single row of the texture.
            // Since the texture is in RGBA8 format (4 bytes per pixel), this is calculated
            // as `4 * dimensions.0` (width of the texture in pixels).
            bytes_per_row: Some(4 * dimensions.0),
            //  Specifies the number of rows in the texture. This is set to the height of the texture
            rows_per_image: Some(dimensions.1),
        },
        texture_size,
    );

    // Create a texture view for the texture
    let (texture_view, sampler) = generate_default_texture_view_and_sampler(texture, device);
    // Create a bind group layout
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: texture_bind_group_layout_label,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

    // Create a bind group
    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: bind_group_label,
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });
    (texture_bind_group, texture_bind_group_layout)
}

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
