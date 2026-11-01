mod document;
mod spirograph;

#[cfg(test)]
mod test_utils;

use document::*;
use macroquad::prelude::*;
use spirograph::*;

#[derive(Debug, Clone, Copy)]
enum DisplayMode {
    _Normal,
    Debug,
}

fn draw_spirograph(
    spiro: &Spirograph,
    pen: &Pen,
    display_mode: DisplayMode,
    time: f32,
    scale: f32,
) {
    let Spirograph { ring_radius, arms } = spiro;
    let draw_seconds = 10.0;
    let steps = ((spiro.distance() * scale) as u32 / 2).max(1);
    let period = spiro.period();
    let thickness = pen.thickness / scale;

    let mut prev_point = pen_point(arms, 0.0);

    let progress = (time / draw_seconds).min(1.0);
    for t in (0..=steps).map(|i| i as f32 / steps as f32 * period) {
        let pen_point = pen_point(arms, t * progress);

        draw_line(
            prev_point.x,
            prev_point.y,
            pen_point.x,
            pen_point.y,
            thickness,
            pen.color,
        );

        prev_point = pen_point;
    }

    if let DisplayMode::Debug = display_mode {
        // draw the individual arms, actual size
        let points = pen_points(arms, period * progress);
        let mut prev_point = Point { x: 0.0, y: 0.0 };
        draw_circle_lines(prev_point.x, prev_point.y, *ring_radius, thickness, BLUE);

        let wheel_sizes = spiro.wheel_sizes();

        for (point, wheel_radius) in points.iter().zip(wheel_sizes.iter()) {
            // draw the wheel
            draw_circle_lines(point.x, point.y, *wheel_radius, thickness, RED);

            // draw the arm
            draw_line(
                prev_point.x,
                prev_point.y,
                point.x,
                point.y,
                thickness,
                GREEN,
            );
            prev_point = *point;
        }
    }
}

#[macroquad::main("spirograph")]
async fn main() {
    //let doc = _classic();
    let doc = _toy(96, 36, 28.8);
    // let doc = fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE);

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

        let room = screen_width().min(screen_height()) / 2.0;
        let margin = room * 0.1;

        let scale = (room - margin) / reach;

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
