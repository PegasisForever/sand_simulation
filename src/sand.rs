use macroquad::prelude::*;

struct SandRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    x: f32,
    y: f32,
}

impl SandRenderer {
    pub fn new(radius: f32, color: Color) -> Self {
        let sides = 16u8;
        let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
        let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);

        vertices.push(Vertex::new(0., 0., 0., 0., 0., color));
        for i in 0..sides + 1 {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2.).cos();
            let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2.).sin();
            let vertex = Vertex::new(radius * rx, radius * ry, 0., rx, ry, color);
            vertices.push(vertex);

            if i != sides {
                indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }

        Self {
            vertices,
            indices,
            x: 0f32,
            y: 0f32,
        }
    }

    pub fn prepare_draw(&mut self, x: f32, y: f32) {
        let x_diff = x - self.x;
        let y_diff = y - self.y;
        self.vertices.iter_mut().for_each(|vertex| {
            vertex.pos[0] += x_diff;
            vertex.pos[1] += y_diff;
        });

        self.x = x;
        self.y = y;
    }

    pub fn draw(&mut self, gl: &mut QuadGl) {
        gl.geometry(&self.vertices, &self.indices);
    }
}

pub struct Sand {
    renderer: SandRenderer,
    pub x: f32,
    pub y: f32,
}

impl Sand {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            renderer: SandRenderer::new(4f32, GREEN),
            x,
            y,
        }
    }

    pub fn prepare_draw(&mut self) {
        self.renderer.prepare_draw(self.x, self.y);
    }

    pub fn draw(&mut self, gl: &mut QuadGl) {
        self.renderer.draw(gl);
    }
}
