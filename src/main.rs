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

    // for x in 0..100 {
    //     for y in 0..100 {
    //         let sand = Arc::new(RwLock::new(Sand::new(
    //             (x * 6) as f32,
    //             (y * 4) as f32,
    //             (x + y * 1000) as usize,
    //         )));
    //         world.add_sand(sand);
    //     }
    // }


    let world = Arc::new(RwLock::new(World::new(600, 400, 2)));
    let mut sand_count = 0usize;
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

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let mut world = world.write().unwrap();
            for _ in 0..=((dt * 25f32) as isize) {
                let sand = Arc::new(RwLock::new(Sand::new(
                    x,
                    y,
                    sand_count + 1,
                )));
                world.add_sand(sand);
                sand_count += 1;
            }
        }

        let count = world.read().unwrap().sands.len();
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

        draw_text(&*format!("FPS: {:.1}", (1000_000f64) / (start_time.elapsed().unwrap().as_micros() as f64)), 0.0, 0.0, 30.0, WHITE);
        draw_text(&*format!("Sand count: {}", count), 0.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
