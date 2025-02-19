//! The selected code defines attributes for the `Vertex` struct in Rust:
//!
//! ```rust
//! #[repr(C)]
//! ```
//! This attribute ensures that the struct has a C-compatible memory layout, which is important for interoperability with C code or certain binary formats.
//!
//! ```rust
//! #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
//! ```
//! This attribute automatically implements several traits for the `Vertex` struct:
//! - `Copy` and `Clone` allow for bitwise copying of the struct.
//! - `Debug` enables formatting the struct using the `{:?}` formatter.
//! - `bytemuck::Pod` and `bytemuck::Zeroable` are traits from the `bytemuck` crate that ensure the struct can be safely converted to and from byte slices and can be zero-initialized, respectively.
//!
//! > If your struct includes types that don't implement Pod and Zeroable, you'll need to implement these traits manually. These traits don't require us to implement any methods, so we just need to use the following to get our code to work.
//!
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: [f32; 3],
}
