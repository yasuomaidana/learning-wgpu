use wgpu::{Device, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline, ShaderModule};

pub fn create_pipeline_layout(device: &Device, label_name: &str) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some(label_name),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
}

pub fn create_render_pipeline(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    shader_module: &ShaderModule,
    config: &wgpu::SurfaceConfiguration,
    pipe_line_label: &str,
    vertex_entrypoint: &str,
    fragment_entrypoint: &str,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(pipe_line_label),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: Some(vertex_entrypoint),
            buffers: &[],
            // compilation_options: Default::default(),
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: Some(fragment_entrypoint),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
