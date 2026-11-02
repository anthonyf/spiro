use std::f32::consts::TAU;
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Arm {
    pub length: f32,
    pub speed: i32,
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub struct Spirograph {
    pub ring_radius: f32,
    pub arms: Vec<Arm>,
}

impl Spirograph {
    pub fn reach(&self) -> f32 {
        self.ring_radius
            .max(self.arms.iter().map(|a| a.length.abs()).sum::<f32>())
    }

    pub fn period(&self) -> f32 {
        TAU / self
            .arms
            .iter()
            .map(|a| a.speed.unsigned_abs())
            .reduce(gcd)
            .unwrap_or(1) as f32
    }

    pub fn distance(&self) -> f32 {
        self.period()
            * self
                .arms
                .iter()
                .map(|a| a.length.abs() * a.speed.unsigned_abs() as f32)
                .sum::<f32>()
    }

    pub fn wheel_sizes(&self) -> Vec<f32> {
        let mut sizes = Vec::new();
        let mut prev_radius = self.ring_radius;

        for arm in &self.arms {
            let wheel_radius = (arm.length - prev_radius).abs();
            sizes.push(wheel_radius);
            prev_radius = wheel_radius;
        }

        sizes
    }

    /// How fast each wheel spins as it rolls, without skidding, on the one
    /// before it. The first wheel rolls on the ring, which stays still.
    ///
    /// Where two wheels touch, the point has to move at the same speed on
    /// both. So a wheel inside the one before it spins against its trip
    /// round, and a wheel outside spins with it.
    pub fn wheel_spins(&self) -> Vec<f32> {
        let mut spins = Vec::new();
        let mut prev_radius = self.ring_radius;
        let mut prev_spin = 0.0;

        for (arm, radius) in self.arms.iter().zip(self.wheel_sizes()) {
            // How fast the wheel's centre moves, and how fast the edge of
            // the wheel before it moves, both sideways to the arm.
            let centre_speed = arm.speed as f32 * arm.length;
            let edge_speed = prev_spin * prev_radius;

            let spin = if radius == 0.0 {
                0.0
            } else if arm.length < prev_radius {
                (edge_speed - centre_speed) / radius
            } else {
                (centre_speed - edge_speed) / radius
            };

            spins.push(spin);
            prev_radius = radius;
            prev_spin = spin;
        }

        spins
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn pen_points(arms: &[Arm], time: f32) -> Vec<Point> {
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

pub fn pen_point(arms: &[Arm], time: f32) -> Point {
    pen_points(arms, time)
        .pop()
        .unwrap_or(Point { x: 0.0, y: 0.0 })
}

pub fn max_reach(spirographs: &[Spirograph]) -> f32 {
    spirographs
        .iter()
        .map(|spiro| spiro.reach())
        .reduce(f32::max)
        .unwrap_or(1.0)
}

/// A known classic spirograph, used by tests and the `_classic` example.
#[allow(dead_code)]
pub fn classic() -> Spirograph {
    let ring_radius = 96.0;
    let arms = vec![
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
    ];
    Spirograph { ring_radius, arms }
}

pub fn fractal(
    length: f32,
    speed: i32,
    length_multiplier: f32,
    speed_multiplier: i32,
    iterations: u32,
) -> Spirograph {
    let mut arms = Vec::new();
    let mut current_length = length;
    let mut current_speed = speed;

    for _ in 0..iterations {
        arms.push(Arm {
            length: current_length,
            speed: current_speed,
            ..Default::default()
        });
        current_length *= length_multiplier;
        current_speed *= speed_multiplier;
    }
    let ring_radius = length / (1.0 + length_multiplier);

    Spirograph { ring_radius, arms }
}

pub fn toy(ring: u32, wheel: u32, hole: f32) -> Result<Spirograph, SpirographError> {
    if wheel == 0 {
        return Err(SpirographError::WheelIsZero);
    }

    if wheel >= ring {
        return Err(SpirographError::WheelTooBig { wheel, ring });
    }

    let ring_radius = ring as f32;
    let wheel_radius = wheel as f32;
    let g = gcd(ring, wheel);

    Ok(Spirograph {
        ring_radius,
        arms: vec![
            Arm {
                length: ring_radius - wheel_radius,
                speed: (wheel / g) as i32,
                ..Default::default()
            },
            Arm {
                length: hole,
                speed: -(ring as i32 - wheel as i32) / g as i32,
                ..Default::default()
            },
        ],
    })
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpirographError {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_close;

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

    #[test]
    fn test_fractal_ring() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7);
        assert_close!(spiro.ring_radius, 45.0);
    }

    #[test]
    fn test_reach_classic() {
        let spiro = classic();
        assert_close!(spiro.reach(), 96.0);
    }

    #[test]
    fn test_reach_fractal() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7);
        assert_close!(spiro.reach(), 90.0 * (1.0 - (1.0_f32 / 3.0).powi(7)));
    }

    #[test]
    fn test_wheel_sizes_outside_case() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7);
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 15.0);
        assert_close!(wheel_sizes[1], 5.0);
        assert_close!(wheel_sizes[2], 5.0 / 3.0);
    }

    #[test]
    fn test_wheel_sizes_inside_case() {
        let spiro = classic();
        let wheel_sizes = spiro.wheel_sizes();
        assert_eq!(wheel_sizes.len(), spiro.arms.len());
        assert_close!(wheel_sizes[0], 36.0);
        assert_close!(wheel_sizes[1], 7.2);
    }

    #[test]
    fn test_wheel_spins_inside_case() {
        // A 36 wheel rolling inside a 96 ring spins backwards, at exactly
        // the speed of the toy's pen arm.
        let spiro = toy(96, 36, 28.8).unwrap();
        let spins = spiro.wheel_spins();
        assert_eq!(spins.len(), spiro.arms.len());
        assert_close!(spins[0], -5.0);
        assert_close!(spins[0], spiro.arms[1].speed as f32);
    }

    #[test]
    fn test_wheel_spins_outside_case() {
        // Rolling round the outside, a wheel spins with its trip round:
        // the opposite way to the fractal's next arm (+12).
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7);
        let spins = spiro.wheel_spins();
        assert_close!(spins[0], -12.0);
        assert_close!(spins[1], 84.0);
    }

    #[test]
    fn test_wheel_spins_zero_wheel() {
        let spiro = Spirograph {
            ring_radius: 10.0,
            arms: vec![Arm {
                length: 10.0,
                speed: 1,
                ..Default::default()
            }],
        };
        assert_eq!(spiro.wheel_spins(), vec![0.0]);
    }

    #[test]
    fn test_toy_vs_classic_equivalence() {
        let toy_spiro = toy(96, 36, 28.8).unwrap();
        let classic_spiro = classic();

        assert_close!(toy_spiro.ring_radius, classic_spiro.ring_radius);
        assert_eq!(toy_spiro.arms.len(), classic_spiro.arms.len());

        for (toy_arm, classic_arm) in toy_spiro.arms.iter().zip(classic_spiro.arms.iter()) {
            assert_close!(toy_arm.length, classic_arm.length);
            assert_eq!(toy_arm.speed, classic_arm.speed);
        }
    }
}
