use crate::spirograph::{Arm, Spirograph, SpirographError, classic_arms, fractal, toy};
use macroquad::prelude::{BLACK, Color, WHITE};

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub background_color: Color,
    pub layers: Vec<Layer>,
}

impl Document {
    pub fn spirographs(&self) -> Result<Vec<Spirograph>, SpirographError> {
        self.layers
            .iter()
            .map(|layer| layer.shape.to_spirograph())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub pen: Pen,
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pen {
    pub color: Color,
    pub thickness: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// The arms as written. The ring and wheels are worked out from them.
    Classic {
        arms: Vec<Arm>,
    },
    Fractal {
        length: f32,
        speed: i32,
        length_multiplier: f32,
        speed_multiplier: i32,
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
            Shape::Classic { arms } => Spirograph::new(arms.clone()),
            Shape::Fractal {
                length,
                speed,
                length_multiplier,
                speed_multiplier,
                iterations,
            } => fractal(
                *length,
                *speed,
                *length_multiplier,
                *speed_multiplier,
                *iterations,
            ),
            Shape::Toy { ring, wheel, hole } => toy(*ring, *wheel, *hole),
        }
    }
}

pub fn _toy(ring: u32, wheel: u32, hole: f32) -> Document {
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

pub fn _classic() -> Document {
    Document {
        background_color: WHITE,
        layers: vec![Layer {
            pen: Pen {
                color: BLACK,
                thickness: 3.0,
            },
            shape: Shape::Classic {
                arms: classic_arms(),
            },
        }],
    }
}

pub fn _fractal(
    start_length: f32,
    start_speed: i32,
    next_length_multiplier: f32,
    next_speed_multiplier: i32,
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
                length_multiplier: next_length_multiplier,
                speed_multiplier: next_speed_multiplier,
                iterations,
            },
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
