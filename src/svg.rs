use crate::document::Document;
use crate::render::scale_to_fit;
use crate::spirograph::{Spirograph, max_reach, pen_point};
use macroquad::prelude::Color;
use std::fmt::Write;

/// Draws a document's finished curves as a square SVG image, `size` pixels
/// on each side. Only the curves are drawn, not the ring, wheels or arms.
///
/// The drawing is fitted with the same margin as the window, and y points
/// down, as it does on screen.
pub fn to_svg(doc: &Document, spirographs: &[Spirograph], size: f32) -> String {
    let scale = scale_to_fit(max_reach(spirographs), size, size);
    let centre = size / 2.0;

    // Writing to a String can't fail, so the results of write! are ignored.
    let mut svg = String::new();
    let _ = writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 {size} {size}">"#
    );
    let _ = writeln!(
        svg,
        r#"<rect width="100%" height="100%" fill="{}"/>"#,
        hex(doc.background_color)
    );

    for (spiro, layer) in spirographs.iter().zip(&doc.layers) {
        // The same step count as the window: a point about every 2 pixels.
        let steps = ((spiro.distance() * scale) as u32 / 2).max(1);
        let period = spiro.period();

        let _ = write!(
            svg,
            r#"<path fill="none" stroke="{}" stroke-width="{}" stroke-linecap="round" stroke-linejoin="round" d=""#,
            hex(layer.pen.color),
            layer.pen.thickness
        );
        for i in 0..=steps {
            let t = i as f32 / steps as f32 * period;
            let point = pen_point(spiro.arms(), t);
            let command = if i == 0 { 'M' } else { 'L' };
            let _ = write!(
                svg,
                "{command}{:.1} {:.1}",
                centre + point.x * scale,
                centre + point.y * scale
            );
        }
        svg.push_str("\"/>\n");
    }

    svg.push_str("</svg>\n");
    svg
}

/// A color as `#rrggbb`. Each channel is rounded, so a color made from
/// bytes (like `x008080`) comes back exactly.
fn hex(color: Color) -> String {
    let byte = |channel: f32| (channel * 255.0).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        byte(color.r),
        byte(color.g),
        byte(color.b)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_source;

    fn svg_for(source: &str, size: f32) -> String {
        let doc = parse_source(source).unwrap();
        let spirographs = doc.spirographs().unwrap();
        to_svg(&doc, &spirographs, size)
    }

    #[test]
    fn test_hex_color() {
        assert_eq!(hex(macroquad::prelude::BLACK), "#000000");
        assert_eq!(hex(macroquad::prelude::WHITE), "#ffffff");
        assert_eq!(hex(Color::from_rgba(0, 128, 128, 255)), "#008080");
    }

    #[test]
    fn test_svg_has_background_and_one_path_per_layer() {
        let svg = svg_for(
            "background black
             layer { pen red 2 toy ring 96 wheel 36 hole 28.8 }
             layer { pen x008080 3 toy ring 96 wheel 52 hole 40 }",
            400.0,
        );
        assert!(svg.starts_with("<svg "));
        assert!(svg.contains(r#"width="400" height="400""#));
        assert!(svg.contains(r##"<rect width="100%" height="100%" fill="#000000"/>"##));
        assert_eq!(svg.matches("<path ").count(), 2);
        assert!(svg.contains(r##"stroke="#e62938" stroke-width="2""##));
        assert!(svg.contains(r##"stroke="#008080" stroke-width="3""##));
        assert!(svg.trim_end().ends_with("</svg>"));
    }

    #[test]
    fn test_svg_circle_fits_the_image() {
        // One arm makes a circle. In a 200 px image the margin is 10 px, so
        // the circle starts on the right at x = 190, level with the centre.
        let svg = svg_for("layer { pen black 1 classic { arm 10 speed 1 } }", 200.0);
        assert!(svg.contains(r#"d="M190.0 100.0L"#), "{svg}");
    }
}
