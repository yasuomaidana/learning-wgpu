pub struct Camera {
    pub(crate) eye: cgmath::Point3<f32>, // camera position
    pub(crate) target: cgmath::Point3<f32>, // look at this position
    pub(crate) up: cgmath::Vector3<f32>, // up direction
    pub(crate) aspect: f32, // aspect ratio of the window width/height
    pub(crate) fovy: f32, // field of view in the y direction
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1. The view matrix moves the world to be at the position and rotation of the camera. 
        // It's essentially an inverse of whatever the transform matrix of the camera would be.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2. The proj matrix warps the scene to give the effect of depth. 
        // Without this, objects up close would be the same size as objects far away.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        // 3. The coordinate system in WGPU is based on DirectX and Metal's coordinate systems.
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);