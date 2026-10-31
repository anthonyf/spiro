use macroquad::prelude::*;
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, Default)]
struct Arm {
    length: f32,
    speed: i32,
    phase: f32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn pen_points(arms: &[Arm], time: f32) -> Vec<Point> {
    let mut points = Vec::new();
    let mut x = 0.0;
    let mut y = 0.0;

    for arm in arms {
        let angle = arm.phase + arm.speed as f32 * time;
        x += arm.length * angle.cos();
        y += arm.length * angle.sin();
        points.push(Point { x, y });
    }

    points
}

fn pen_point(arms: &[Arm], time: f32) -> Point {
    pen_points(arms, time)
        .pop()
        .unwrap_or(Point { x: 0.0, y: 0.0 })
}

fn _classic() -> Vec<Arm> {
    vec![
        Arm {
            length: 60.0,
            speed: 3,
            ..Default::default()
        },
        Arm {
            length: 28.8,
            speed: -5,
            ..Default::default()
        },
    ]
}

fn fractal(
    start_length: f32,
    start_speed: i32,
    next_length_multiplyer: f32,
    next_speed_multiplyer: i32,
    iterations: u32,
) -> Vec<Arm> {
    let mut arms = Vec::new();
    let mut length = start_length;
    let mut speed = start_speed;

    for _ in 0..iterations {
        arms.push(Arm {
            length,
            speed,
            ..Default::default()
        });
        length *= next_length_multiplyer;
        speed *= next_speed_multiplyer;
    }

    arms
}

fn draw_spirograph(ring_radius: f32, arms: &[Arm], time: f32, scale: f32) {
    let draw_seconds = 10.0;

    // TAU / gcd(all speeds)
    let period = TAU
        / arms
            .iter()
            .map(|a| a.speed.unsigned_abs())
            .reduce(gcd)
            .unwrap_or(1) as f32;

    // distance (pixels) ≈ period × (sum of length × |speed|) × scale
    // steps             ≈ distance / 2        (about 2 pixels per segment)
    let distance = (period
        * arms
            .iter()
            .map(|a| a.length.abs() * a.speed.unsigned_abs() as f32)
            .sum::<f32>()
        * scale) as u32;

    let steps = (distance / 2).max(1);

    let thickness = 3.0 / scale;

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
            BLACK,
        );

        prev_point = pen_point;
    }

    // draw the individual arms, actual size
    let points = pen_points(arms, period * progress);
    let mut prev_point = Point { x: 0.0, y: 0.0 };
    let mut prev_radius = ring_radius;
    draw_circle_lines(prev_point.x, prev_point.y, ring_radius, thickness, BLUE);

    for (arm, point) in arms.iter().zip(points.iter()) {
        // draw the wheel
        let wheel_radius = (arm.length - prev_radius).abs();
        draw_circle_lines(point.x, point.y, wheel_radius, thickness, RED);

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
        prev_radius = wheel_radius;
    }
}

#[macroquad::main("Spirograph")]
async fn main() {
    let arms = fractal(60.0, -3, 1.0 / 3.0, -4, 7);

    let mut time = 0.0;

    let reach = arms.iter().map(|c| c.length.abs()).sum::<f32>();

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

        clear_background(WHITE);

        draw_spirograph(45.0, &arms, time, scale);

        next_frame().await
    }
}
