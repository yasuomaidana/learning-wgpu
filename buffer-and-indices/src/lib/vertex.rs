#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: [f32; 3],
}
 