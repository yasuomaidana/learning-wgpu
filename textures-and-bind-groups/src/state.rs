use crate::vertex_layout::const_values::{INDICES, VERTICES};
use crate::vertex_layout::hex_values::{HEX_INDICES, HEX_VERTICES};
use crate::vertex_layout::vertex::Vertex;
use common::pipeline_builder::{
    create_pipeline_layout_with_bind_groups, create_render_pipeline_with_buffers,
};
use common::state_builder::{
    create_adapter, create_device, create_gpu_instance, create_render_pass, create_surface_config,
};
use image::GenericImageView;
use std::sync::Arc;
use wgpu::util::DeviceExt;
// Import the DeviceExt trait to use create_buffer_init
use common::get_rgba_image_and_dimensions;
use wgpu::{BindGroup, Color, Device, Queue, RenderPipeline, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    // Pipeline
    render_pipeline: RenderPipeline,
    // Vertex buffer
    vertex_buffers: Vec<wgpu::Buffer>,
    // Index buffer
    index_buffers: Vec<wgpu::Buffer>,
    // Vertices
    // num_indices: u32,
    toggled: bool,
    diffuse_bind_group: BindGroup,
}

impl<'a> State<'a> {
    pub fn new(window: Window) -> State<'a> {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        // Instance is used to create surfaces and adapters
        let instance = create_gpu_instance();

        // The surface is the "window" that we will render the
        // graphics to
        // It is the part of the window that we draw to.
        let surface = instance
            .create_surface(window_arc.clone())
            .expect("Failed to create surface");

        // Adapter is a handle for our actual graphics card
        let adapter = create_adapter(instance, &surface);

        // Device is the handle to the GPU. Responsible for the creation of most rendering and compute resources.
        // Queue is the handle to the command queue. Responsible for submitting commands to the GPU.
        let (device, queue) = create_device(&adapter);

        // SurfaceCapabilities are the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let config = create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        // Embeds the image files as a byte array at compile time, making it part of the binary
        // let diffuse_bytes = include_bytes!("happy-tree.png");
        // Decodes the byte array into an image object using the image crate
        // let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        // Converts the image object into a texture format that WGPU can use (RGBA8)
        let diffuse_rgba = diffuse_image.to_rgba8();

        let dimensions = diffuse_image.dimensions();

        // let diffuse_rgba = diffuse_image.to_rgba8();
        let (diffuse_rgba, dimensions) = get_rgba_image_and_dimensions!("happy-tree.png");
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            // To determine the depth of your texture, you need to know the number of layers in your
            // 3D texture. If you are working with a 3D texture file, the depth is typically specified in the file's metadata or format.
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Diffuse Texture"),
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
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The pixel data to copy. This is a slice of bytes
            &diffuse_rgba,
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

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
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

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Diffuse Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Hex Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Vertex buffer
        let hex_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Hex Vertex Buffer"),
            contents: bytemuck::cast_slice(&HEX_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer
        let hex_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Hex Index Buffer"),
            contents: bytemuck::cast_slice(&HEX_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let render_pipeline_layout = create_pipeline_layout_with_bind_groups(
            &device,
            "Render Pipeline Layout",
            &[&texture_bind_group_layout],
        );

        let render_pipeline = create_render_pipeline_with_buffers(
            &device,
            &render_pipeline_layout,
            &shader,
            &config,
            "Render Pipeline",
            "vs_main",
            "fs_main",
            &[Vertex::desc()],
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
            render_pipeline,
            vertex_buffers: vec![vertex_buffer, hex_vertex_buffer],
            index_buffers: vec![index_buffer, hex_index_buffer],
            // num_indices,
            toggled: false,
            diffuse_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self
            .surface
            .get_current_texture()
            .expect("Failed to get texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = create_render_pass(
                &mut encoder,
                &view,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            let vertex_buffer;
            let index_buffer;
            let num_indices;
            if !self.toggled {
                vertex_buffer = &self.vertex_buffers[0];
                index_buffer = &self.index_buffers[0];
                num_indices = INDICES.len() as u32;
            } else {
                vertex_buffer = &self.vertex_buffers[1];
                index_buffer = &self.index_buffers[1];
                num_indices = HEX_INDICES.len() as u32;
            }

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw(0..self.num_indices, 0..1);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self) {
        // Update the state of the application
        self.toggled = !self.toggled;
        self.render().unwrap();
    }
}
