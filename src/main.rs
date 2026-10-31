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

fn pen_points(circles: &[Circle], time: f32) -> Vec<Point> {
    let mut points = Vec::new();
    let mut x = 0.0;
    let mut y = 0.0;

    for circle in circles {
        let angle = circle.phase + circle.speed as f32 * time;
        x += circle.radius * angle.cos();
        y += circle.radius * angle.sin();
        points.push(Point { x, y });
    }

    points
}

fn pen_point(circles: &[Circle], time: f32) -> Point {
    pen_points(circles, time)
        .pop()
        .unwrap_or(Point { x: 0.0, y: 0.0 })
}

fn draw_spirograph(circles: &[Circle], time: f32, px_per_unit: f32) {
    let draw_seconds = 3.0;
    let steps = 1000;
    let thickness = 3.0 * px_per_unit;

    let mut prev_point = pen_point(circles, 0.0);

    // TAU / gcd(all speeds)
    let period = TAU
        / circles
            .iter()
            .map(|c| c.speed.unsigned_abs())
            .reduce(gcd)
            .unwrap_or(1) as f32;

    let progress = (time / draw_seconds).min(1.0);
    for t in (0..=steps).map(|i| i as f32 / steps as f32 * period) {
        let pen_point = pen_point(circles, t * progress);

        draw_line(
            prev_point.x,
            prev_point.y,
            pen_point.x,
            pen_point.y,
            thickness,
            BLACK,
        );

        prev_point = pen_point;
    }

    // draw the individual circles, actual size
    let points = pen_points(circles, period * progress);
    let mut prev_point = Point { x: 0.0, y: 0.0 };
    for (_circle, point) in circles.iter().zip(points.iter()) {
        //draw_circle_lines(prev_point.x, prev_point.y, circle.radius, thickness, RED);
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

#[macroquad::main("Spirograph")]
async fn main() {
    let scale = 20.0;

    let camera = Camera2D {
        zoom: vec2(scale / screen_width() * 2.0, scale / screen_height() * 2.0),
        target: vec2(0.0, 0.0),
        ..Default::default()
    };

    set_camera(&camera);

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

        draw_spirograph(&circles, time, 1.0 / scale);

        next_frame().await
    }
}
