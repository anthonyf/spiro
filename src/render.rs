use crate::document::Pen;
use crate::spirograph::{Point, Spirograph, pen_point, pen_points};
use macroquad::prelude::{BLUE, GREEN, RED, draw_circle_lines, draw_line};

#[derive(Debug, Clone, Copy)]
pub enum DisplayMode {
    _Normal,
    Debug,
}

pub fn scale_to_fit(reach: f32, width: f32, height: f32) -> f32 {
    let room = width.min(height) / 2.0;
    let margin = room * 0.1;

    (room - margin) / reach
}

pub fn draw_spirograph(
    spiro: &Spirograph,
    pen: &Pen,
    display_mode: DisplayMode,
    time: f32,
    scale: f32,
) {
    let arms = spiro.arms();
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
        // draw the gear train, actual size
        let t = period * progress;
        let points = pen_points(arms, t);
        let origin = Point { x: 0.0, y: 0.0 };
        draw_circle_lines(origin.x, origin.y, spiro.ring_radius(), thickness, BLUE);

        // Wheel i is on the end of arm i, and rolls on a gear at the start
        // of the arm. The first gear is the ring, drawn above.
        for (i, wheel) in spiro.wheels().iter().enumerate() {
            let centre = points[i];
            if i > 0 {
                let gear_centre = points[i - 1];
                draw_circle_lines(
                    gear_centre.x,
                    gear_centre.y,
                    wheel.gear_radius,
                    thickness,
                    BLUE,
                );
            }
            draw_circle_lines(centre.x, centre.y, wheel.radius, thickness, RED);
        }

        // draw the arms. Each arm after the first is a spoke of the wheel at
        // its start, so it shows that wheel turning.
        let mut prev_point = origin;
        for point in &points {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_close;

    #[test]
    fn test_scale_to_fit() {
        let reach = 100.0;
        let width = 800.0;
        let height = 600.0;

        let scale = scale_to_fit(reach, width, height);
        assert_close!(scale, 2.7);
    }

    #[test]
    fn test_scale_to_fit_portrait() {
        let reach = 100.0;
        let width = 1000.0;
        let height = 1200.0;

        let scale = scale_to_fit(reach, width, height);
        assert_close!(scale, 4.5);
    }

    #[test]
    fn test_scale_to_fit_square() {
        let reach = 100.0;
        let width = 800.0;
        let height = 800.0;

        let scale = scale_to_fit(reach, width, height);
        assert_close!(scale, 3.6);
    }
}
