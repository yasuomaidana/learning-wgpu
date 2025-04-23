use crate::vertex_layout::vertex::Vertex;

pub const HEX_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.1961161, -0.1961161, 0.0],
        tex_coords: [0.4131759, 0.99240386],
    },
    Vertex {
        position: [-0.0980581, 0.3134085, 0.0],
        tex_coords: [0.0048659444, 0.56958647],
    },
    Vertex {
        position: [-0.6864065, 0.3134085, 0.0],
        tex_coords: [0.28081453, 0.05060294],
    },
    Vertex {
        position: [-0.9805807, -0.1961161, 0.0],
        tex_coords: [0.85967, 0.1526709],
    },
    Vertex {
        position: [-0.6864065, -0.7056408, 0.0],
        tex_coords: [0.9414737, 0.7347359],
    },
    Vertex {
        position: [-0.0980581, -0.7056408, 0.0],
        tex_coords: [0.9414737, 0.99240386],
    },
];

pub const HEX_INDICES: &[u16] = &[2, 5, 0, 4, 2, 3, 1, 2, 0, 2, 4, 5];
