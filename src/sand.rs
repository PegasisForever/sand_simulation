use macroquad::prelude::*;
use std::sync::{RwLock, Arc, Mutex};
use crate::world::World;
use std::result::Result::Ok;

#[derive(Clone)]
struct SandRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    x: f32,
    y: f32,
}

impl SandRenderer {
    pub fn new(radius: f32, color: Color) -> Self {
        let sides = 6u8;
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
    renderer: Arc<Mutex<SandRenderer>>,
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub id: usize,
}

const SAND_RADIUS: f32 = 1f32;

impl Sand {
    pub fn new(x: f32, y: f32, id: usize) -> Self {
        Self {
            renderer: Arc::new(Mutex::new(SandRenderer::new(SAND_RADIUS, GREEN))),
            x,
            y,
            delta_x: 0.0,
            delta_y: 0.0,
            id,
        }
    }

    pub fn update(&mut self, dt: f32, world: Arc<RwLock<World>>) {
        self.delta_x *= 0.99f32 * (1f32 - dt);
        self.x += self.delta_x * dt;
        self.delta_y += 1.2f32 * dt;
        self.y += self.delta_y * dt;

        let world = world.read().unwrap();
        let nearby = world.get_nearby(self.x, self.y);
        for sand in nearby {
            let sand = sand.read().unwrap();
            if sand.id != self.id {
                let x_diff = self.x - sand.x;
                let y_diff = self.y - sand.y;
                if x_diff > -SAND_RADIUS * 2f32 && x_diff < SAND_RADIUS * 2f32 &&
                    y_diff > -SAND_RADIUS * 2f32 && y_diff < SAND_RADIUS * 2f32 {
                    // todo self.x += (random.randint(0, 4) - 2) / 16
                    if x_diff < 0f32 { // self is at left of the sand
                        self.x -= SAND_RADIUS * 2f32 + x_diff;
                        self.delta_x -= (SAND_RADIUS * 2f32 + x_diff) / 2f32;
                        self.delta_y -= 0.1f32;
                    } else { // self is at right of the sand
                        self.x += SAND_RADIUS * 2f32 - x_diff;
                        self.delta_x += (SAND_RADIUS * 2f32 - x_diff) / 2f32;
                        self.delta_y -= 0.1f32;
                    }
                    if y_diff <= 0f32 { // self is on top of the sand
                        self.y -= SAND_RADIUS * 2f32 + y_diff;
                        self.delta_y = 0f32;
                    }
                }
            }
        }

        if self.y > world.height as f32 - SAND_RADIUS * 2f32 {
            self.y = world.height as f32 - SAND_RADIUS * 2f32;
        } else if self.y < SAND_RADIUS * 2f32 {
            self.y = SAND_RADIUS * 2f32;
        }
        if self.x > world.width as f32 - SAND_RADIUS * 2f32 {
            self.x = world.width as f32 - SAND_RADIUS * 2f32;
        } else if self.x < SAND_RADIUS * 2f32 {
            self.x = SAND_RADIUS * 2f32;
        }

        self.prepare_draw();
    }

    pub fn prepare_draw(&mut self) {
        self.renderer.lock().unwrap().prepare_draw(self.x, self.y);
    }

    pub fn draw(&mut self, gl: &mut QuadGl) {
        self.renderer.lock().unwrap().draw(gl);
    }

    pub fn clone(&self) -> Self {
        Self {
            renderer: self.renderer.clone(),
            x: self.x,
            y: self.y,
            delta_x: self.delta_x,
            delta_y: self.delta_y,
            id: self.id,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Sand #{} ({},{})", self.id, self.x, self.y)
    }
}
