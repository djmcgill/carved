use graphics::Vertex;

macro_rules! vert (($p:expr, $t:expr) => (
    Vertex {
        pos: [$p[0] as f32, $p[1] as f32, $p[2] as f32, 1.0],
        tex_coord: [$t[0] as f32, $t[1] as f32],
    }
));

pub const CUBE_VERTS: [Vertex; 24] = [
    // top (0, 0, 1)
    vert!([-1, -1,  1], [0, 0]),
    vert!([ 1, -1,  1], [1, 0]),
    vert!([ 1,  1,  1], [1, 1]),
    vert!([-1,  1,  1], [0, 1]),
    // bottom (0, 0, -1)
    vert!([-1,  1, -1], [1, 0]),
    vert!([ 1,  1, -1], [0, 0]),
    vert!([ 1, -1, -1], [0, 1]),
    vert!([-1, -1, -1], [1, 1]),
    // right (1, 0, 0)
    vert!([ 1, -1, -1], [0, 0]),
    vert!([ 1,  1, -1], [1, 0]),
    vert!([ 1,  1,  1], [1, 1]),
    vert!([ 1, -1,  1], [0, 1]),
    // left (-1, 0, 0)
    vert!([-1, -1,  1], [1, 0]),
    vert!([-1,  1,  1], [0, 0]),
    vert!([-1,  1, -1], [0, 1]),
    vert!([-1, -1, -1], [1, 1]),
    // front (0, 1, 0)
    vert!([ 1,  1, -1], [1, 0]),
    vert!([-1,  1, -1], [0, 0]),
    vert!([-1,  1,  1], [0, 1]),
    vert!([ 1,  1,  1], [1, 1]),
    // back (0, -1, 0)
    vert!([ 1, -1,  1], [0, 0]),
    vert!([-1, -1,  1], [1, 0]),
    vert!([-1, -1, -1], [1, 1]),
    vert!([ 1, -1, -1], [0, 1]),
];

pub const CUBE_INDICES: [u16; 36] = [
    0,  1,  2,  2,  3,  0, // top
    4,  5,  6,  6,  7,  4, // bottom
    8,  9, 10, 10, 11,  8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];
