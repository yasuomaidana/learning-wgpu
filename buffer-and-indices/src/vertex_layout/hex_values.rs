use crate::vertex_layout::vertex::Vertex;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.1961161, -0.1961161, 0.0],
        color: [0.00, 0.50, 0.00],
    },
    Vertex {
        position: [-0.0980581, 0.3134085, 0.0],
        color: [0.00, 0.50, 0.00],
    },
    Vertex {
        position: [-0.6864065, 0.3134085, 0.0],
        color: [0.00, 0.50, 0.00],
    },
    Vertex {
        position: [-0.9805807, -0.1961161, 0.0],
        color: [0.00, 0.50, 0.00],
    },
    Vertex {
        position: [-0.6864065, -0.7056408, 0.0],
        color: [0.00, 0.50, 0.00],
    },
    Vertex {
        position: [-0.0980581, -0.7056408, 0.0],
        color: [0.00, 0.50, 0.00],
    },
];

pub const INDICES: &[u16] = &[2, 5, 0, 4, 2, 3, 1, 2, 0, 2, 4, 5];
