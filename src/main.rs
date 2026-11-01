use macroquad::prelude::*;
use std::fmt;

mod spirograph;
use spirograph::*;

#[cfg(test)]
mod test_utils;

#[derive(Debug, Clone)]
struct Document {
    background_color: Color,
    layers: Vec<Layer>,
}

impl Document {
    fn spirographs(&self) -> Result<Vec<Spirograph>, SpirographError> {
        self.layers
            .iter()
            .map(|layer| layer.shape.to_spirograph())
            .collect()
    }
}

fn max_reach(spirographs: &[Spirograph]) -> f32 {
    spirographs
        .iter()
        .map(|spiro| spiro.reach())
        .reduce(f32::max)
        .unwrap_or(1.0)
}

#[derive(Debug, Clone, PartialEq)]
enum SpirographError {
    WheelTooBig { wheel: u32, ring: u32 },
    WheelIsZero,
}

impl fmt::Display for SpirographError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpirographError::WheelTooBig { wheel, ring } => {
                write!(f, "Wheel size {wheel} is too big for ring size {ring}")
            }
            SpirographError::WheelIsZero => {
                write!(f, "Wheel size is zero")
            }
        }
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
    fn to_spirograph(&self) -> Result<Spirograph, SpirographError> {
        match self {
            Shape::Classic { ring, arms } => Ok(Spirograph {
                ring_radius: *ring,
                arms: arms.clone(),
            }),
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

                Ok(Spirograph { ring_radius, arms })
            }
            Shape::Toy { ring, wheel, hole } => {
                if *wheel == 0 {
                    return Err(SpirographError::WheelIsZero);
                }

                if *wheel >= *ring {
                    return Err(SpirographError::WheelTooBig {
                        wheel: *wheel,
                        ring: *ring,
                    });
                }

                let ring_radius = *ring as f32;
                let wheel_radius = *wheel as f32;
                let g = gcd(*ring, *wheel);

                Ok(Spirograph {
                    ring_radius,
                    arms: vec![
                        Arm {
                            length: ring_radius - wheel_radius,
                            speed: (*wheel / g) as i32,
                            ..Default::default()
                        },
                        Arm {
                            length: *hole,
                            speed: -(*ring as i32 - *wheel as i32) / g as i32,
                            ..Default::default()
                        },
                    ],
                })
            }
        }
    }
}

fn _classic() -> Document {
    Document {
        background_color: WHITE,
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

fn _toy(ring: u32, wheel: u32, hole: f32) -> Document {
    Document {
        background_color: WHITE,
        layers: vec![Layer {
            pen: Pen {
                color: BLACK,
                thickness: 3.0,
            },
            shape: Shape::Toy { ring, wheel, hole },
        }],
    }
}

fn _fractal(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_close;

    fn first_spirograph(doc: &Document) -> Spirograph {
        doc.spirographs().unwrap()[0].clone()
    }

    #[test]
    fn test_fractal_ring() {
        let spiro = first_spirograph(&_fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE));
        assert_close!(spiro.ring_radius, 45.0);
    }

    #[test]
    fn test_reach_classic() {
        let spiro = first_spirograph(&_classic());
        assert_close!(spiro.reach(), 96.0);
    }

    #[test]
    fn test_reach_fractal() {
        let spiro = first_spirograph(&_fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE));
        assert_close!(spiro.reach(), 90.0 * (1.0 - (1.0_f32 / 3.0).powi(7)));
    }

    #[test]
    fn test_wheel_sizes_outside_case() {
        let spiro = first_spirograph(&_fractal(60.0, -3, 1.0 / 3.0, -4, 7, WHITE));
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 15.0);
        assert_close!(wheel_sizes[1], 5.0);
        assert_close!(wheel_sizes[2], 5.0 / 3.0);
    }

    #[test]
    fn test_wheel_sizes_inside_case() {
        let spiro = first_spirograph(&_classic());
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 36.0);
        assert_close!(wheel_sizes[1], 7.2);
    }

    #[test]
    fn test_toy_vs_classic_equivalence() {
        let toy_doc = _toy(96, 36, 28.8);
        let classic_doc = _classic();

        let toy_spiro = first_spirograph(&toy_doc);
        let classic_spiro = first_spirograph(&classic_doc);

        assert_close!(toy_spiro.ring_radius, classic_spiro.ring_radius);
        assert_eq!(toy_spiro.arms.len(), classic_spiro.arms.len());

        for (toy_arm, classic_arm) in toy_spiro.arms.iter().zip(classic_spiro.arms.iter()) {
            assert_close!(toy_arm.length, classic_arm.length);
            assert_eq!(toy_arm.speed, classic_arm.speed);
        }
    }

    #[test]
    fn test_toy_invalid_wheel() {
        let toy_doc = _toy(96, 100, 28.8);
        let result = toy_doc.spirographs();
        assert_eq!(
            result.unwrap_err(),
            SpirographError::WheelTooBig {
                wheel: 100,
                ring: 96
            }
        );
    }

    #[test]
    fn test_toy_invalid_wheel_zero() {
        let toy_doc = _toy(96, 0, 28.8);
        let result = toy_doc.spirographs();
        assert_eq!(result.unwrap_err(), SpirographError::WheelIsZero);
    }

    #[test]
    fn test_toy_zero_zero() {
        let toy_doc = _toy(0, 0, 28.8);
        let result = toy_doc.spirographs();
        assert_eq!(result.unwrap_err(), SpirographError::WheelIsZero);
    }
}
