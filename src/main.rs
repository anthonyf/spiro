use macroquad::prelude::*;
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
struct Document {
    background_color: Color,
    layers: Vec<Layer>,
}

impl Document {
    fn spirographs(&self) -> Vec<Spirograph> {
        self.layers
            .iter()
            .map(|layer| layer.shape.to_spirograph())
            .collect()
    }

    fn max_reach(&self) -> f32 {
        self.spirographs()
            .iter()
            .map(|spiro| spiro.reach())
            .fold(0.0, f32::max)
    }
}

#[derive(Debug, Clone, Copy)]
enum DisplayMode {
    _Normal,
    Debug,
}

#[derive(Debug, Clone)]
struct Layer {
    pen: Pen,
    shape: Shape,
}

#[derive(Debug, Clone)]
struct Pen {
    color: Color,
    thickness: f32,
}

#[derive(Debug, Clone)]
enum Shape {
    Classic {
        ring: f32,
        arms: Vec<Arm>,
    },
    Fractal {
        length: f32,
        speed: i32,
        length_multiplyer: f32,
        speed_multiplyer: i32,
        iterations: u32,
    },
    Toy {
        ring: u32,
        wheel: u32,
        hole: f32,
    },
}

impl Shape {
    fn to_spirograph(&self) -> Spirograph {
        match self {
            Shape::Classic { ring, arms } => Spirograph {
                ring_radius: *ring,
                arms: arms.clone(),
            },
            Shape::Fractal {
                length,
                speed,
                length_multiplyer,
                speed_multiplyer,
                iterations,
            } => {
                let mut arms = Vec::new();
                let mut current_length = *length;
                let mut current_speed = *speed;

                for _ in 0..*iterations {
                    arms.push(Arm {
                        length: current_length,
                        speed: current_speed,
                        ..Default::default()
                    });
                    current_length *= *length_multiplyer;
                    current_speed *= *speed_multiplyer;
                }
                let ring_radius = length / (1.0 + length_multiplyer);

                Spirograph { ring_radius, arms }
            }
            Shape::Toy { .. } => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Arm {
    length: f32,
    speed: i32,
    phase: f32,
}

#[derive(Debug, Clone)]
struct Spirograph {
    ring_radius: f32,
    arms: Vec<Arm>,
}

impl Spirograph {
    fn period(&self) -> f32 {
        TAU / self
            .arms
            .iter()
            .map(|a| a.speed.unsigned_abs())
            .reduce(gcd)
            .unwrap_or(1) as f32
    }

    fn distance(&self) -> f32 {
        self.period()
            * self
                .arms
                .iter()
                .map(|a| a.length.abs() * a.speed.unsigned_abs() as f32)
                .sum::<f32>()
    }

    fn reach(&self) -> f32 {
        self.ring_radius
            .max(self.arms.iter().map(|a| a.length.abs()).sum::<f32>())
    }

    fn wheel_sizes(&self) -> Vec<f32> {
        let mut sizes = Vec::new();
        let mut prev_radius = self.ring_radius;

        for arm in &self.arms {
            let wheel_radius = (arm.length - prev_radius).abs();
            sizes.push(wheel_radius);
            prev_radius = wheel_radius;
        }

        sizes
    }
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

fn _classic() -> Document {
    Document {
        background_color: BLACK,
        layers: vec![Layer {
            pen: Pen {
                color: BLACK,
                thickness: 3.0,
            },
            shape: Shape::Classic {
                ring: 96.0,
                arms: vec![
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
                ],
            },
        }],
    }
}

fn fractal(
    start_length: f32,
    start_speed: i32,
    next_length_multiplyer: f32,
    next_speed_multiplyer: i32,
    iterations: u32,
    background_color: Color,
) -> Document {
    Document {
        background_color,
        layers: vec![Layer {
            pen: Pen {
                color: BLACK,
                thickness: 3.0,
            },
            shape: Shape::Fractal {
                length: start_length,
                speed: start_speed,
                length_multiplyer: next_length_multiplyer,
                speed_multiplyer: next_speed_multiplyer,
                iterations,
            },
        }],
    }
}

fn draw_document(doc: &Document, time: f32, scale: f32, display_mode: DisplayMode) {
    for layer in &doc.layers {
        let spiro = layer.shape.to_spirograph();

        draw_spirograph(&spiro, &layer.pen, display_mode, time, scale);
    }
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

#[macroquad::main("Spirograph")]
async fn main() {
    let background_color = WHITE;
    let doc = fractal(60.0, -3, 1.0 / 3.0, -4, 7, background_color);

    let mut time = 0.0;

    let reach = doc.max_reach();

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

        draw_document(&doc, time, scale, DisplayMode::Debug);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_teeth_example() {
        assert_eq!(gcd(96, 36), 12);
    }

    #[test]
    fn test_gcd_fold_reduce_start_from_zero() {
        assert_eq!(gcd(0, 5), 5);
    }

    fn spiro_from_speeds(speeds: &[i32]) -> Spirograph {
        let arms = speeds
            .iter()
            .map(|&speed| Arm {
                length: 1.0,
                speed,
                ..Default::default()
            })
            .collect();
        Spirograph {
            ring_radius: 1.0,
            arms,
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    macro_rules! assert_close {
        ($a:expr, $b:expr) => {{
            let sa = stringify!($a);
            let sb = stringify!($b);
            let a = $a;
            let b = $b;
            assert!(close(a, b), "{sa} = {a} != {sb} = {b}");
        }};
    }

    #[test]
    fn test_period_basic_case() {
        let spiro = spiro_from_speeds(&[1, 2, 3]);
        assert_close!(spiro.period(), TAU);
    }

    #[test]
    fn test_period_with_gcd_isnt_one() {
        let spiro = spiro_from_speeds(&[2, 4]);
        assert_close!(spiro.period(), TAU / 2.0);
    }

    #[test]
    fn test_period_as_u32_bug() {
        let spiro = spiro_from_speeds(&[3, -1]);
        assert_close!(spiro.period(), TAU);
    }

    #[test]
    fn test_fractal_ring() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE).spirographs()[0].clone();
        assert_close!(spiro.ring_radius, 45.0);
    }

    #[test]
    fn test_reach_classic() {
        let spiro = _classic().spirographs()[0].clone();
        assert_close!(spiro.reach(), 96.0);
    }

    #[test]
    fn test_reach_fractal() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE).spirographs()[0].clone();
        assert_close!(spiro.reach(), 90.0 * (1.0 - (1.0_f32 / 3.0).powi(7)));
    }

    #[test]
    fn test_wheel_sizes_outside_case() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE).spirographs()[0].clone();
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 15.0);
        assert_close!(wheel_sizes[1], 5.0);
        assert_close!(wheel_sizes[2], 5.0 / 3.0);
    }

    #[test]
    fn test_wheel_sizes_inside_case() {
        let spiro = _classic().spirographs()[0].clone();
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 36.0);
        assert_close!(wheel_sizes[1], 7.2);
    }

    #[test]
    fn test_pen_point_basic_case() {
        let spiro = Spirograph {
            ring_radius: 1.0,
            arms: vec![
                Arm {
                    length: 1.0,
                    speed: 1,
                    phase: 0.0,
                },
                Arm {
                    length: 2.0,
                    speed: -2,
                    phase: 1.0,
                },
                Arm {
                    length: 3.0,
                    speed: -2,
                    phase: 2.0,
                },
            ],
        };
        let point = pen_point(&spiro.arms, 0.0);
        assert_close!(point.x, 0.832164);
        assert_close!(point.y, 4.4108343);
    }

    #[test]
    fn test_pen_points_with_t() {
        let spiro = Spirograph {
            ring_radius: 1.0,
            arms: vec![Arm {
                length: 1.0,
                speed: 1,
                phase: 0.0,
            }],
        };
        let points = pen_points(&spiro.arms, TAU / 4.0);
        assert_eq!(points.len(), 1);
        assert_close!(points[0].x, 0.0);
        assert_close!(points[0].y, 1.0);
    }
}
