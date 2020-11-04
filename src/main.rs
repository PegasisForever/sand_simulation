mod sand;
mod world;

use macroquad::prelude::*;
use macroquad::get_context;
use std::time::SystemTime;
use sand::Sand;
use std::sync::{Arc, RwLock};
use crate::world::World;
use std::ops::Deref;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand Simulation".to_owned(),
        window_width: 600,
        window_height: 400,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let gl = &mut get_context().draw_context.gl;
    gl.texture(None);
    gl.draw_mode(DrawMode::Triangles);

    let mut world = World::new(600, 400, 2);

    for x in 0..100 {
        for y in 0..100 {
            let sand = Arc::new(RwLock::new(Sand::new(
                (x * 6) as f32,
                (y * 4) as f32,
                (x + y * 1000) as usize,
            )));
            world.add_sand(sand);
        }
    }


    let mut world = Arc::new(RwLock::new(world));

    let pool = ThreadPool::new(16);

    loop {
        let start_time = SystemTime::now();
        clear_background(BLACK);

        for sand in &world.read().unwrap().sands {
            let sand = sand.clone();
            let world = world.clone();
            pool.execute(move || {
                let mut sand2 = sand.read().unwrap().deref().clone();
                sand2.update(10f32 / 16f32, world);
                std::mem::replace(&mut *sand.write().unwrap(), sand2);
            });
        }
        pool.join();

        for sand in &world.read().unwrap().sands {
            let mut sand = sand.write().unwrap();
            sand.draw(gl);
        }

        world.write().unwrap().recreate_grid();

        println!("{}fps", (1000_000f64) / (start_time.elapsed().unwrap().as_micros() as f64));
        next_frame().await
    }
}
