use pollster::FutureExt;
use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, PresentMode, Queue, Surface, SurfaceCapabilities};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    blue: f64,
    window: Arc<Window>,
}

impl<'a> State<'a> {
    pub fn new(window: Window) -> State<'a> {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        // Instance is used to create surfaces and adapters
        let instance = Self::create_gpu_instance();

        // The surface is the "window" that we will render the
        // graphics to
        // It is the part of the window that we draw to.
        let surface = instance
            .create_surface(window_arc.clone())
            .expect("Failed to create surface");

        // Adapter is a handle for our actual graphics card
        let adapter = Self::create_adapter(instance, &surface);

        // Device is the handle to the GPU. Responsible for the creation of most rendering and compute resources.
        // Queue is the handle to the command queue. Responsible for submitting commands to the GPU.
        let (device, queue) = Self::create_device(&adapter);

        // SurfaceCapabilities are the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            blue: 0.0,
            window: window_arc,
        }
    }

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                // power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            // Look for wasm compatible adapter we shouldn't use block_on
            .block_on()
            .unwrap()
    }

    // Here we can add the WASM specific code
    fn create_gpu_instance() -> Instance {
        Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);

        println!("Resized to {:?} from state!", new_size);
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
            println!("Rendering from state!");
            println!("Blue: {}", self.blue);
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: self.blue,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Handles input events and returns a boolean indicating whether the event has been fully processed.
    ///
    /// If the method returns `true`, the main loop won't process the event any further.
    ///
    /// # Arguments
    ///
    /// * `event` - A reference to the `WindowEvent` that needs to be processed.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the event has been fully processed, `false` otherwise.
    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            
            WindowEvent::TouchpadPressure { pressure , .. } => {
                self.blue = *pressure as f64;
                println!("Pressure: {}", pressure);
                return true;
            }
            _ => {
                
            }
        }
        false
    }
    
    pub fn update(&mut self) {
        // Update the state of the application
        self.render().unwrap();
    }
}
