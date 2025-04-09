use crate::vertex_layout::const_values::{INDICES, VERTICES};
use crate::vertex_layout::hex_values::{HEX_INDICES, HEX_VERTICES};
use crate::vertex_layout::vertex::Vertex;
use common::pipeline_builder::{create_pipeline_layout, create_render_pipeline_with_buffers};
use common::state_builder::{
    create_adapter, create_device, create_gpu_instance, create_render_pass, create_surface_config,
};
use std::sync::Arc;
use wgpu::util::DeviceExt; // Import the DeviceExt trait to use create_buffer_init
use wgpu::{Color, Device, Queue, RenderPipeline, Surface};
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

        // long way
        // let shader = device.create_shader_module(ShaderModuleDescriptor {
        //     label: Some("Shader"),
        //     source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        // });

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

        let render_pipeline_layout = create_pipeline_layout(&device, "Render Pipeline Layout");

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
