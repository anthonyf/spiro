use macroquad::prelude::*;
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f32,
    phase: f32,
    speed: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn pen_point(circles: &[Circle], time: f32) -> Point {
    let mut x = 0.0;
    let mut y = 0.0;

    for circle in circles {
        let angle = circle.phase + circle.speed as f32 * time;
        x += circle.radius * angle.cos();
        y += circle.radius * angle.sin();
    }

    Point { x, y }
}

fn draw_spirograph(circles: &[Circle], time: f32) {
    let scale = 20.0; // this should be a parameter or use a camera to scale things
    let draw_seconds = 3.0;
    let steps = 1000;

    let mut prev_point = pen_point(circles, 0.0);

    // TAU / gcd(all speeds)
    let period = TAU
        / circles
            .iter()
            .map(|c| c.speed.unsigned_abs())
            .reduce(gcd)
            .unwrap_or(1) as f32;

    for t in (0..=steps).map(|i| i as f32 / steps as f32 * period) {
        let progress = (time / draw_seconds).min(1.0);
        let pen_point = pen_point(circles, t * progress);

        draw_line(
            prev_point.x * scale + screen_width() / 2.0,
            prev_point.y * scale + screen_height() / 2.0,
            pen_point.x * scale + screen_width() / 2.0,
            pen_point.y * scale + screen_height() / 2.0,
            2.0,
            BLACK,
        );

        prev_point = pen_point;
    }
}

#[macroquad::main("Spirograph")]
async fn main() {
    let circles = vec![
        Circle {
            radius: 1.0,
            phase: 0.0,
            speed: 1,
        },
        Circle {
            radius: 2.0,
            phase: 1.0,
            speed: 2,
        },
        Circle {
            radius: 3.0,
            phase: 2.0,
            speed: 3,
        },
    ];

    let mut time = 0.0;

    loop {
        time += get_frame_time();

        clear_background(WHITE);

        draw_spirograph(&circles, time);

        next_frame().await
    }
}
