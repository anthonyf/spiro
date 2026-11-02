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
        let t = period * progress;
        let points = pen_points(arms, t);
        let mut prev_point = Point { x: 0.0, y: 0.0 };
        draw_circle_lines(prev_point.x, prev_point.y, *ring_radius, thickness, BLUE);

        let wheel_sizes = spiro.wheel_sizes();
        let wheel_spins = spiro.wheel_spins();

        for (((point, wheel_radius), spin), arm) in points
            .iter()
            .zip(wheel_sizes.iter())
            .zip(wheel_spins.iter())
            .zip(arms.iter())
        {
            // draw the wheel
            draw_circle_lines(point.x, point.y, *wheel_radius, thickness, RED);

            // draw a spoke, turning as the wheel rolls. It starts lined up
            // with its arm.
            let spoke_angle = arm.phase + spin * t;
            draw_line(
                point.x,
                point.y,
                point.x + wheel_radius * spoke_angle.cos(),
                point.y + wheel_radius * spoke_angle.sin(),
                thickness,
                RED,
            );

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
