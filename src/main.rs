mod document;
mod lexer;
mod parser;
mod render;
mod spirograph;

#[cfg(test)]
mod test_utils;

use document::_toy;
use macroquad::prelude::*;
use render::{DisplayMode, draw_spirograph, scale_to_fit};
use spirograph::max_reach;

#[macroquad::main("spirograph")]
async fn main() {
    // let doc = _classic();
    let doc = _toy(96, 36, 28.8);
    // let doc = _fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE);

    let spiros = match doc.spirographs() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error evaluating spirographs: {e}");
            return;
        }
    };

    let reach = max_reach(&spiros);

    let mut time = 0.0;

    loop {
        time += get_frame_time();

        let scale = scale_to_fit(reach, screen_width(), screen_height());

        let camera = Camera2D {
            zoom: vec2(scale / screen_width() * 2.0, scale / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(doc.background_color);

        for (spiro, layer) in spiros.iter().zip(doc.layers.iter()) {
            draw_spirograph(spiro, &layer.pen, DisplayMode::Debug, time, scale);
        }

        next_frame().await
    }
}
