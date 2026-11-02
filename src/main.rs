mod document;
mod error;
mod lexer;
mod parser;
mod render;
mod spirograph;

#[cfg(test)]
mod test_utils;

use document::Document;
use error::SpiroError;
use macroquad::prelude::*;
use render::{DisplayMode, draw_spirograph, scale_to_fit};
use spirograph::{Spirograph, max_reach};

const DEFAULT_FILE: &str = "examples/readme.spiro";

/// Reads, parses and evaluates a `.spiro` file.
fn load(path: &str) -> Result<(Document, Vec<Spirograph>), SpiroError> {
    let source = std::fs::read_to_string(path)?;
    let document = parser::parse_source(&source)?;
    let spirographs = document.spirographs()?;
    Ok((document, spirographs))
}

#[macroquad::main("spirograph")]
async fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_FILE.to_string());

    let (doc, spiros) = match load(&path) {
        Ok(loaded) => loaded,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(1);
        }
    };

    let reach = max_reach(&spiros);

    let mut time = 0.0;

    loop {
        time += get_frame_time();

        let scale = scale_to_fit(reach, screen_width(), screen_height());

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
