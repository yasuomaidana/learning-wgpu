use wgpu::PipelineLayout;

pub fn create_pipeline_layout(device: &wgpu::Device, label_name: &str) -> PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label_name),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
}
