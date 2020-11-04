mod sand;

use macroquad::prelude::*;
use macroquad::get_context;
use std::time::SystemTime;
use sand::Sand;

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

    let mut sands = Vec::with_capacity(45 * 45);
    for x in 0..45 {
        for y in 0..45 {
            let mut sand = Sand::new((x * 13) as f32, (y * 8) as f32);
            sand.prepare_draw();
            sands.push(sand);
        }
    }

    loop {
        let start_time = SystemTime::now();
        clear_background(BLACK);

        for sand in &mut sands {
            sand.draw(gl);
        }

        println!("{}fps", (1000_000f64)/(start_time.elapsed().unwrap().as_micros() as f64));
        next_frame().await
    }
}
