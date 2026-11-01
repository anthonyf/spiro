use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, Default)]
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
}
