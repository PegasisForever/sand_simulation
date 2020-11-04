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


    let world = Arc::new(RwLock::new(world));

    let pool = ThreadPool::new(16);

    let mut last_start_time: Option<SystemTime> = None;
    loop {
        let start_time = SystemTime::now();
        let dt = if let Some(last_start_time) = last_start_time {
            start_time.duration_since(last_start_time).unwrap().as_millis() as f32 * 0.01f32
        } else {
            1f32 / 6f32
        };
        last_start_time = Some(start_time.clone());

        clear_background(BLACK);

        for sand in &world.read().unwrap().sands {
            let sand = sand.clone();
            let world = world.clone();
            pool.execute(move || {
                let mut sand2 = sand.read().unwrap().deref().clone();
                sand2.update(dt, world);
                *sand.write().unwrap() = sand2;
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
