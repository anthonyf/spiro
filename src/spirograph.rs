use std::f32::consts::TAU;
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Arm {
    pub length: f32,
    pub speed: i32,
    pub phase: f32,
}

impl Arm {
    /// Which way the arm points at time `t`.
    pub fn angle(&self, t: f32) -> f32 {
        self.phase + self.speed as f32 * t
    }
}

/// A wheel on the end of an arm, and the gear it rolls on. The gear sits at
/// the start of the arm, fixed to the arm before it, and the first gear is
/// the ring. The next arm is a spoke of the wheel, so it turns with it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wheel {
    pub radius: f32,
    pub gear_radius: f32,
}

/// A list of arms, plus the gear train that drives them. The fields are
/// private, so the only way to build one is `new`, which checks the gears
/// can be made.
#[derive(Debug, Clone)]
pub struct Spirograph {
    arms: Vec<Arm>,
    wheels: Vec<Wheel>,
}

impl Spirograph {
    /// Builds a spirograph from its arms, and works out its gears.
    ///
    /// Every arm but the last carries a wheel at its end. The wheel rolls on
    /// a gear at the start of its arm, and that gear is fixed to the arm
    /// before (the first gear is the ring, which stays still). The next arm
    /// is a spoke of the wheel. That makes a real gear train: turning the
    /// first arm drives all the others, as in a toy.
    ///
    /// Seen from its gear, a wheel's centre goes round at `turn` and the
    /// wheel spins at `spin`. Rolling without skidding fixes the sizes:
    /// wheel = length × turn / spin, and gear = length × (spin − turn) / spin.
    pub fn new(arms: Vec<Arm>) -> Result<Self, SpirographError> {
        if arms.is_empty() {
            return Err(SpirographError::NoArms);
        }
        if arms.iter().all(|arm| arm.speed == 0) {
            return Err(SpirographError::NoSpeed);
        }

        let mut wheels = Vec::new();
        for (i, pair) in arms.windows(2).enumerate() {
            let (arm, next) = (pair[0], pair[1]);
            let gear_speed = if i == 0 { 0 } else { arms[i - 1].speed };

            let turn = (arm.speed - gear_speed) as f32;
            let spin = (next.speed - gear_speed) as f32;
            if spin == 0.0 {
                // Arm numbers count from 1, and 0 is the ring.
                return Err(SpirographError::SameSpeed {
                    first: i,
                    second: i + 2,
                });
            }

            let length = arm.length.abs();
            wheels.push(Wheel {
                radius: (length * turn / spin).abs(),
                gear_radius: (length * (spin - turn) / spin).abs(),
            });
        }

        Ok(Spirograph { arms, wheels })
    }

    pub fn arms(&self) -> &[Arm] {
        &self.arms
    }

    /// One wheel for each arm but the last. Wheel `i` is centred at the end
    /// of arm `i`.
    pub fn wheels(&self) -> &[Wheel] {
        &self.wheels
    }

    /// The first wheel's gear. With only one arm there are no wheels, so the
    /// pen just goes round the ring.
    pub fn ring_radius(&self) -> f32 {
        match self.wheels.first() {
            Some(wheel) => wheel.gear_radius,
            None => self.arms[0].length.abs(),
        }
    }

    pub fn reach(&self) -> f32 {
        self.ring_radius()
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
        let angle = arm.angle(time);
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

/// The arms of a known classic spirograph, used by tests and the `_classic`
/// example.
#[allow(dead_code)]
pub fn classic_arms() -> Vec<Arm> {
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

pub fn fractal(
    length: f32,
    speed: i32,
    length_multiplier: f32,
    speed_multiplier: i32,
    iterations: u32,
) -> Result<Spirograph, SpirographError> {
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

    Spirograph::new(arms)
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

    // The gear train works out the same ring and wheel from these arms.
    Spirograph::new(vec![
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
    ])
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpirographError {
    WheelTooBig {
        wheel: u32,
        ring: u32,
    },
    WheelIsZero,
    NoArms,
    NoSpeed,
    /// Two arms with one arm between them turn at the same speed, so the
    /// wheel on the middle arm would have to be infinitely big. Arms count
    /// from 1, and arm 0 is the ring, which has speed 0.
    SameSpeed {
        first: usize,
        second: usize,
    },
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
            SpirographError::NoArms => write!(f, "a spirograph needs at least one arm"),
            SpirographError::NoSpeed => {
                write!(f, "at least one arm needs a speed other than 0")
            }
            SpirographError::SameSpeed { first, second } => {
                let middle = first + 1;
                if *first == 0 {
                    write!(f, "arm {second} can't have speed 0")?;
                } else {
                    write!(f, "arms {first} and {second} can't have the same speed")?;
                }
                write!(
                    f,
                    ": the wheel on arm {middle} would have to be infinitely big"
                )
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

    fn arms_from_speeds(speeds: &[i32]) -> Vec<Arm> {
        speeds
            .iter()
            .map(|&speed| Arm {
                length: 1.0,
                speed,
                ..Default::default()
            })
            .collect()
    }

    fn spiro_from_speeds(speeds: &[i32]) -> Spirograph {
        Spirograph::new(arms_from_speeds(speeds)).unwrap()
    }

    fn classic() -> Spirograph {
        Spirograph::new(classic_arms()).unwrap()
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
        let arms = [
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
        ];
        let point = pen_point(&arms, 0.0);
        assert_close!(point.x, 0.832164);
        assert_close!(point.y, 4.4108343);
    }

    #[test]
    fn test_pen_points_with_t() {
        let arms = [Arm {
            length: 1.0,
            speed: 1,
            phase: 0.0,
        }];
        let points = pen_points(&arms, TAU / 4.0);
        assert_eq!(points.len(), 1);
        assert_close!(points[0].x, 0.0);
        assert_close!(points[0].y, 1.0);
    }

    #[test]
    fn test_fractal_ring() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7).unwrap();
        assert_close!(spiro.ring_radius(), 75.0);
    }

    #[test]
    fn test_reach_classic() {
        let spiro = classic();
        assert_close!(spiro.reach(), 96.0);
    }

    #[test]
    fn test_reach_fractal() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7).unwrap();
        assert_close!(spiro.reach(), 90.0 * (1.0 - (1.0_f32 / 3.0).powi(7)));
    }

    #[test]
    fn test_wheels_classic() {
        // The classic arms are the 36 wheel in the 96 ring.
        let spiro = classic();
        assert_eq!(spiro.wheels().len(), 1);
        assert_close!(spiro.wheels()[0].radius, 36.0);
        assert_close!(spiro.wheels()[0].gear_radius, 96.0);
        assert_close!(spiro.ring_radius(), 96.0);
    }

    #[test]
    fn test_wheels_fractal() {
        let spiro = fractal(60.0, -3, 1.0 / 3.0, -4, 7).unwrap();
        let wheels = spiro.wheels();
        assert_eq!(wheels.len(), 6);
        assert_close!(wheels[0].radius, 15.0);
        assert_close!(wheels[0].gear_radius, 75.0);
        assert_close!(wheels[1].radius, 20.0 / 3.0);
        assert_close!(wheels[1].gear_radius, 80.0 / 3.0);
    }

    #[test]
    fn test_one_arm_goes_round_the_ring() {
        let spiro = Spirograph::new(vec![Arm {
            length: -10.0,
            speed: 1,
            ..Default::default()
        }])
        .unwrap();
        assert!(spiro.wheels().is_empty());
        assert_close!(spiro.ring_radius(), 10.0);
    }

    #[test]
    fn test_toy_ring_from_gears() {
        let spiro = toy(105, 52, 40.0).unwrap();
        assert_close!(spiro.ring_radius(), 105.0);
        assert_close!(spiro.wheels()[0].radius, 52.0);
    }

    #[test]
    fn test_spirograph_errors() {
        assert_eq!(
            Spirograph::new(vec![]).unwrap_err(),
            SpirographError::NoArms
        );
        assert_eq!(
            Spirograph::new(arms_from_speeds(&[0, 0])).unwrap_err(),
            SpirographError::NoSpeed
        );
        // Arm 2 has the ring's speed, 0.
        assert_eq!(
            Spirograph::new(arms_from_speeds(&[3, 0])).unwrap_err(),
            SpirographError::SameSpeed {
                first: 0,
                second: 2
            }
        );
        // A speed multiplier of -1 makes arms 1 and 3 match.
        assert_eq!(
            fractal(60.0, 2, 0.5, -1, 3).unwrap_err(),
            SpirographError::SameSpeed {
                first: 1,
                second: 3
            }
        );
        assert_eq!(
            fractal(60.0, 2, 0.5, -1, 3).unwrap_err().to_string(),
            "arms 1 and 3 can't have the same speed: the wheel on arm 2 would have to be infinitely big"
        );
    }

    /// The velocity of a point fixed to something turning at `spin` about
    /// `centre`, which itself moves at `centre_velocity`.
    fn velocity_of(point: Point, centre: Point, centre_velocity: Point, spin: f32) -> Point {
        Point {
            x: centre_velocity.x - spin * (point.y - centre.y),
            y: centre_velocity.y + spin * (point.x - centre.x),
        }
    }

    /// Where the end of each arm is going: the sum of each arm's velocity.
    fn end_velocities(arms: &[Arm], t: f32) -> Vec<Point> {
        let mut velocity = Point { x: 0.0, y: 0.0 };
        let mut velocities = Vec::new();
        for arm in arms {
            let angle = arm.angle(t);
            let speed = arm.speed as f32 * arm.length;
            velocity.x -= speed * angle.sin();
            velocity.y += speed * angle.cos();
            velocities.push(velocity);
        }
        velocities
    }

    #[test]
    fn test_wheels_roll_without_skidding() {
        // A wheel inside its ring, outside its gear, around a smaller gear,
        // and on a backwards arm.
        let spirographs = [
            toy(96, 36, 28.8).unwrap(),
            fractal(60.0, -3, 1.0 / 3.0, -4, 4).unwrap(),
            fractal(60.0, 1, 0.5, 2, 4).unwrap(),
            spiro_from_speeds(&[2, 1, 5]),
            Spirograph::new(vec![
                Arm {
                    length: -40.0,
                    speed: 3,
                    phase: 0.3,
                },
                Arm {
                    length: 10.0,
                    speed: -2,
                    phase: 1.0,
                },
            ])
            .unwrap(),
        ];

        for spiro in &spirographs {
            let arms = spiro.arms();
            for t in [0.0, 0.4, 1.7] {
                let ends = pen_points(arms, t);
                let end_velocities = end_velocities(arms, t);
                let origin = Point { x: 0.0, y: 0.0 };

                for (i, wheel) in spiro.wheels().iter().enumerate() {
                    let (gear_centre, gear_velocity, gear_spin) = if i == 0 {
                        (origin, origin, 0.0)
                    } else {
                        (ends[i - 1], end_velocities[i - 1], arms[i - 1].speed as f32)
                    };
                    let centre = ends[i];

                    // The touching point is on the line through both
                    // centres, `gear_radius` from the gear's centre: towards
                    // the wheel, or away from it when the gear is inside.
                    let distance = arms[i].length.abs();
                    let towards = (wheel.gear_radius - distance).abs() - wheel.radius;
                    let side = if towards.abs() < 1e-3 { 1.0 } else { -1.0 };
                    let scale = side * wheel.gear_radius / distance;
                    let touching = Point {
                        x: gear_centre.x + (centre.x - gear_centre.x) * scale,
                        y: gear_centre.y + (centre.y - gear_centre.y) * scale,
                    };

                    let on_gear = velocity_of(touching, gear_centre, gear_velocity, gear_spin);
                    let on_wheel = velocity_of(
                        touching,
                        centre,
                        end_velocities[i],
                        arms[i + 1].speed as f32,
                    );
                    let skid = (on_gear.x - on_wheel.x).abs() + (on_gear.y - on_wheel.y).abs();
                    assert!(
                        skid < 1e-2,
                        "{spiro:?} wheel {i} skids by {skid} at t = {t}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_toy_vs_classic_equivalence() {
        let toy_spiro = toy(96, 36, 28.8).unwrap();
        let classic_spiro = classic();

        assert_close!(toy_spiro.ring_radius(), classic_spiro.ring_radius());
        assert_eq!(toy_spiro.arms().len(), classic_spiro.arms().len());

        for (toy_arm, classic_arm) in toy_spiro.arms().iter().zip(classic_spiro.arms().iter()) {
            assert_close!(toy_arm.length, classic_arm.length);
            assert_eq!(toy_arm.speed, classic_arm.speed);
        }
    }
}
