use common::get_rgba_image_and_dimensions;
use common::pipeline_builder::{
    create_pipeline_layout_with_bind_groups, create_render_pipeline_with_buffers,
};
use common::state_builder::{
    create_adapter, create_device, create_gpu_instance, create_render_pass, create_surface_config,
};
use common::state_traits::{DefaultAppStateMethods, DefaultResizeWindowMethods};
use common::texture_builder::create_bind_group_and_layout;
use image::GenericImageView;

use crate::camera::{Camera, CameraUniform};
use common::key_q_s_handler::qs_state_app::AppState;
use common::vertex_layout::const_values::{INDICES, VERTICES};
use common::vertex_layout::vertex::Vertex;
use state_derive::DefaultResizeWindowMethods;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Color, Device, Queue, RenderPipeline, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(DefaultResizeWindowMethods)]
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
    vertex_buffer: wgpu::Buffer,
    // Index buffer
    index_buffer: wgpu::Buffer,
    // Vertices
    // num_indices: u32,
    toggled: bool,
    diffuse_bind_group: BindGroup,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_bind_group: BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl DefaultAppStateMethods for State<'_> {
    fn update(&mut self) {
        // Update the state of the application
        self.toggled = !self.toggled;
        if let Err(e) = self.render() {
            eprintln!("Render error: {:?}", e);
        }
    }
}

impl<'a> AppState for State<'a> {
    // in the original example, it uses async, but it breaks my trait
    fn new(window: Window) -> State<'a> {
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

        // Loading textures
        let (diffuse_rgba, dimensions) =
            get_rgba_image_and_dimensions!("../../textures-and-bind-groups/src/happy-tree.png");

        // Creating bind groups and layouts
        let (diffuse_bind_group, texture_bind_group_layout) = create_bind_group_and_layout(
            &device,
            &queue,
            Some("Texture Label"),
            dimensions,
            &diffuse_rgba,
            Some("Diffuse Texture Bind Layout"),
            Some("Diffuse Bind Group"),
        );

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

        let camera = Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        // Create a buffer for the camera uniform
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a bind group layout for the camera,
        // It tells the GPU what kind of data we will be passing to the shader
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    // Visibility to vertex shader
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform, // Uniform means read-only data
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create a bind group for the camera
        // It binds the camera buffer to the bind group layout
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Creating pipelines
        let render_pipeline_layout = create_pipeline_layout_with_bind_groups(
            &device,
            "Render Pipeline Layout",
            &[
                &texture_bind_group_layout,
                // it contains the bind groups
                // since the camera's bind group layout is in the first index
                // we will set the group to 1
                &camera_bind_group_layout,
            ],
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
            vertex_buffer,
            index_buffer,
            // num_indices,
            toggled: false,
            diffuse_bind_group,
            camera,
            camera_uniform,
            camera_bind_group,
            camera_buffer,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

            println!("Toggled {}", self.toggled);
            if self.toggled {
                self.camera.eye = (2.0, 0.5, 2.0).into();
                println!("Non -Toggled eye{:?}", self.camera.eye);
            } else {
                self.camera.eye = (0.0, 1.0, 2.0).into();

                println!("Toggled eye{:?}", self.camera.eye);
            }

            self.camera_uniform.update_view_proj(&self.camera);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            let vertex_buffer = &self.vertex_buffer;
            let index_buffer = &self.index_buffer;

            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            let num_indices = INDICES.len() as u32;

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw(0..self.num_indices, 0..1);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
